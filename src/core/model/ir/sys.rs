use crate::core::model::ir::*;

pub enum RRegOp {
    Bit(Reg1),
    Bv8(Reg8),
    Bv16(Reg16),
    Bv32(Reg32),
    Bv64(Reg64),
    Bv128(Reg128),
}

pub enum WRegOp {
    Bit(Reg1, Bit),
    Bv8(Reg8, Bv8),
    Bv16(Reg16, Bv16),
    Bv32(Reg32, Bv32),
    Bv64(Reg64, Bv64),
    Bv128(Reg128, Bv128),
}

pub enum RMemOp<P: Processor> {
    Bv8(Addr, P::LdstAttrs),
    Bv16(Addr, P::LdstAttrs),
    Bv32(Addr, P::LdstAttrs),
    Bv64(Addr, P::LdstAttrs),
    Bv128(Addr, P::LdstAttrs),
}

pub enum WMemOp<P: Processor> {
    Bv8(Addr, Bv8, P::LdstAttrs),
    Bv16(Addr, Bv16, P::LdstAttrs),
    Bv32(Addr, Bv32, P::LdstAttrs),
    Bv64(Addr, Bv64, P::LdstAttrs),
    Bv128(Addr, Bv128, P::LdstAttrs),
}

impl<P: Processor> IrBlock<P> {
    pub fn br(&mut self, target: Addr, attrs: P::BrAttrs) {
        self.ops.push((IRRes::none(), IROp::Br(target, attrs)));
    }

    pub fn cbr(&mut self, cond: Bit, t_target: Addr, f_target: Addr, attrs: P::BrAttrs) {
        self.ops
            .push((IRRes::none(), IROp::Cbr(cond, t_target, f_target, attrs)));
    }

    pub fn read_reg1(&mut self, reg: Reg1) -> Bit {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bit(reg))));
        Bit::Var(def)
    }

    pub fn write_reg1(&mut self, reg: Reg1, val: Bit) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bit(reg, val))));
    }

    pub fn read_reg8(&mut self, reg: Reg8) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bv8(reg))));
        Bv8::Var(def)
    }

    pub fn write_reg8(&mut self, reg: Reg8, val: Bv8) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bv8(reg, val))));
    }

    pub fn read_mem8(&mut self, addr: Addr, attrs: P::LdstAttrs) -> Bv8 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RMem(RMemOp::Bv8(addr, attrs))));
        Bv8::Var(def)
    }

    pub fn write_mem8(&mut self, addr: Addr, val: Bv8, attrs: P::LdstAttrs) {
        self.ops
            .push((IRRes::none(), IROp::WMem(WMemOp::Bv8(addr, val, attrs))));
    }

    pub fn read_reg16(&mut self, reg: Reg16) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bv16(reg))));
        Bv16::Var(def)
    }

    pub fn write_reg16(&mut self, reg: Reg16, val: Bv16) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bv16(reg, val))));
    }

    pub fn read_mem16(&mut self, addr: Addr, attrs: P::LdstAttrs) -> Bv16 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RMem(RMemOp::Bv16(addr, attrs))));
        Bv16::Var(def)
    }

    pub fn write_mem16(&mut self, addr: Addr, val: Bv16, attrs: P::LdstAttrs) {
        self.ops
            .push((IRRes::none(), IROp::WMem(WMemOp::Bv16(addr, val, attrs))));
    }

    pub fn read_reg32(&mut self, reg: Reg32) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bv32(reg))));
        Bv32::Var(def)
    }

    pub fn write_reg32(&mut self, reg: Reg32, val: Bv32) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bv32(reg, val))));
    }

    pub fn read_mem32(&mut self, addr: Addr, attrs: P::LdstAttrs) -> Bv32 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RMem(RMemOp::Bv32(addr, attrs))));
        Bv32::Var(def)
    }

    pub fn write_mem32(&mut self, addr: Addr, val: Bv32, attrs: P::LdstAttrs) {
        self.ops
            .push((IRRes::none(), IROp::WMem(WMemOp::Bv32(addr, val, attrs))));
    }

    pub fn read_reg64(&mut self, reg: Reg64) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bv64(reg))));
        Bv64::Var(def)
    }

    pub fn write_reg64(&mut self, reg: Reg64, val: Bv64) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bv64(reg, val))));
    }

    pub fn read_mem64(&mut self, addr: Addr, attrs: P::LdstAttrs) -> Bv64 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RMem(RMemOp::Bv64(addr, attrs))));
        Bv64::Var(def)
    }

    pub fn write_mem64(&mut self, addr: Addr, val: Bv64, attrs: P::LdstAttrs) {
        self.ops
            .push((IRRes::none(), IROp::WMem(WMemOp::Bv64(addr, val, attrs))));
    }

    pub fn read_reg128(&mut self, reg: Reg128) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RReg(RRegOp::Bv128(reg))));
        Bv128::Var(def)
    }

    pub fn write_reg128(&mut self, reg: Reg128, val: Bv128) {
        self.ops
            .push((IRRes::none(), IROp::WReg(WRegOp::Bv128(reg, val))));
    }

    pub fn read_mem128(&mut self, addr: Addr, attrs: P::LdstAttrs) -> Bv128 {
        let def = Def {
            node: self.idx,
            line: self.ops.len(),
            retval: 0,
        };
        self.ops
            .push((IRRes::one(def), IROp::RMem(RMemOp::Bv128(addr, attrs))));
        Bv128::Var(def)
    }

    pub fn write_mem128(&mut self, addr: Addr, val: Bv128, attrs: P::LdstAttrs) {
        self.ops
            .push((IRRes::none(), IROp::WMem(WMemOp::Bv128(addr, val, attrs))));
    }

    pub fn nop(&mut self) {
        self.ops.push((IRRes::none(), IROp::Nop));
    }
}
