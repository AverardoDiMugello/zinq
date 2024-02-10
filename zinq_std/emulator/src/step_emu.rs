use std::collections::HashMap;

use bitvec::prelude::*;

use zinq::insn::semantics::*;
use zinq::insn::Instruction;
use zinq::system::{Processor, System};
use zinq::Emulator;

pub struct StepEmu {
    max_insns: Option<usize>,
}

impl StepEmu {
    pub fn new() -> Self {
        Self { max_insns: None }
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

            if let Ok(insn) = system.proc().fetch_decode(system.proc().ip(), system.mem()) {
                icount += 1;

                println!("{0}", insn.disassemble());
                println!("{0:?}", insn);
                let code = insn.semantics(system.proc());

                // Using HashMap for now
                let mut exec_ctx = HashMap::new();
                for (lineno, stmt) in code.into_iter().enumerate() {
                    println!("{lineno}: {stmt:?}");
                    match stmt {
                        Stmt::Assignment(expr) => {
                            let val = expr
                                .eval(&exec_ctx)
                                .expect("Expression could not be evaluated");
                            println!("val = {val:?}");
                            exec_ctx.insert(lineno, val);
                        }
                        Stmt::WriteProc { var, start, val } => {
                            let var = var as *const BitSlice;
                            let var = var.cast_mut();
                            let val = val
                                .eval(&exec_ctx)
                                .expect("WriteProc terminal could not be evaluated");

                            unsafe {
                                let var = (*var)
                                    .get_mut(start..val.len())
                                    .expect("Tried to write outside the bounds of a Processor context BitVec!");
                                var.copy_from_bitslice(val);
                            }
                        }
                        Stmt::Goto(addr) => {
                            let addr = addr
                                .eval(&exec_ctx)
                                .expect("Goto terminal could not be evaluated");
                            system.proc_mut().set_ip(addr.load());
                            // I know that branches are the end of IrBlocks but the compiler doesn't.
                            // Until I fix that, we need this explicit break after taking the proc as
                            // mutable.
                            break;
                        }
                        Stmt::IfElse {
                            cond,
                            true_case,
                            false_case,
                        } => {
                            let cond = cond
                                .eval(&exec_ctx)
                                .expect("If-else terminal could not be evaluated");
                            let true_case = true_case
                                .eval(&exec_ctx)
                                .expect("If-else terminal could not be evaluated");
                            let false_case = false_case
                                .eval(&exec_ctx)
                                .expect("If-else terminal could not be evaluated");

                            let next_addr = if cond.any() { true_case } else { false_case };

                            system.proc_mut().set_ip(next_addr.load());
                            // I know that branches are the end of IrBlocks but the compiler doesn't.
                            // Until I fix that, we need this explicit break after taking the proc as
                            // mutable.
                            break;
                        }
                        _ => panic!("Statement not supported!"),
                    };
                }
            } else {
                panic!("Fetch err!");
            }
        }
    }
}
