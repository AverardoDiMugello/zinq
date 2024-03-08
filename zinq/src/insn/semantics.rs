use std::{cmp::max, collections::HashMap};

use bitvec::prelude::*;

/// A variables is an index that points to an Assignment statement in an IrContext that
/// defines a BitVec
pub type Var = usize;

/// Terminals are either a BitVec literal or a variable. All Expressions evalutate to
/// terminals
#[derive(Debug, Clone)]
pub enum Term {
    Lit(BitVec),
    Var(Var),
}

impl Term {
    pub fn eval<'a>(&'a self, exec_ctx: &'a HashMap<usize, BitVec>) -> Option<&'a BitSlice> {
        match self {
            Term::Lit(val) => Some(val.as_bitslice()),
            Term::Var(var) => exec_ctx.get(var).and_then(|val| Some(val.as_bitslice())),
        }
    }
}

#[derive(Debug)]
pub enum Expr<'p> {
    Term(Term),
    // Unary
    Not(Term),
    Lsb(Term),
    Msb(Term),
    IsZero(Term),
    // Logic
    And(Term, Term),
    // Arithmetic
    Add(Term, Term),
    // Compare
    Eq(Term, Term),
    Neq(Term, Term),
    // Vector
    Sext { val: Term, size: Term },
    Zext { val: Term, size: Term },
    // Context access
    ReadProc(&'p BitSlice, usize),
}

// /// Expressions can manipulate one or more BitVec terminals to produce a new BitVec or read
// /// a BitVec from a Processor context. Expressions by themselves do not alter an execution
// /// context. Memory reads are not considered expressions, because they can have side effects,
// /// e.g. in MMIO.
// /// Expressions with a bool operand use the additional flag to determine if the two BitVec
// /// operands should be sign extended (if true) to the same length.
// #[derive(Debug)]
// pub enum Expr<'p> {
//     Term(Term),
//     Unary(UnaOp, Term),
//     Binary(BinOp, Term, Term, bool),
//     // Bit vector
//     Lsl {
//         val: Term,
//         amt: usize,
//     },
//     Lsr {
//         val: Term,
//         amt: usize,
//     },
//     Asr {
//         val: Term,
//         amt: usize,
//     },
//     Ror {
//         val: Term,
//         amt: usize,
//     },
//     Zext {
//         val: Term,
//         size: usize,
//     },
//     Bit {
//         val: Term,
//         idx: usize,
//     },
//     Slice {
//         val: Term,
//         start: usize,
//         len: usize,
//     },
//     Merge {
//         lhs: Term,
//         start_lhs: usize,
//         rhs: Term,
//         start_rhs: usize,
//         len: usize,
//     },
//     // Context access
//     ReadProc {
//         var: &'p BitSlice,
//         size: usize,
//     },
// }

// impl<'p> Expr<'p> {
//     /// Evaluate this expression, returning a new, concrete BitVec
//     pub fn eval(&self, exec_ctx: &HashMap<usize, BitVec>) -> Option<BitVec> {
//         match self {
//             Expr::Term(t) => t.eval(exec_ctx).and_then(|t| Some(t.to_bitvec())),
//             Expr::Unary(op, val) => match op {
//                 UnaOp::Not => val.eval(exec_ctx).and_then(|val| Some(!val.to_bitvec())),
//                 UnaOp::IsPos | UnaOp::IsNeg | UnaOp::IsZero | UnaOp::IsNonZero => val
//                     .eval(exec_ctx)
//                     .and_then(|val| match op {
//                         UnaOp::IsPos => Some(!val.last()?),
//                         UnaOp::IsNeg => Some(*val.last()?),
//                         UnaOp::IsZero => Some(!val.any()),
//                         UnaOp::IsNonZero => Some(val.any()),
//                         _ => panic!("Unreachable"),
//                     })
//                     .and_then(|result| {
//                         if result {
//                             Some(bitvec!(1))
//                         } else {
//                             Some(bitvec!(0))
//                         }
//                     }),
//             },
//             Expr::Binary(op, lhs, rhs, sext) => {
//                 // Sign or zero extend the operands to the same length according to the flag
//                 let mut lhs = lhs.eval(exec_ctx)?.to_bitvec();
//                 let mut rhs = rhs.eval(exec_ctx)?.to_bitvec();
//                 let new_len = max(lhs.len(), rhs.len());
//                 if *sext {
//                     let sign = *lhs.last()?;
//                     lhs.resize(new_len, sign);
//                     let sign = *rhs.last()?;
//                     rhs.resize(new_len, sign);
//                 } else {
//                     lhs.resize(new_len, false);
//                     rhs.resize(new_len, false);
//                 }

//                 match op {
//                     BinOp::Eq | BinOp::Neq | BinOp::Gt | BinOp::Gte | BinOp::Lt | BinOp::Lte => {
//                         let result = match op {
//                             BinOp::Eq => lhs == rhs,
//                             BinOp::Neq => lhs != rhs,
//                             BinOp::Gt => lhs > rhs,
//                             BinOp::Gte => lhs >= rhs,
//                             BinOp::Lt => lhs < rhs,
//                             BinOp::Lte => lhs <= rhs,
//                             _ => panic!("Unreachable"),
//                         };
//                         if result {
//                             Some(bitvec!(1))
//                         } else {
//                             Some(bitvec!(0))
//                         }
//                     }
//                     BinOp::And => Some(lhs & rhs),
//                     BinOp::Or => Some(lhs | rhs),
//                     BinOp::Xor => Some(lhs ^ rhs),
//                     BinOp::Add => {
//                         // Literally an emulated Full Adder circuit. There must be a better way...
//                         let mut carry = false;
//                         let mut sum = lhs
//                             .iter()
//                             .zip(rhs.iter())
//                             .map(|(a, b)| {
//                                 let (a, b) = (*a, *b);
//                                 let a_x_b = a ^ b;
//                                 let sum = a_x_b ^ carry;
//                                 let c_and_a_x_b = carry & a_x_b;
//                                 let a_and_b = a & b;
//                                 carry = c_and_a_x_b | a_and_b;

//                                 sum
//                             })
//                             .collect::<BitVec>();
//                         if carry {
//                             sum.push(true);
//                         }
//                         Some(sum)
//                     }
//                 }
//             }
//             Expr::Bit { val, idx } => Some(val.eval(exec_ctx)?.get(*idx..*idx + 1)?.to_bitvec()),
//             Expr::Slice { val, start, len } => {
//                 Some(val.eval(exec_ctx)?.get(*start..*start + *len)?.to_bitvec())
//             }
//             Expr::Merge {
//                 lhs,
//                 start_lhs,
//                 rhs,
//                 start_rhs,
//                 len,
//             } => {
//                 // lhs[start_lhs..start_lhs+len] = rhs[start_rhs.._start_rhs_len]
//                 let len = *len;
//                 let (start_lhs, end_lhs) = (*start_lhs, *start_lhs + len);
//                 let (start_rhs, end_rhs) = (*start_rhs, *start_rhs + len);

//                 let lhs = lhs.eval(exec_ctx)?;
//                 let rhs = rhs.eval(exec_ctx)?.get(start_rhs..end_rhs)?;

//                 let merged = lhs
//                     .iter()
//                     .enumerate()
//                     .map_while(|(idx, bit)| {
//                         if idx < start_lhs {
//                             Some(bit)
//                         } else if start_lhs <= idx && idx < end_lhs {
//                             rhs.get(start_rhs + (idx - start_lhs))
//                         } else {
//                             Some(bit)
//                         }
//                     })
//                     .collect();
//                 Some(merged)
//             }
//             Expr::Lsl { val, amt } => {
//                 let mut val = val.eval(exec_ctx)?.to_bitvec();
//                 for i in 0..*amt {
//                     val.insert(0, false);
//                 }
//                 Some(val)
//             }
//             Expr::Lsr { val, amt } => {
//                 let mut val = val.eval(exec_ctx)?.to_bitvec();
//                 for i in 0..*amt {
//                     val.remove(0);
//                     val.push(false);
//                 }
//                 Some(val)
//             }
//             Expr::Asr { val, amt } => {
//                 let mut val = val.eval(exec_ctx)?.to_bitvec();
//                 for i in 0..*amt {
//                     val.remove(0);
//                     val.push(true);
//                 }
//                 Some(val)
//             }
//             Expr::Ror { val, amt } => panic!("Expression not supported"),
//             Expr::Zext { val, size } => {
//                 let mut val = val.eval(exec_ctx)?.to_bitvec();
//                 val.resize(*size, false);
//                 Some(val)
//             }
//             Expr::ReadProc(var) => Some((*var).to_bitvec()),
//         }
//     }
// }

/// Statements alter one or more of the execution contexts (Processor(s), Memory, and/or Emulation)
#[derive(Debug)]
pub enum Stmt<'p> {
    Assignment(Expr<'p>),
    WriteProc {
        var: &'p BitSlice,
        val: Term,
    },
    Goto(Term),
    IfElse {
        cond: Term,
        true_case: Term,
        false_case: Term,
    },
}

#[derive(Debug)]
pub struct IrBlock<'p> {
    stmts: Vec<Stmt<'p>>,
}

impl<'p> IntoIterator for IrBlock<'p> {
    type Item = Stmt<'p>;
    type IntoIter = std::vec::IntoIter<Stmt<'p>>;

    fn into_iter(self) -> Self::IntoIter {
        self.stmts.into_iter()
    }
}

impl<'p> IrBlock<'p> {
    /// Create a new, empty IrBlock
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    /// Add an assign statement and return the variable that is defined by this assignment
    pub fn assign(&mut self, expr: Expr<'p>) -> Var {
        self.stmts.push(Stmt::Assignment(expr));
        return self.stmts.len() - 1;
    }

    /// Add a processor write statement
    pub fn write_proc(&mut self, var: &'p BitSlice, val: Term) {
        self.stmts.push(Stmt::WriteProc { var, val });
    }

    /// Add an unconditional branch statement
    pub fn goto(&mut self, addr: Term) {
        self.stmts.push(Stmt::Goto(addr));
    }

    /// Add a conditional branch statement
    pub fn if_else(&mut self, cond: Term, true_case: Term, false_case: Term) {
        self.stmts.push(Stmt::IfElse {
            cond,
            true_case,
            false_case,
        });
    }
}
