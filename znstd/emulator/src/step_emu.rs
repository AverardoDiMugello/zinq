use std::collections::HashMap;

use zinq::insn::semantics::{expr::*, stmt::*, *};
use zinq::insn::Instruction;
use zinq::system::{Processor, System};

pub struct StepEmu;

impl StepEmu {
    pub fn new() -> Self {
        Self
    }

    pub fn run<'sys, P>(&mut self, mut system: System<P>) -> System<P>
    where
        P: Processor,
    {
        loop {
            let cpu = system.proc();
            match cpu.fetch_decode(cpu.ip(), system.mem()) {
                Ok(insn) => {
                    println!("{0}", insn.disassemble());
                    let ir = insn.semanitcs(cpu);
                    if let Some(branch_dst) = self.emulate(ir) {
                        system.proc_mut().set_ip(branch_dst);
                    } else {
                        let next_insn = cpu.ip() + insn.size();
                        system.proc_mut().set_ip(next_insn);
                    }
                }
                Err(e) => {
                    println!("Fetch err: {e}");
                    break;
                }
            }
        }
        return system;
    }

    fn emulate(&mut self, block: IrBlock) -> Option<usize> {
        let mut tmp_ctx = TmpCtx::new();

        for (line_num, stmt) in block.into_iter().enumerate() {
            match stmt {
                Stmt::Assignment(expr) => self.emulate_assignment(expr, line_num, &mut tmp_ctx),
                Stmt::WriteProc(stmt) => self.emulate_write_proc(stmt, &tmp_ctx),
                Stmt::Branch(stmt) => match stmt {
                    Branch::Goto32(addr) => return Some(addr.eval(&tmp_ctx) as usize),
                    Branch::Goto64(addr) => return Some(addr.eval(&tmp_ctx) as usize),
                    Branch::IfElse {
                        cond,
                        true_case,
                        false_case,
                    } => {
                        if cond.eval(&tmp_ctx) {
                            return Some(true_case);
                        } else {
                            return Some(false_case);
                        }
                    }
                },
                Stmt::MemTx(tx) => match tx {
                    MemTx::Read32(addr) => panic!("32-bit memtx not supported yet"),
                    MemTx::Write32(addr, val) => panic!("32-bit memtx not supported yet"),
                    MemTx::Read64(addr) => panic!("64-bit memtx not supported yet"),
                    MemTx::Write64(addr, val) => panic!("64-bit memtx not supported yet"),
                },
                Stmt::Termination => panic!("Termination not supported!"),
            };
        }

        None
    }

    fn emulate_assignment(&mut self, expr: Expr, line_num: usize, emu_ctx: &mut TmpCtx) {
        match expr {
            // Typed expressions
            Expr::ExprBool(expr) => {
                emu_ctx.vals_bool.insert(line_num, expr.eval(emu_ctx));
            }
            Expr::Expr32(expr) => {
                emu_ctx.vals_32.insert(line_num, expr.eval(emu_ctx));
            }
            Expr::Expr64(expr) => {
                emu_ctx.vals_64.insert(line_num, expr.eval(emu_ctx));
            }
            Expr::Expr128(expr) => {
                emu_ctx.vals_128.insert(line_num, expr.eval(emu_ctx));
            }
            // Conversion expressions
            Expr::ToBool(expr) => {
                emu_ctx.vals_bool.insert(
                    line_num,
                    match expr {
                        ToBool::Cmp32(expr) => expr.eval(emu_ctx),
                        ToBool::Cmp64(expr) => expr.eval(emu_ctx),
                        ToBool::Cmp128(expr) => expr.eval(emu_ctx),
                    },
                );
            }
            Expr::To32(expr) => {
                emu_ctx.vals_32.insert(
                    line_num,
                    match expr {
                        To32::From64(expr) => expr.eval(emu_ctx),
                        To32::From128(expr) => expr.eval(emu_ctx),
                    },
                );
            }
            Expr::To64(expr) => {
                emu_ctx.vals_64.insert(
                    line_num,
                    match expr {
                        To64::From32(expr) => expr.eval(emu_ctx),
                        To64::From128(expr) => expr.eval(emu_ctx),
                    },
                );
            }
            Expr::To128(expr) => {
                emu_ctx.vals_128.insert(
                    line_num,
                    match expr {
                        To128::From32(expr) => expr.eval(emu_ctx),
                        To128::From64(expr) => expr.eval(emu_ctx),
                    },
                );
            }
        }
    }

    fn emulate_write_proc(&mut self, stmt: WriteProc, emu_ctx: &TmpCtx) {
        stmt.execute(emu_ctx);
    }
}

struct TmpCtx {
    vals_bool: HashMap<usize, bool>,
    vals_32: HashMap<usize, u32>,
    vals_64: HashMap<usize, u64>,
    vals_128: HashMap<usize, u128>,
}

impl TmpCtx {
    fn new() -> Self {
        Self {
            vals_bool: HashMap::new(),
            vals_32: HashMap::new(),
            vals_64: HashMap::new(),
            vals_128: HashMap::new(),
        }
    }
}

impl EvalCtx<bool> for TmpCtx {
    fn look_up(&self, v: Var<bool>) -> bool {
        self.vals_bool
            .get(&v.id())
            .expect(&format!("No Var<bool> exists with ID {v:?}"))
            .clone()
    }
}

impl EvalCtx<u32> for TmpCtx {
    fn look_up(&self, v: Var<u32>) -> u32 {
        self.vals_32
            .get(&v.id())
            .expect(&format!("No Var<u32> exists with ID {v:?}"))
            .clone()
    }
}

impl EvalCtx<u64> for TmpCtx {
    fn look_up(&self, v: Var<u64>) -> u64 {
        self.vals_64
            .get(&v.id())
            .expect(&format!("No Var<u64> exists with ID {v:?}"))
            .clone()
    }
}

impl EvalCtx<u128> for TmpCtx {
    fn look_up(&self, v: Var<u128>) -> u128 {
        self.vals_128
            .get(&v.id())
            .expect(&format!("No Var<u128> exists with ID {v:?}"))
            .clone()
    }
}
