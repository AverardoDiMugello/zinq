use std::marker::PhantomData;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub};

/// A 1-bit datum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct I1(bool);

/// A 32-bit datum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct I32(u32);

/// A 64-bit datum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct I64(u64);

/// A 128-bit datum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct I128(u128);

/// All statements and expressions operate on terminals, which are typed literal values or variables.
/// Statements and expressions do not recur on themselves, i.e. no (a + b) * (c + d)
#[derive(Debug)]
pub enum Term<T: Copy> {
    Lit(T),
    Var(Var<T>),
}

/// A variable contains the line number of the IrContext for which this variable is defined
#[derive(Debug, Clone, Copy)]
pub struct Var<T: Copy>(usize, PhantomData<T>);

impl<T: Copy> Var<T> {
    pub fn new(def: usize) -> Self {
        Self(def, PhantomData)
    }
}

/// Statements alter the execution contexts (Processor(s), Memory, and Emulation)
#[derive(Debug)]
pub enum Stmt<'p> {
    Assignment(Expr<'p>),
    Branch(Branch),
    WriteProc(WriteProc<'p>),
}

/// Branch statements alter control-flow
#[derive(Debug)]
pub enum Branch {
    Cond {
        cond: Term<I1>,
        t_case: Term<usize>,
        f_case: Term<usize>,
    },
    Uncond(Term<usize>),
}

/// WriteProc statements alter the processor context
#[derive(Debug)]
pub enum WriteProc<'p> {
    WriteI1 { ctx_var: &'p bool, val: Term<I1> },
    WriteI32 { ctx_var: &'p u32, val: Term<I32> },
    WriteI64 { ctx_var: &'p u64, val: Term<I64> },
    WriteI128 { ctx_var: &'p u128, val: Term<I128> },
}

/// Expressions are evaluated in assignment statements. They are organized based on types
/// Expr[T] are "typed" expressions that operate on and produce data of type T
/// To[T] are "conversion" expressions that operate on one type and produce data of another type T
#[derive(Debug)]
pub enum Expr<'p> {
    // Typed expressions
    I1(ExprI1<'p>),
    I32(ExprI32<'p>),
    I64(ExprI64<'p>),
    I128(ExprI128<'p>),
    // Conversion expressions
    ToI1(ToI1),
    ToI32(ToI32),
    ToI64(ToI64),
    ToI128(ToI128),
}

/// Expressions that operate on and return 1-bit data
#[derive(Debug)]
pub enum ExprI1<'p> {
    Term(Term<I1>),
    Logic(Logic<I1>),
    ReadProc(ReadProc<'p, I1>),
}

/// Expressions that operate on and return 32-bit data
#[derive(Debug)]
pub enum ExprI32<'p> {
    Term(Term<I32>),
    Logic(Logic<I32>),
    Arith(Arith<I32>),
    Bitwise(Bitwise<I32>),
    ReadProc(ReadProc<'p, u32>),
}

/// Expressions that operate on and return 64-bit data
#[derive(Debug)]
pub enum ExprI64<'p> {
    Term(Term<I64>),
    Logic(Logic<I64>),
    Arith(Arith<I64>),
    Bitwise(Bitwise<I64>),
    ReadProc(ReadProc<'p, I64>),
}

/// Expressions that operate on and return 128-bit data
#[derive(Debug)]
pub enum ExprI128<'p> {
    Term(Term<I128>),
    Logic(Logic<I128>),
    Arith(Arith<I128>),
    Bitwise(Bitwise<I128>),
    ReadProc(ReadProc<'p, I128>),
}

/// Logical expressions
#[derive(Debug)]
pub enum Logic<T>
where
    T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Not<Output = T> + Copy,
{
    And { a: Term<T>, b: Term<T> },
    Not(Term<T>),
    Or { a: Term<T>, b: Term<T> },
    Xor { a: Term<T>, b: Term<T> },
}

/// Arithmetic expressions
#[derive(Debug)]
pub enum Arith<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    Add { a: Term<T>, b: Term<T> },
    Sub { a: Term<T>, b: Term<T> },
}

/// Bitwise expressions
#[derive(Debug)]
pub enum Bitwise<T>
where
    T: Shl<Output = T> + Shr<Output = T> + Copy,
{
    ShiftL { val: Term<T>, amt: Term<T> },
    ShiftR { val: Term<T>, amt: Term<T> },
}

/// Context read expression
#[derive(Debug)]
pub struct ReadProc<'p, T: Copy>(&'p T);

/// Comparison expressions
#[derive(Debug)]
pub enum Cmp<T>
where
    T: PartialOrd + PartialEq + Copy,
{
    Eq { lhs: Term<T>, rhs: Term<T> },
    Gt { lhs: Term<T>, rhs: Term<T> },
    Gte { lhs: Term<T>, rhs: Term<T> },
    Lt { lhs: Term<T>, rhs: Term<T> },
    Lte { lhs: Term<T>, rhs: Term<T> },
    Neq { lhs: Term<T>, rhs: Term<T> },
}

/// Expressions for converting into 1-bit data
#[derive(Debug)]
pub enum ToI1 {
    CmpI32(Cmp<I32>),
    CmpI64(Cmp<I64>),
    CmpI128(Cmp<I128>),
    MsbI32(Term<I32>),
    MsbI64(Term<I64>),
    MsbI128(Term<I128>),
}

/// Expressions for converting into 32-bit data
#[derive(Debug)]
pub enum ToI32 {
    FromI64(TruncConv<I64, I32>),
    FromI128(TruncConv<I128, I32>),
}

/// Expressions for converting into 64-bit data
#[derive(Debug)]
pub enum ToI64 {
    FromI32(ExtConv<I32, I64>),
    FromI128(TruncConv<I128, I64>),
}

/// Expressions for converting into 128-bit data
#[derive(Debug)]
pub enum ToI128 {
    FromI32(ExtConv<I32, I128>),
    FromI64(ExtConv<I64, I128>),
}

/// Expressions that convert a terminals from one type to another by extension
#[derive(Debug)]
pub enum ExtConv<I, O>
where
    I: Copy, /*Unsigned +*/
    O: Copy, /*Unsigned +*/
{
    Sign(Term<I>, PhantomData<O>),
    Zero(Term<I>, PhantomData<O>),
}

/// Expressions that convert a terminals from one type to another by truncation
#[derive(Debug)]
pub enum TruncConv<I, O>
where
    I: Copy, /*Unsigned +*/
    O: Copy, /*Unsigned +*/
{
    Sign(Term<I>, PhantomData<O>),
    Zero(Term<I>, PhantomData<O>),
}

/*
 * Trait implementations on the primitive newtypes to satisfy the above expressions
 */

// I1
// Logic
impl BitAnd for I1 {
    type Output = I1;
    fn bitand(self, rhs: Self) -> Self::Output {
        I1(self.0 & rhs.0)
    }
}

impl BitOr for I1 {
    type Output = I1;
    fn bitor(self, rhs: Self) -> Self::Output {
        I1(self.0 | rhs.0)
    }
}

impl BitXor for I1 {
    type Output = I1;
    fn bitxor(self, rhs: Self) -> Self::Output {
        I1(self.0 ^ rhs.0)
    }
}

impl Not for I1 {
    type Output = I1;
    fn not(self) -> Self::Output {
        I1(!self.0)
    }
}

// I32
// Logic
impl BitAnd for I32 {
    type Output = I32;
    fn bitand(self, rhs: Self) -> Self::Output {
        I32(self.0 & rhs.0)
    }
}

impl BitOr for I32 {
    type Output = I32;
    fn bitor(self, rhs: Self) -> Self::Output {
        I32(self.0 | rhs.0)
    }
}

impl BitXor for I32 {
    type Output = I32;
    fn bitxor(self, rhs: Self) -> Self::Output {
        I32(self.0 ^ rhs.0)
    }
}

impl Not for I32 {
    type Output = I32;
    fn not(self) -> Self::Output {
        I32(!self.0)
    }
}

// Arith
impl Add for I32 {
    type Output = I32;
    fn add(self, rhs: Self) -> Self::Output {
        I32(self.0 + rhs.0)
    }
}

impl Sub for I32 {
    type Output = I32;
    fn sub(self, rhs: Self) -> Self::Output {
        I32(self.0 - rhs.0)
    }
}

// Bitwise
impl Shl for I32 {
    type Output = I32;
    fn shl(self, rhs: Self) -> Self::Output {
        I32(self.0 << rhs.0)
    }
}

impl Shr for I32 {
    type Output = I32;
    fn shr(self, rhs: Self) -> Self::Output {
        I32(self.0 >> rhs.0)
    }
}

// I64
// Logic
impl BitAnd for I64 {
    type Output = I64;
    fn bitand(self, rhs: Self) -> Self::Output {
        I64(self.0 & rhs.0)
    }
}

impl BitOr for I64 {
    type Output = I64;
    fn bitor(self, rhs: Self) -> Self::Output {
        I64(self.0 | rhs.0)
    }
}

impl BitXor for I64 {
    type Output = I64;
    fn bitxor(self, rhs: Self) -> Self::Output {
        I64(self.0 ^ rhs.0)
    }
}

impl Not for I64 {
    type Output = I64;
    fn not(self) -> Self::Output {
        I64(!self.0)
    }
}

// Arith
impl Add for I64 {
    type Output = I64;
    fn add(self, rhs: Self) -> Self::Output {
        I64(self.0 + rhs.0)
    }
}

impl Sub for I64 {
    type Output = I64;
    fn sub(self, rhs: Self) -> Self::Output {
        I64(self.0 - rhs.0)
    }
}

// Bitwise
impl Shl for I64 {
    type Output = I64;
    fn shl(self, rhs: Self) -> Self::Output {
        I64(self.0 << rhs.0)
    }
}

impl Shr for I64 {
    type Output = I64;
    fn shr(self, rhs: Self) -> Self::Output {
        I64(self.0 >> rhs.0)
    }
}

// I128
// Logic
impl BitAnd for I128 {
    type Output = I128;
    fn bitand(self, rhs: Self) -> Self::Output {
        I128(self.0 & rhs.0)
    }
}

impl BitOr for I128 {
    type Output = I128;
    fn bitor(self, rhs: Self) -> Self::Output {
        I128(self.0 | rhs.0)
    }
}

impl BitXor for I128 {
    type Output = I128;
    fn bitxor(self, rhs: Self) -> Self::Output {
        I128(self.0 ^ rhs.0)
    }
}

impl Not for I128 {
    type Output = I128;
    fn not(self) -> Self::Output {
        I128(!self.0)
    }
}

// Arith
impl Add for I128 {
    type Output = I128;
    fn add(self, rhs: Self) -> Self::Output {
        I128(self.0 + rhs.0)
    }
}

impl Sub for I128 {
    type Output = I128;
    fn sub(self, rhs: Self) -> Self::Output {
        I128(self.0 - rhs.0)
    }
}

// Bitwise
impl Shl for I128 {
    type Output = I128;
    fn shl(self, rhs: Self) -> Self::Output {
        I128(self.0 << rhs.0)
    }
}

impl Shr for I128 {
    type Output = I128;
    fn shr(self, rhs: Self) -> Self::Output {
        I128(self.0 >> rhs.0)
    }
}
