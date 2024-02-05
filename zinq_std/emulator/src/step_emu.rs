use std::collections::HashMap;

use zinq::insn::semantics::{expr::*, stmt::*, *};
use zinq::insn::Instruction;
use zinq::system::{Processor, System};
use zinq::{Emulator, Result};

pub struct StepEmu {
    max_insns: Option<usize>,
    bb_cb: Option<fn() -> ()>,
}

impl StepEmu {
    pub fn new() -> Self {
        Self {
            max_insns: None,
            bb_cb: None,
        }
    }

    pub fn max_insns(&mut self, icount: usize) {
        self.max_insns = Some(icount);
    }

    pub fn no_max_insns(&mut self) {
        self.max_insns = None;
    }
}

impl<P: Processor> Emulator<System<P>> for StepEmu {
    fn run(&mut self, system: &mut System<P>)
    where
        P: Processor,
    {
        let mut icount = 0;
        loop {
            if let Some(max_insns) = self.max_insns {
                if icount >= max_insns {
                    break;
                }
            }

            match system.proc().fetch_decode(system.proc().ip(), system.mem()) {
                Ok(insn) => {
                    icount += 1;

                    println!("{0}", insn.disassemble());
                    println!("{0:?}", insn);
                    let code = insn.semantics(system.proc());

                    let mut exec_ctx = TmpCtx::new();
                    for (line_num, stmt) in code.into_iter().enumerate() {
                        match stmt {
                            Stmt::Assignment(expr) => assignment(expr, line_num, &mut exec_ctx),
                            Stmt::ProcWrite(stmt) => proc_write(stmt, &exec_ctx),
                            Stmt::Branch(stmt) => {
                                match stmt {
                                    Branch::Uncond(addr) => {
                                        if let Some(bb_cb) = self.bb_cb {
                                            bb_cb();
                                        }

                                        system.proc_mut().set_ip(addr.eval(&exec_ctx))
                                    }
                                    Branch::Cond {
                                        cond,
                                        true_case,
                                        false_case,
                                    } => {
                                        let next_addr = if cond.eval(&exec_ctx) {
                                            true_case.eval(&exec_ctx)
                                        } else {
                                            false_case.eval(&exec_ctx)
                                        };

                                        if let Some(bb_cb) = self.bb_cb {
                                            bb_cb();
                                        }

                                        system.proc_mut().set_ip(next_addr);
                                    }
                                }
                                // TODO: I know that branches are the last stmt in an IrBlock, but this isn't actually defined anywhere.
                                // As such, if this break isn't put here to explicitly inform the compiler that the loop ends, then the
                                // compiler throws an error about borring the processor at mutable to change the IP. Re-examine this.
                                break;
                            }
                            Stmt::MemTx(tx) => match tx {
                                MemTx::Read32(addr) => panic!("32-bit memtx not supported yet"),
                                MemTx::Write32(addr, val) => {
                                    panic!("32-bit memtx not supported yet")
                                }
                                MemTx::Read64(addr) => panic!("64-bit memtx not supported yet"),
                                MemTx::Write64(addr, val) => {
                                    panic!("64-bit memtx not supported yet")
                                }
                            },
                        };
                    }
                }
                Err(e) => {
                    println!("Fetch err: {e}");
                    return;
                }
            }
        }
    }
}

/*
 * Helpers
 */

struct TmpCtx {
    vals_bool: HashMap<usize, bool>,
    vals_32: HashMap<usize, u32>,
    vals_64: HashMap<usize, u64>,
    vals_128: HashMap<usize, u128>,
    vals_addr: HashMap<usize, usize>,
}

impl TmpCtx {
    pub fn new() -> Self {
        Self {
            vals_bool: HashMap::new(),
            vals_32: HashMap::new(),
            vals_64: HashMap::new(),
            vals_128: HashMap::new(),
            vals_addr: HashMap::new(),
        }
    }
}

impl<'p> ExecCtx<bool> for TmpCtx {
    fn look_up(&self, v: Var<bool>) -> bool {
        self.vals_bool
            .get(&v.id())
            .expect(&format!("No Var<bool> exists with ID {v:?}"))
            .clone()
    }
}

impl<'p> ExecCtx<u32> for TmpCtx {
    fn look_up(&self, v: Var<u32>) -> u32 {
        self.vals_32
            .get(&v.id())
            .expect(&format!("No Var<u32> exists with ID {v:?}"))
            .clone()
    }
}

impl<'p> ExecCtx<u64> for TmpCtx {
    fn look_up(&self, v: Var<u64>) -> u64 {
        self.vals_64
            .get(&v.id())
            .expect(&format!("No Var<u64> exists with ID {v:?}"))
            .clone()
    }
}

impl<'p> ExecCtx<u128> for TmpCtx {
    fn look_up(&self, v: Var<u128>) -> u128 {
        self.vals_128
            .get(&v.id())
            .expect(&format!("No Var<u128> exists with ID {v:?}"))
            .clone()
    }
}

impl<'p> ExecCtx<usize> for TmpCtx {
    fn look_up(&self, v: Var<usize>) -> usize {
        self.vals_addr
            .get(&v.id())
            .expect(&format!("No Var<usize> exists with ID {v:?}"))
            .clone()
    }
}

fn assignment(expr: Expr, line_num: usize, exec_ctx: &mut TmpCtx) {
    match expr {
        // Typed expressions
        Expr::ExprBool(expr) => {
            let eval = expr.eval(exec_ctx);
            println!("var{line_num} = {eval}");
            exec_ctx.vals_bool.insert(line_num, eval);
        }
        Expr::Expr32(expr) => {
            let eval = expr.eval(exec_ctx);
            println!("var{line_num} = 0x{eval:X}");
            exec_ctx.vals_32.insert(line_num, eval);
        }
        Expr::Expr64(expr) => {
            let eval = expr.eval(exec_ctx);
            println!("var{line_num} = 0x{eval:X}");
            exec_ctx.vals_64.insert(line_num, eval);
        }
        Expr::Expr128(expr) => {
            let eval = expr.eval(exec_ctx);
            println!("var{line_num} = 0x{eval:X}");
            exec_ctx.vals_128.insert(line_num, eval);
        }
        // Conversion expressions
        Expr::ToBool(expr) => {
            exec_ctx.vals_bool.insert(
                line_num,
                match expr {
                    ToBool::Cmp32(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = {eval}");
                        eval
                    }
                    ToBool::Cmp64(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = {eval}");
                        eval
                    }
                    ToBool::Cmp128(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = {eval}");
                        eval
                    }
                },
            );
        }
        Expr::To32(expr) => {
            exec_ctx.vals_32.insert(
                line_num,
                match expr {
                    To32::From64(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                    To32::From128(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                },
            );
        }
        Expr::To64(expr) => {
            exec_ctx.vals_64.insert(
                line_num,
                match expr {
                    To64::From32(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                    To64::From128(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                },
            );
        }
        Expr::To128(expr) => {
            exec_ctx.vals_128.insert(
                line_num,
                match expr {
                    To128::From32(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                    To128::From64(expr) => {
                        let eval = expr.eval(exec_ctx);
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                },
            );
        }
        Expr::ToAddr(expr) => {
            exec_ctx.vals_addr.insert(
                line_num,
                match expr {
                    ToAddr::From32(expr) => {
                        let eval = expr.eval(exec_ctx) as usize;
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                    ToAddr::From64(expr) => {
                        let eval = expr.eval(exec_ctx) as usize;
                        println!("var{line_num} = 0x{eval:X}");
                        eval
                    }
                },
            );
        }
    }
}

fn proc_write(stmt: ProcWrite, exec_ctx: &TmpCtx) {
    stmt.execute(exec_ctx);
}
