use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Display;

use daggy::{Children, Dag, EdgeIndex, NodeIndex, RecursiveWalk};

use crate::core::model::proc::*;

mod bool;
pub use bool::*;
mod call;
pub use call::*;
mod float;
pub use float::*;
mod int;
pub use int::*;
mod sys;
pub use sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrBlockIdx(NodeIndex);

impl IrBlockIdx {
    fn new(val: NodeIndex) -> Self {
        IrBlockIdx(val)
    }

    fn end() -> Self {
        IrBlockIdx(NodeIndex::end())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrEdgeIdx(EdgeIndex);

impl IrEdgeIdx {
    fn new(val: EdgeIndex) -> Self {
        IrEdgeIdx(val)
    }

    fn end() -> Self {
        IrEdgeIdx(EdgeIndex::end())
    }
}

pub struct IRGraph<P: Processor> {
    root: IrBlockIdx,
    graph: Dag<IrBlock<P>, IrEdge>,
}

impl<P: Processor> IRGraph<P> {
    /// Construct an IRGraph whose initial node, edge, and root node capacities are equal to the given values
    pub fn with_capacity(nodes: usize, edges: usize, root_capacity: usize) -> Self {
        let mut graph = Dag::with_capacity(nodes, edges);
        let root = graph.add_node(IrBlock {
            idx: IrBlockIdx::end(),
            ops: Vec::with_capacity(root_capacity),
        });
        let root = IrBlockIdx::new(root);
        graph.node_weight_mut(root.0).unwrap().idx = root;
        Self { root, graph }
    }

    /// Return the root node index
    pub fn root(&self) -> IrBlockIdx {
        self.root
    }

    /// Return a reference to the specified block
    pub fn block(&self, block: IrBlockIdx) -> &IrBlock<P> {
        self.graph.node_weight(block.0).unwrap()
    }

    /// Return a mutable reference to the specified block
    pub fn block_mut(&mut self, block: IrBlockIdx) -> &mut IrBlock<P> {
        self.graph.node_weight_mut(block.0).unwrap()
    }

    pub fn add_child_block(
        &mut self,
        parent: IrBlockIdx,
        cond: Bit,
        weight: bool,
        capacity: usize,
    ) -> IrBlockIdx {
        let (_, child) = self.graph.add_child(
            parent.0,
            IrEdge(cond, weight),
            IrBlock {
                idx: IrBlockIdx::end(),
                ops: Vec::with_capacity(capacity),
            },
        );
        let child = IrBlockIdx::new(child);
        self.graph.node_weight_mut(child.0).unwrap().idx = child;
        child
    }

    // pub fn recursive_walk(
    //     &self,
    //     start: IrBlockIdx,
    //     f: F,
    // ) -> RecursiveWalk<IrBlock<P>, IrEdge, u32, F>
    // where
    //     F: FnMut(&Dag<IrBlock<P>, IrEdge, u32>, IrBlockIdx) -> Option<(IrEdgeIdx, IrBlockIdx)>,
    // {
    //     let recursive_fn = |graph, node| f(graph, IrBlockIdx(node));
    //     self.graph.recursive_walk(start.0, recursive_fn)
    // }

    pub fn pop(mut self) -> IrBlock<P> {
        // TODO: heinous!
        self.graph.remove_node(self.root.0).unwrap()
        // todo!()
    }
}

pub struct IrBlock<P: Processor> {
    idx: IrBlockIdx,
    ops: Vec<(IRRes, IROp<P>)>,
}

impl<P: Processor> IntoIterator for IrBlock<P> {
    type Item = (IRRes, IROp<P>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.ops.into_iter()
    }
}

#[derive(Clone, Copy)]
pub struct IrEdge(Bit, bool);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IRRes {
    pub def0: Option<Def>,
    pub def1: Option<Def>,
}

impl IRRes {
    fn none() -> Self {
        IRRes {
            def0: None,
            def1: None,
        }
    }

    fn one(def: Def) -> Self {
        IRRes {
            def0: Some(def),
            def1: None,
        }
    }

    fn two(def0: Def, def1: Def) -> Self {
        IRRes {
            def0: Some(def0),
            def1: Some(def1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Def {
    node: IrBlockIdx,
    line: usize,
    retval: usize,
}

impl Display for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:?}_{1}_{2}", self.node, self.line, self.retval)
    }
}

pub enum IROp<P: Processor> {
    // System control
    Br(Addr, P::BrAttrs),
    Cbr(Bit, Addr, Addr, P::BrAttrs),
    // RaiseExcp(P::ExcepAttrs), // TODO: support IR-generated exceptions
    // System access
    RReg(RRegOp),
    WReg(WRegOp),
    RMem(RMemOp<P>),
    WMem(WMemOp<P>),
    // IR-control
    Call(CallOp<P>),
    Nop,
    // Data-processing
    // Addr(AddrOp),
    Bool(BoolOp),
    Int8(Int8Op),
    Int16(Int16Op),
    Int32(Int32Op),
    Int64(Int64Op),
    Int128(Int128Op),
    Float16(Float16Op),
    Float32(Float32Op),
    Float64(Float64Op),
}

// Interface to the Processor model

pub struct ProcIRCtx {
    pub(super) offsets: HashMap<TypeId, usize>,
}

impl ProcIRCtx {
    pub fn reg1<T>(&self) -> Reg1
    where
        T: RegisterView,
        Assert<{ T::LEN == 1 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg1(base_offset + T::OFFSET)
    }

    pub fn arr1<T>(&self, idx: usize) -> Reg1
    where
        T: RegisterArrayView,
        Assert<{ T::LEN == 1 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg1(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }

    pub fn reg8<T>(&self) -> Reg8
    where
        T: RegisterView,
        Assert<{ T::LEN <= 8 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg8(base_offset + T::OFFSET)
    }

    pub fn arr8<T>(&self, idx: usize) -> Reg8
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 8 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg8(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }

    pub fn reg16<T>(&self) -> Reg16
    where
        T: RegisterView,
        Assert<{ T::LEN <= 16 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg16(base_offset + T::OFFSET)
    }

    pub fn arr16<T>(&self, idx: usize) -> Reg16
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 16 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg16(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }

    pub fn reg32<T>(&self) -> Reg32
    where
        T: RegisterView,
        Assert<{ T::LEN <= 32 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg32(base_offset + T::OFFSET)
    }

    pub fn arr32<T>(&self, idx: usize) -> Reg32
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 32 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg32(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }

    pub fn reg64<T>(&self) -> Reg64
    where
        T: RegisterView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg64(base_offset + T::OFFSET)
    }

    pub fn arr64<T>(&self, idx: usize) -> Reg64
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 64 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg64(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }

    pub fn reg128<T>(&self) -> Reg128
    where
        T: RegisterView,
        Assert<{ T::LEN <= 128 }>: IsTrue,
    {
        let base_offset = self
            .offsets
            .get(&TypeId::of::<T::Register>())
            .unwrap_or_else(|| panic!("Unknown Register {0}.", T::NAME))
            .clone();
        Reg128(base_offset + T::OFFSET)
    }

    pub fn arr128<T>(&self, idx: usize) -> Reg128
    where
        T: RegisterArrayView,
        Assert<{ T::LEN <= 128 }>: IsTrue,
    {
        let array_offset = self
            .offsets
            .get(&TypeId::of::<T::RegisterArray>())
            .unwrap_or_else(|| panic!("Unknown Register {0}", T::NAME))
            .clone();

        if idx >= T::RegisterArray::ELEMS {
            panic!(
                "RegisterArray access out of bounds: {0} at {idx} (max is {1})",
                T::NAME,
                T::RegisterArray::ELEMS
            );
        }
        Reg128(array_offset + T::RegisterArray::ELEM_LEN * idx + T::OFFSET)
    }
}

// Primitive types

pub struct Reg1(pub(super) usize);

pub struct Reg8(pub(super) usize);

pub struct Reg16(pub(super) usize);

pub struct Reg32(pub(super) usize);

pub struct Reg64(pub(super) usize);

pub struct Reg128(pub(super) usize);

pub type Addr = Bv64;

/// A 1-bit value
#[derive(Clone, Copy)]
pub enum Bit {
    Lit(bool),
    Var(Def),
}

impl Bit {
    pub fn lit(v: bool) -> Self {
        Bit::Lit(v)
    }

    pub fn zero() -> Self {
        Bit::Lit(false)
    }

    pub fn one() -> Self {
        Bit::Lit(true)
    }
}

/// An 8-bit value
#[derive(Clone, Copy)]
pub enum Bv8 {
    Lit(u8),
    Var(Def),
}

impl Bv8 {
    pub fn zero() -> Self {
        Self::Lit(0)
    }

    pub fn one() -> Self {
        Self::Lit(1)
    }
}

/// A 16-bit value
#[derive(Clone, Copy)]
pub enum Bv16 {
    Lit(u16),
    Var(Def),
}

impl Bv16 {
    pub fn zero() -> Self {
        Self::Lit(0)
    }

    pub fn one() -> Self {
        Self::Lit(1)
    }
}

/// A 32-bit value
#[derive(Clone, Copy)]
pub enum Bv32 {
    Lit(u32),
    Var(Def),
}

impl Bv32 {
    pub fn zero() -> Self {
        Self::Lit(0)
    }

    pub fn one() -> Self {
        Self::Lit(1)
    }
}

/// A 64-bit value
#[derive(Clone, Copy)]
pub enum Bv64 {
    Lit(u64),
    Var(Def),
}

impl Bv64 {
    pub fn zero() -> Self {
        Self::Lit(0)
    }

    pub fn one() -> Self {
        Self::Lit(1)
    }
}

/// A 128-bit value
#[derive(Clone, Copy)]
pub enum Bv128 {
    Lit(u128),
    Var(Def),
}

impl Bv128 {
    pub fn zero() -> Self {
        Self::Lit(0)
    }

    pub fn one() -> Self {
        Self::Lit(1)
    }
}
