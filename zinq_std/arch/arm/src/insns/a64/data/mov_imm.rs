use bitvec::prelude::*;
use zinq::insn::{semantics::*, syntax::Decodable, Instruction};

use crate::{
    insns::{a64, disas, helpers::*},
    Arm,
};

#[derive(Debug, Clone)]
enum MovOp {
    K,
    N,
    Z,
}

impl From<(bool, bool)> for MovOp {
    fn from(value: (bool, bool)) -> Self {
        let opc_0 = value.0;
        let opc_1 = value.1;

        match (opc_1, opc_0) {
            (false, false) => MovOp::N,
            (false, true) => panic!("Unreachable"),
            (true, false) => MovOp::Z,
            (true, true) => MovOp::K,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovImm {
    raw: u32,
    sf: bool,
    opc: MovOp,
    hw_1: bool,
    hw_0: bool,
    imm16: BitArr!(for 16),
    rd: usize,
}

impl Decodable<a64::InsnSize> for MovImm {
    const FIXEDBITS: a64::InsnSize = 0b0001_0010_1000_0000_0000_0000_0000_0000;
    const FIXEDMASK: a64::InsnSize = 0b0001_1111_1000_0000_0000_0000_0000_0000;
}

impl Instruction<Arm> for MovImm {
    type InsnSize = a64::InsnSize;

    fn decode(bits: &BitSlice) -> Option<Self> {
        let mut imm16 = BitArray::ZERO;
        imm16 |= bits.get(5..21)?;

        let insn = Self {
            raw: bits.get(0..32)?.load(),
            sf: *bits.get(31)?,
            opc: bits.get(29..31).and_then(|bits| match (bits[1], bits[0]) {
                (false, false) => Some(MovOp::N),
                (false, true) => None,
                (true, false) => Some(MovOp::Z),
                (true, true) => Some(MovOp::K),
            })?,
            hw_1: *bits.get(22)?,
            hw_0: *bits.get(21)?,
            imm16,
            rd: bits.get(0..5)?.load(),
        };

        if !insn.sf && insn.hw_1 {
            return None; // Undefined
        }

        Some(insn)
    }

    fn assemble(&self) -> &u32 {
        &self.raw
    }

    fn disassemble(&self, proc: &Arm) -> String {
        disas::a64(self.raw, proc)
    }

    fn size(&self) -> usize {
        4
    }

    fn semantics<'p>(&self, proc: &'p Arm) -> IrBlock<'p> {
        let mut code = IrBlock::new();

        // Decode
        let datasize = if self.sf { 64 } else { 32 };
        let pos = (((self.hw_1 as usize) << 1) | (self.hw_0 as usize)) << 4;

        // result : bits('datasize) = undefined;
        // if opcode == MoveWideOp_K then {
        //     result = X_read(d, datasize)
        // } else {
        //     result = Zeros(datasize)
        // };
        let result = match self.opc {
            MovOp::K => x_read(self.rd, datasize, proc, &mut code),
            _ => zeros(datasize, &mut code),
        };

        // result[pos + 15 .. pos] = imm;
        let result = code.assign(Expr::Merge {
            lhs: Term::Var(result),
            start_lhs: pos,
            rhs: Term::Lit(self.imm16.to_bitvec()),
            start_rhs: 0,
            len: 16,
        });

        // if opcode == MoveWideOp_N then {
        //     result = not_vec(result)
        // };
        let result = match self.opc {
            MovOp::N => code.assign(Expr::Unary(UnaOp::Not, Term::Var(result))),
            _ => result,
        };

        // X_set(d, datasize) = result
        x_set(self.rd, Term::Var(result), proc, &mut code);

        inc_pc(proc, &mut code);

        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Version::Armv9p4a;
    use zinq::{
        system::{Processor, System},
        Emulator,
    };
    use zinq_std_emu::StepEmu;

    fn run_test(test_case: &[u8], mem_size: usize, x0: u64) -> System<Arm> {
        let mut vm = System::new(Arm::new(Armv9p4a), mem_size);
        vm.write_mem(0, test_case).unwrap();

        let proc = vm.proc_mut();
        proc.set_ip(0);
        proc.set_x(0, x0);

        let mut emu = StepEmu::new();
        emu.max_insns(1);
        emu.run(&mut vm);
        vm
    }

    #[test]
    fn movk_64() {
        // MOVK X0, #1, LSL 48
        let test_case = [0x20, 0x00, 0xE0, 0xF2];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0x0001aaaaaaaaaaaa));
    }

    #[test]
    fn movk_32() {
        // MOVK W0, #1
        let test_case = [0x20, 0x00, 0x80, 0x72];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0xaaaaaaaaaaaa0001));
    }

    #[test]
    fn movn_64() {
        // MOVN X0, #1, LSL 16
        let test_case = [0x20, 0x00, 0xA0, 0x92];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0xfffffffffffeffff));
    }

    #[test]
    fn movn_32() {
        // MOVN X0, #1, LSL 16
        let test_case = [0x20, 0x00, 0xA0, 0x12];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0xaaaaaaaafffeffff));
    }

    #[test]
    fn movz_64() {
        // MOVZ X0, #1, LSL 32
        let test_case = [0x20, 0x00, 0xC0, 0xD2];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0x100000000));
    }

    #[test]
    fn movz_32() {
        // MOVZ W0, #1, LSL 16
        let test_case = [0x20, 0x00, 0xA0, 0x52];
        let vm = run_test(&test_case, 8, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(vm.proc().x(0), Some(0xaaaaaaaa00010000));
    }
}
