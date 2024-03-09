#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use bitvec::prelude::*;
use capstone::Endian;
use zinq::{insn::Instruction, system::Processor, Error, Result};
pub mod insns {
    use crate::Arm;
    use zinq_macros::insn_set;
    mod disas {
        use bitvec::prelude::*;
        use capstone::{
            arch::{arm64::ArchMode, BuildsCapstone, BuildsCapstoneEndian},
            Capstone, Endian,
        };
        use crate::Arm;
        /// Capstone-based disassembly for all A64 instructions
        pub fn a64(raw: u32, proc: &Arm) -> String {
            let cap = Capstone::new()
                .arm64()
                .mode(ArchMode::Arm)
                .endian(proc.endian)
                .build()
                .expect("Unable to build Arm64 Capstone instance");
            let bytes = match proc.endian {
                Endian::Big => raw.to_be_bytes(),
                Endian::Little => raw.to_le_bytes(),
            };
            let disas = cap
                .disasm_all(&bytes, proc.pc.load())
                .expect("Unable to disassemble instruction");
            let disas = disas
                .first()
                .expect(
                    "Instruction was disassembled but could not be unpacked from Capstone",
                );
            {
                let res = ::alloc::fmt::format(format_args!("{0}", disas));
                res
            }
        }
    }
    mod helpers {
        mod extend_reg {
            use bitvec::prelude::*;
            use zinq::insn::semantics::IrCtx;
            use crate::Arm;
            pub enum ExtendType {
                SXTB,
                SXTH,
                SXTW,
                SXTX,
                UXTB,
                UXTH,
                UXTW,
                UXTX,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ExtendType {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ExtendType::SXTB => "SXTB",
                            ExtendType::SXTH => "SXTH",
                            ExtendType::SXTW => "SXTW",
                            ExtendType::SXTX => "SXTX",
                            ExtendType::UXTB => "UXTB",
                            ExtendType::UXTH => "UXTH",
                            ExtendType::UXTW => "UXTW",
                            ExtendType::UXTX => "UXTX",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExtendType {
                #[inline]
                fn clone(&self) -> ExtendType {
                    match self {
                        ExtendType::SXTB => ExtendType::SXTB,
                        ExtendType::SXTH => ExtendType::SXTH,
                        ExtendType::SXTW => ExtendType::SXTW,
                        ExtendType::SXTX => ExtendType::SXTX,
                        ExtendType::UXTB => ExtendType::UXTB,
                        ExtendType::UXTH => ExtendType::UXTH,
                        ExtendType::UXTW => ExtendType::UXTW,
                        ExtendType::UXTX => ExtendType::UXTX,
                    }
                }
            }
            impl<T: AsRef<BitSlice>> From<T> for ExtendType {
                fn from(value: T) -> Self {
                    let bit2 = *value.as_ref().get(2).expect("Cannot build ExtendType");
                    let bit1 = *value.as_ref().get(1).expect("Cannot build ExtendType");
                    let bit0 = *value.as_ref().get(0).expect("Cannot build ExtendType");
                    match (bit2, bit1, bit0) {
                        (false, false, false) => Self::UXTB,
                        (false, false, true) => Self::UXTH,
                        (false, true, false) => Self::UXTW,
                        (false, true, true) => Self::UXTX,
                        (true, false, false) => Self::SXTB,
                        (true, false, true) => Self::SXTH,
                        (true, true, false) => Self::SXTW,
                        (true, true, true) => Self::SXTX,
                    }
                }
            }
        }
        pub use extend_reg::*;
        mod integer {
            use crate::Arm;
            use bitvec::prelude::*;
            use zinq::insn::semantics::*;
            use zinq_macros::ir_fn;
            pub fn add_with_carry_I32(
                x: Term<I32>,
                y: Term<I32>,
                carry: Term<I1>,
            ) -> (Var<I32>, (Var<bool>, Var<bool>, Var<bool>, Var<bool>)) {}
            pub fn add_with_carry_I64(
                x: Term<I64>,
                y: Term<I64>,
                carry: Term<I1>,
            ) -> (Var<I64>, (Var<bool>, Var<bool>, Var<bool>, Var<bool>)) {}
        }
        pub use integer::*;
        mod registers {
            use bitvec::prelude::*;
            use zinq::insn::semantics::*;
            use crate::Arm;
        }
        pub use registers::*;
        mod enums {
            use std::fmt;
            pub enum MoveWideOp {
                N,
                Z,
                K,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MoveWideOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MoveWideOp::N => "N",
                            MoveWideOp::Z => "Z",
                            MoveWideOp::K => "K",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MoveWideOp {
                #[inline]
                fn clone(&self) -> MoveWideOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MoveWideOp {}
            pub enum ShiftType {
                LSL,
                LSR,
                ASR,
                ROR,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ShiftType {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ShiftType::LSL => "LSL",
                            ShiftType::LSR => "LSR",
                            ShiftType::ASR => "ASR",
                            ShiftType::ROR => "ROR",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ShiftType {
                #[inline]
                fn clone(&self) -> ShiftType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ShiftType {}
            impl From<(bool, bool)> for ShiftType {
                fn from(value: (bool, bool)) -> Self {
                    let (high, low) = value;
                    match (high, low) {
                        (false, false) => ShiftType::LSL,
                        (false, true) => ShiftType::LSR,
                        (true, false) => ShiftType::ASR,
                        (true, true) => ShiftType::ROR,
                    }
                }
            }
            impl fmt::Display for ShiftType {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        Self::LSL => f.write_fmt(format_args!("LSL")),
                        Self::LSR => f.write_fmt(format_args!("LSR")),
                        Self::ASR => f.write_fmt(format_args!("ASR")),
                        Self::ROR => f.write_fmt(format_args!("ROR")),
                    }
                }
            }
            pub enum LogicalOp {
                AND,
                EOR,
                ORR,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LogicalOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            LogicalOp::AND => "AND",
                            LogicalOp::EOR => "EOR",
                            LogicalOp::ORR => "ORR",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LogicalOp {
                #[inline]
                fn clone(&self) -> LogicalOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for LogicalOp {}
            pub enum CountOp {
                CLZ,
                CLS,
                CNT,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for CountOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            CountOp::CLZ => "CLZ",
                            CountOp::CLS => "CLS",
                            CountOp::CNT => "CNT",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for CountOp {
                #[inline]
                fn clone(&self) -> CountOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for CountOp {}
            pub enum FPMaxMinOp {
                MAX,
                MIN,
                MAXNUM,
                MINNUM,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FPMaxMinOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FPMaxMinOp::MAX => "MAX",
                            FPMaxMinOp::MIN => "MIN",
                            FPMaxMinOp::MAXNUM => "MAXNUM",
                            FPMaxMinOp::MINNUM => "MINNUM",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FPMaxMinOp {
                #[inline]
                fn clone(&self) -> FPMaxMinOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for FPMaxMinOp {}
            pub enum FPUnaryOp {
                ABS,
                MOV,
                NEG,
                SQRT,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FPUnaryOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FPUnaryOp::ABS => "ABS",
                            FPUnaryOp::MOV => "MOV",
                            FPUnaryOp::NEG => "NEG",
                            FPUnaryOp::SQRT => "SQRT",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FPUnaryOp {
                #[inline]
                fn clone(&self) -> FPUnaryOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for FPUnaryOp {}
            pub enum FPConvOp {
                CvtFtoI,
                CvtItoF,
                MovFtoI,
                MovItoF,
                CvtFtoIJs,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FPConvOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FPConvOp::CvtFtoI => "CvtFtoI",
                            FPConvOp::CvtItoF => "CvtItoF",
                            FPConvOp::MovFtoI => "MovFtoI",
                            FPConvOp::MovItoF => "MovItoF",
                            FPConvOp::CvtFtoIJs => "CvtFtoIJs",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FPConvOp {
                #[inline]
                fn clone(&self) -> FPConvOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for FPConvOp {}
            pub enum FPRounding {
                TIEEVEN,
                POSINF,
                NEGINF,
                ZERO,
                TIEAWAY,
                ODD,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FPRounding {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FPRounding::TIEEVEN => "TIEEVEN",
                            FPRounding::POSINF => "POSINF",
                            FPRounding::NEGINF => "NEGINF",
                            FPRounding::ZERO => "ZERO",
                            FPRounding::TIEAWAY => "TIEAWAY",
                            FPRounding::ODD => "ODD",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FPRounding {
                #[inline]
                fn clone(&self) -> FPRounding {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for FPRounding {}
            pub enum MemOp {
                LOAD,
                STORE,
                PREFETCH,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MemOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MemOp::LOAD => "LOAD",
                            MemOp::STORE => "STORE",
                            MemOp::PREFETCH => "PREFETCH",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MemOp {
                #[inline]
                fn clone(&self) -> MemOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MemOp {}
            pub enum MemAtomicOp {
                GCSSS1,
                ADD,
                BIC,
                EOR,
                ORR,
                SMAX,
                SMIN,
                UMAX,
                UMIN,
                SWP,
                CAS,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MemAtomicOp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MemAtomicOp::GCSSS1 => "GCSSS1",
                            MemAtomicOp::ADD => "ADD",
                            MemAtomicOp::BIC => "BIC",
                            MemAtomicOp::EOR => "EOR",
                            MemAtomicOp::ORR => "ORR",
                            MemAtomicOp::SMAX => "SMAX",
                            MemAtomicOp::SMIN => "SMIN",
                            MemAtomicOp::UMAX => "UMAX",
                            MemAtomicOp::UMIN => "UMIN",
                            MemAtomicOp::SWP => "SWP",
                            MemAtomicOp::CAS => "CAS",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MemAtomicOp {
                #[inline]
                fn clone(&self) -> MemAtomicOp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MemAtomicOp {}
            pub enum MOPSStage {
                Prologue,
                Main,
                Epilogue,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MOPSStage {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MOPSStage::Prologue => "Prologue",
                            MOPSStage::Main => "Main",
                            MOPSStage::Epilogue => "Epilogue",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MOPSStage {
                #[inline]
                fn clone(&self) -> MOPSStage {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MOPSStage {}
            /// Arm condition codes
            pub enum CondCode {
                EQ,
                NE,
                CS,
                CC,
                MI,
                PL,
                VS,
                VC,
                HI,
                LS,
                GE,
                LT,
                GT,
                LE,
                AL,
                NV,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for CondCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            CondCode::EQ => "EQ",
                            CondCode::NE => "NE",
                            CondCode::CS => "CS",
                            CondCode::CC => "CC",
                            CondCode::MI => "MI",
                            CondCode::PL => "PL",
                            CondCode::VS => "VS",
                            CondCode::VC => "VC",
                            CondCode::HI => "HI",
                            CondCode::LS => "LS",
                            CondCode::GE => "GE",
                            CondCode::LT => "LT",
                            CondCode::GT => "GT",
                            CondCode::LE => "LE",
                            CondCode::AL => "AL",
                            CondCode::NV => "NV",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for CondCode {
                #[inline]
                fn clone(&self) -> CondCode {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for CondCode {}
            impl From<u8> for CondCode {
                fn from(value: u8) -> Self {
                    match value {
                        0b0000 => Self::EQ,
                        0b0001 => Self::NE,
                        0b0010 => Self::CS,
                        0b0011 => Self::CC,
                        0b0100 => Self::MI,
                        0b0101 => Self::PL,
                        0b0110 => Self::VS,
                        0b0111 => Self::VC,
                        0b1000 => Self::HI,
                        0b1001 => Self::LS,
                        0b1010 => Self::GE,
                        0b1011 => Self::LT,
                        0b1100 => Self::GT,
                        0b1101 => Self::LE,
                        0b1110 => Self::AL,
                        0b1111 => Self::NV,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Invalid A64 condition code. Arm instructions should never allow this to happen.",
                                ),
                            );
                        }
                    }
                }
            }
            impl fmt::Display for CondCode {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        CondCode::EQ => f.write_fmt(format_args!("EQ")),
                        CondCode::NE => f.write_fmt(format_args!("NE")),
                        CondCode::CS => f.write_fmt(format_args!("CS")),
                        CondCode::CC => f.write_fmt(format_args!("CC")),
                        CondCode::MI => f.write_fmt(format_args!("MI")),
                        CondCode::PL => f.write_fmt(format_args!("PL")),
                        CondCode::VS => f.write_fmt(format_args!("VS")),
                        CondCode::VC => f.write_fmt(format_args!("VC")),
                        CondCode::HI => f.write_fmt(format_args!("HI")),
                        CondCode::LS => f.write_fmt(format_args!("LS")),
                        CondCode::GE => f.write_fmt(format_args!("GE")),
                        CondCode::LT => f.write_fmt(format_args!("LT")),
                        CondCode::GT => f.write_fmt(format_args!("GT")),
                        CondCode::LE => f.write_fmt(format_args!("LE")),
                        CondCode::AL => f.write_fmt(format_args!("AL")),
                        CondCode::NV => f.write_fmt(format_args!("NV")),
                    }
                }
            }
            pub enum BranchType {
                DIRCALL,
                INDCALL,
                ERET,
                DBGEXIT,
                RET,
                DIR,
                INDIR,
                EXCEPTION,
                TMFAIL,
                RESET,
                UNKNOWN,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for BranchType {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            BranchType::DIRCALL => "DIRCALL",
                            BranchType::INDCALL => "INDCALL",
                            BranchType::ERET => "ERET",
                            BranchType::DBGEXIT => "DBGEXIT",
                            BranchType::RET => "RET",
                            BranchType::DIR => "DIR",
                            BranchType::INDIR => "INDIR",
                            BranchType::EXCEPTION => "EXCEPTION",
                            BranchType::TMFAIL => "TMFAIL",
                            BranchType::RESET => "RESET",
                            BranchType::UNKNOWN => "UNKNOWN",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for BranchType {
                #[inline]
                fn clone(&self) -> BranchType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for BranchType {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for BranchType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for BranchType {
                #[inline]
                fn eq(&self, other: &BranchType) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for BranchType {}
            #[automatically_derived]
            impl ::core::cmp::Eq for BranchType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            pub enum MBReqDomain {
                Nonshareable,
                InnerShareable,
                OuterShareable,
                FullSystem,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MBReqDomain {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MBReqDomain::Nonshareable => "Nonshareable",
                            MBReqDomain::InnerShareable => "InnerShareable",
                            MBReqDomain::OuterShareable => "OuterShareable",
                            MBReqDomain::FullSystem => "FullSystem",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MBReqDomain {
                #[inline]
                fn clone(&self) -> MBReqDomain {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MBReqDomain {}
            pub enum MBReqTypes {
                Reads,
                Writes,
                All,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MBReqTypes {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            MBReqTypes::Reads => "Reads",
                            MBReqTypes::Writes => "Writes",
                            MBReqTypes::All => "All",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MBReqTypes {
                #[inline]
                fn clone(&self) -> MBReqTypes {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for MBReqTypes {}
        }
        pub use enums::*;
    }
    pub mod a64 {
        pub type InsnSize = u32;
        pub mod data_proc {
            pub mod imm {
                pub mod arithmetic {
                    use crate::insns::helpers::*;
                    use crate::insns::{a64, disas};
                    use crate::Arm;
                    use bitvec::prelude::*;
                    use zinq::insn::semantics::*;
                    use zinq::insn::syntax::Decodable;
                    use zinq::insn::Instruction;
                    fn decode(
                        bits: &BitSlice,
                    ) -> Option<(u32, BitVec, usize, usize, usize)> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let sf = *bits.get(31)?;
                        let sh = *bits.get(22)?;
                        let imm12 = bits.get(10..22)?;
                        let n = bits.get(5..10)?.load();
                        let d = bits.get(0..5)?.load();
                        let datasize = if sf { 64 } else { 32 };
                        let mut imm = BitVec::with_capacity(datasize);
                        match sh {
                            false => {
                                imm.extend_from_bitslice(imm12);
                                imm.resize(datasize, false);
                            }
                            true => {
                                imm.resize(12, false);
                                imm.extend_from_bitslice(imm12);
                                imm.resize(datasize, false);
                            }
                        }
                        Some((raw, imm, datasize, n, d))
                    }
                    pub struct Add {
                        raw: u32,
                        datasize: usize,
                        d: usize,
                        imm: BitVec,
                        n: usize,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Add {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field5_finish(
                                f,
                                "Add",
                                "raw",
                                &self.raw,
                                "datasize",
                                &self.datasize,
                                "d",
                                &self.d,
                                "imm",
                                &self.imm,
                                "n",
                                &&self.n,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Add {
                        #[inline]
                        fn clone(&self) -> Add {
                            Add {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                                n: ::core::clone::Clone::clone(&self.n),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Add {
                        const FIXEDBITS: a64::InsnSize = 0b00010001000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
                    }
                    impl Instruction<Arm> for Add {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let (raw, imm, datasize, n, d) = decode(bits)?;
                            Some(Self { raw, d, datasize, imm, n })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                    pub struct Adds {
                        raw: u32,
                        datasize: usize,
                        d: usize,
                        imm: BitVec,
                        n: usize,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Adds {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field5_finish(
                                f,
                                "Adds",
                                "raw",
                                &self.raw,
                                "datasize",
                                &self.datasize,
                                "d",
                                &self.d,
                                "imm",
                                &self.imm,
                                "n",
                                &&self.n,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Adds {
                        #[inline]
                        fn clone(&self) -> Adds {
                            Adds {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                                n: ::core::clone::Clone::clone(&self.n),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Adds {
                        const FIXEDBITS: a64::InsnSize = 0b00110001000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
                    }
                    impl Instruction<Arm> for Adds {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let (raw, imm, datasize, n, d) = decode(bits)?;
                            Some(Self { raw, d, datasize, imm, n })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                    pub struct Sub {
                        raw: u32,
                        datasize: usize,
                        d: usize,
                        imm: BitVec,
                        n: usize,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Sub {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field5_finish(
                                f,
                                "Sub",
                                "raw",
                                &self.raw,
                                "datasize",
                                &self.datasize,
                                "d",
                                &self.d,
                                "imm",
                                &self.imm,
                                "n",
                                &&self.n,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Sub {
                        #[inline]
                        fn clone(&self) -> Sub {
                            Sub {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                                n: ::core::clone::Clone::clone(&self.n),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Sub {
                        const FIXEDBITS: a64::InsnSize = 0b01010001000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
                    }
                    impl Instruction<Arm> for Sub {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let (raw, imm, datasize, n, d) = decode(bits)?;
                            Some(Self { raw, d, datasize, imm, n })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                    pub struct Subs {
                        raw: u32,
                        datasize: usize,
                        d: usize,
                        imm: BitVec,
                        n: usize,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Subs {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field5_finish(
                                f,
                                "Subs",
                                "raw",
                                &self.raw,
                                "datasize",
                                &self.datasize,
                                "d",
                                &self.d,
                                "imm",
                                &self.imm,
                                "n",
                                &&self.n,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Subs {
                        #[inline]
                        fn clone(&self) -> Subs {
                            Subs {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                                n: ::core::clone::Clone::clone(&self.n),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Subs {
                        const FIXEDBITS: a64::InsnSize = 0b01110001000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b01111111100000000000000000000000;
                    }
                    impl Instruction<Arm> for Subs {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let (raw, imm, datasize, n, d) = decode(bits)?;
                            Some(Self { raw, d, datasize, imm, n })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                }
                pub mod logical {
                    use crate::insns::helpers::*;
                    use crate::insns::{a64, disas};
                    use crate::Arm;
                    use bitvec::prelude::*;
                    use zinq::insn::semantics::*;
                    use zinq::insn::syntax::Decodable;
                    use zinq::insn::Instruction;
                    pub struct IntegerLogicalImmediate {
                        raw: u32,
                        d: usize,
                        datasize: usize,
                        imm: BitVec,
                        n: usize,
                        op: LogicalOp,
                        setflags: bool,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for IntegerLogicalImmediate {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let names: &'static _ = &[
                                "raw",
                                "d",
                                "datasize",
                                "imm",
                                "n",
                                "op",
                                "setflags",
                            ];
                            let values: &[&dyn ::core::fmt::Debug] = &[
                                &self.raw,
                                &self.d,
                                &self.datasize,
                                &self.imm,
                                &self.n,
                                &self.op,
                                &&self.setflags,
                            ];
                            ::core::fmt::Formatter::debug_struct_fields_finish(
                                f,
                                "IntegerLogicalImmediate",
                                names,
                                values,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for IntegerLogicalImmediate {
                        #[inline]
                        fn clone(&self) -> IntegerLogicalImmediate {
                            IntegerLogicalImmediate {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                d: ::core::clone::Clone::clone(&self.d),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                imm: ::core::clone::Clone::clone(&self.imm),
                                n: ::core::clone::Clone::clone(&self.n),
                                op: ::core::clone::Clone::clone(&self.op),
                                setflags: ::core::clone::Clone::clone(&self.setflags),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for IntegerLogicalImmediate {
                        const FIXEDBITS: a64::InsnSize = 0b00010010000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b00011111100000000000000000000000;
                    }
                    impl Instruction<Arm> for IntegerLogicalImmediate {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                            let rd = bits.get(0..5)?;
                            let rn = bits.get(5..10)?;
                            let imms = bits.get(10..16)?;
                            let immr = bits.get(16..22)?;
                            let n = *bits.get(22)?;
                            let opc_0 = *bits.get(29)?;
                            let opc_1 = *bits.get(30)?;
                            let sf = *bits.get(31)?;
                            if sf == false && n != false {
                                return None;
                            }
                            let d = rd.load();
                            let n = rn.load();
                            let datasize = if sf { 64 } else { 32 };
                            let (op, setflags) = match (opc_1, opc_0) {
                                (false, false) => (LogicalOp::AND, false),
                                (false, true) => (LogicalOp::ORR, false),
                                (true, false) => (LogicalOp::EOR, false),
                                (true, true) => (LogicalOp::AND, true),
                            };
                            Some(Self {
                                raw,
                                d,
                                datasize,
                                imm: ::bitvec::vec::BitVec::<
                                    usize,
                                    ::bitvec::order::Lsb0,
                                >::repeat(0 != 0, 64),
                                n,
                                op,
                                setflags,
                            })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "not yet implemented: {0}",
                                        format_args!(
                                            "****THIS HAS A FAKE DECODE!!!!******integer_logical_immediate semantics",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                }
                pub mod pc_rel {
                    use crate::insns::{a64, disas};
                    use crate::Arm;
                    use bitvec::prelude::*;
                    use zinq::insn::semantics::*;
                    use zinq::insn::syntax::Decodable;
                    use zinq::insn::Instruction;
                    pub struct Adr {
                        raw: u32,
                        d: usize,
                        imm: BitVec,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Adr {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field3_finish(
                                f,
                                "Adr",
                                "raw",
                                &self.raw,
                                "d",
                                &self.d,
                                "imm",
                                &&self.imm,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Adr {
                        #[inline]
                        fn clone(&self) -> Adr {
                            Adr {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Adr {
                        const FIXEDBITS: a64::InsnSize = 0b00010000000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b10011111000000000000000000000000;
                    }
                    impl Instruction<Arm> for Adr {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                            let d = bits.get(0..5)?.load();
                            let immhi = bits.get(5..24)?;
                            let immlo = bits.get(29..31)?;
                            let mut imm = BitVec::with_capacity(64);
                            imm.extend_from_bitslice(immlo);
                            imm.extend_from_bitslice(immhi);
                            imm.resize(64, *immhi.last().unwrap());
                            Some(Self { raw, d, imm })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                    pub struct Adrp {
                        raw: u32,
                        d: usize,
                        imm: BitVec,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Adrp {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field3_finish(
                                f,
                                "Adrp",
                                "raw",
                                &self.raw,
                                "d",
                                &self.d,
                                "imm",
                                &&self.imm,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Adrp {
                        #[inline]
                        fn clone(&self) -> Adrp {
                            Adrp {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                d: ::core::clone::Clone::clone(&self.d),
                                imm: ::core::clone::Clone::clone(&self.imm),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for Adrp {
                        const FIXEDBITS: a64::InsnSize = 0b10010000000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b10011111000000000000000000000000;
                    }
                    impl Instruction<Arm> for Adrp {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                            let d = bits.get(0..5)?.load();
                            let immhi = bits.get(5..24)?;
                            let immlo = bits.get(29..31)?;
                            let mut imm = BitVec::with_capacity(64);
                            imm.resize(12, false);
                            imm.extend_from_bitslice(immlo);
                            imm.extend_from_bitslice(immhi);
                            imm.resize(64, *immhi.last().unwrap());
                            Some(Self { raw, d, imm })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            ::core::panicking::panic("not yet implemented")
                        }
                    }
                }
            }
            pub mod reg {
                pub mod arithmetic_shift {
                    use crate::insns::helpers::*;
                    use crate::insns::{a64, disas};
                    use crate::Arm;
                    use bitvec::prelude::*;
                    use zinq::insn::semantics::*;
                    use zinq::insn::syntax::Decodable;
                    use zinq::insn::Instruction;
                    pub struct IntegerArithmeticAddSubShiftedreg {
                        raw: u32,
                        d: usize,
                        datasize: usize,
                        m: usize,
                        n: usize,
                        setflags: bool,
                        shift_amount: usize,
                        shift_type: ShiftType,
                        sub_op: bool,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for IntegerArithmeticAddSubShiftedreg {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let names: &'static _ = &[
                                "raw",
                                "d",
                                "datasize",
                                "m",
                                "n",
                                "setflags",
                                "shift_amount",
                                "shift_type",
                                "sub_op",
                            ];
                            let values: &[&dyn ::core::fmt::Debug] = &[
                                &self.raw,
                                &self.d,
                                &self.datasize,
                                &self.m,
                                &self.n,
                                &self.setflags,
                                &self.shift_amount,
                                &self.shift_type,
                                &&self.sub_op,
                            ];
                            ::core::fmt::Formatter::debug_struct_fields_finish(
                                f,
                                "IntegerArithmeticAddSubShiftedreg",
                                names,
                                values,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for IntegerArithmeticAddSubShiftedreg {
                        #[inline]
                        fn clone(&self) -> IntegerArithmeticAddSubShiftedreg {
                            IntegerArithmeticAddSubShiftedreg {
                                raw: ::core::clone::Clone::clone(&self.raw),
                                d: ::core::clone::Clone::clone(&self.d),
                                datasize: ::core::clone::Clone::clone(&self.datasize),
                                m: ::core::clone::Clone::clone(&self.m),
                                n: ::core::clone::Clone::clone(&self.n),
                                setflags: ::core::clone::Clone::clone(&self.setflags),
                                shift_amount: ::core::clone::Clone::clone(
                                    &self.shift_amount,
                                ),
                                shift_type: ::core::clone::Clone::clone(&self.shift_type),
                                sub_op: ::core::clone::Clone::clone(&self.sub_op),
                            }
                        }
                    }
                    impl Decodable<a64::InsnSize> for IntegerArithmeticAddSubShiftedreg {
                        const FIXEDBITS: a64::InsnSize = 0b00001011000000000000000000000000;
                        const FIXEDMASK: a64::InsnSize = 0b00011111001000000000000000000000;
                    }
                    impl Instruction<Arm> for IntegerArithmeticAddSubShiftedreg {
                        type InsnSize = a64::InsnSize;
                        fn decode(bits: &BitSlice) -> Option<Self> {
                            let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                            let rd = bits.get(0..5)?;
                            let rn = bits.get(5..10)?;
                            let imm6 = bits.get(10..16)?;
                            let rm = bits.get(16..21)?;
                            let shift_0 = *bits.get(22)?;
                            let shift_1 = *bits.get(23)?;
                            let s = *bits.get(29)?;
                            let op = *bits.get(30)?;
                            let sf = *bits.get(31)?;
                            if (shift_1, shift_0) == (true, true) {
                                return None;
                            }
                            if sf == false && imm6[5] == true {
                                return None;
                            }
                            Some(Self {
                                raw,
                                d: rd.load(),
                                datasize: if sf { 64 } else { 32 },
                                m: rm.load(),
                                n: rn.load(),
                                setflags: s,
                                shift_amount: imm6.load(),
                                shift_type: ShiftType::from((shift_1, shift_0)),
                                sub_op: op,
                            })
                        }
                        fn assemble(&self) -> a64::InsnSize {
                            self.raw
                        }
                        fn disassemble(&self, proc: &Arm) -> String {
                            disas::a64(self.raw, proc)
                        }
                        fn size(&self) -> usize {
                            4
                        }
                        fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "not yet implemented: {0}",
                                        format_args!(
                                            "integer_arithmetic_add_sub_shiftedreg semantics",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
        pub mod load_store {
            mod memory_single_general_immediate_signed_post_index {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct MemorySingleGeneralImmediateSignedPostIndex {
                    raw: u32,
                    datasize: usize,
                    memop: MemOp,
                    n: usize,
                    nontemporal: bool,
                    offset: BitVec,
                    postindex: bool,
                    regsize: usize,
                    rt_unknown: bool,
                    is_signed: bool,
                    t: usize,
                    tagchecked: bool,
                    wb_unknown: bool,
                    wback: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for MemorySingleGeneralImmediateSignedPostIndex {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "raw",
                            "datasize",
                            "memop",
                            "n",
                            "nontemporal",
                            "offset",
                            "postindex",
                            "regsize",
                            "rt_unknown",
                            "is_signed",
                            "t",
                            "tagchecked",
                            "wb_unknown",
                            "wback",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.raw,
                            &self.datasize,
                            &self.memop,
                            &self.n,
                            &self.nontemporal,
                            &self.offset,
                            &self.postindex,
                            &self.regsize,
                            &self.rt_unknown,
                            &self.is_signed,
                            &self.t,
                            &self.tagchecked,
                            &self.wb_unknown,
                            &&self.wback,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "MemorySingleGeneralImmediateSignedPostIndex",
                            names,
                            values,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone
                for MemorySingleGeneralImmediateSignedPostIndex {
                    #[inline]
                    fn clone(&self) -> MemorySingleGeneralImmediateSignedPostIndex {
                        MemorySingleGeneralImmediateSignedPostIndex {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            datasize: ::core::clone::Clone::clone(&self.datasize),
                            memop: ::core::clone::Clone::clone(&self.memop),
                            n: ::core::clone::Clone::clone(&self.n),
                            nontemporal: ::core::clone::Clone::clone(&self.nontemporal),
                            offset: ::core::clone::Clone::clone(&self.offset),
                            postindex: ::core::clone::Clone::clone(&self.postindex),
                            regsize: ::core::clone::Clone::clone(&self.regsize),
                            rt_unknown: ::core::clone::Clone::clone(&self.rt_unknown),
                            is_signed: ::core::clone::Clone::clone(&self.is_signed),
                            t: ::core::clone::Clone::clone(&self.t),
                            tagchecked: ::core::clone::Clone::clone(&self.tagchecked),
                            wb_unknown: ::core::clone::Clone::clone(&self.wb_unknown),
                            wback: ::core::clone::Clone::clone(&self.wback),
                        }
                    }
                }
                impl Decodable<a64::InsnSize>
                for MemorySingleGeneralImmediateSignedPostIndex {
                    const FIXEDBITS: a64::InsnSize = 0b00111000000000000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b00111110000000000000000000000000;
                }
                impl Instruction<Arm> for MemorySingleGeneralImmediateSignedPostIndex {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "memory_single_general_immediate_signed_post_idx decode; was a handwrite due to multipl slice options",
                                    ),
                                ),
                            );
                        }
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "memory_single_general_immediate_signed_post_idx semantics",
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
            pub use memory_single_general_immediate_signed_post_index::MemorySingleGeneralImmediateSignedPostIndex;
            mod memory_pair_general_post_idx {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct MemoryPairGeneralPostIdx {
                    raw: u32,
                    datasize: usize,
                    memop: MemOp,
                    n: usize,
                    nontemporal: bool,
                    offset: BitVec,
                    postindex: bool,
                    rt_unknown: bool,
                    is_signed: bool,
                    t: usize,
                    t2: usize,
                    tagchecked: bool,
                    wb_unknown: bool,
                    wback: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for MemoryPairGeneralPostIdx {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "raw",
                            "datasize",
                            "memop",
                            "n",
                            "nontemporal",
                            "offset",
                            "postindex",
                            "rt_unknown",
                            "is_signed",
                            "t",
                            "t2",
                            "tagchecked",
                            "wb_unknown",
                            "wback",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.raw,
                            &self.datasize,
                            &self.memop,
                            &self.n,
                            &self.nontemporal,
                            &self.offset,
                            &self.postindex,
                            &self.rt_unknown,
                            &self.is_signed,
                            &self.t,
                            &self.t2,
                            &self.tagchecked,
                            &self.wb_unknown,
                            &&self.wback,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "MemoryPairGeneralPostIdx",
                            names,
                            values,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for MemoryPairGeneralPostIdx {
                    #[inline]
                    fn clone(&self) -> MemoryPairGeneralPostIdx {
                        MemoryPairGeneralPostIdx {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            datasize: ::core::clone::Clone::clone(&self.datasize),
                            memop: ::core::clone::Clone::clone(&self.memop),
                            n: ::core::clone::Clone::clone(&self.n),
                            nontemporal: ::core::clone::Clone::clone(&self.nontemporal),
                            offset: ::core::clone::Clone::clone(&self.offset),
                            postindex: ::core::clone::Clone::clone(&self.postindex),
                            rt_unknown: ::core::clone::Clone::clone(&self.rt_unknown),
                            is_signed: ::core::clone::Clone::clone(&self.is_signed),
                            t: ::core::clone::Clone::clone(&self.t),
                            t2: ::core::clone::Clone::clone(&self.t2),
                            tagchecked: ::core::clone::Clone::clone(&self.tagchecked),
                            wb_unknown: ::core::clone::Clone::clone(&self.wb_unknown),
                            wback: ::core::clone::Clone::clone(&self.wback),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for MemoryPairGeneralPostIdx {
                    const FIXEDBITS: a64::InsnSize = 0b00101000000000000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b00111110000000000000000000000000;
                }
                impl Instruction<Arm> for MemoryPairGeneralPostIdx {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let rt = bits.get(0..5)?;
                        let rn = bits.get(5..10)?;
                        let rt2 = bits.get(10..15)?;
                        let imm7 = bits.get(15..22)?;
                        let l = *bits.get(22)?;
                        let opc_0 = *bits.get(30)?;
                        let opc_1 = *bits.get(31)?;
                        let memop = if l == true { MemOp::LOAD } else { MemOp::STORE };
                        Some(Self {
                            raw,
                            datasize: 64,
                            memop,
                            n: rn.load(),
                            nontemporal: false,
                            offset: ::bitvec::vec::BitVec::from_bitslice({
                                const BITS: usize = {
                                    const LEN: usize = 0 + 1;
                                    LEN
                                };
                                &{
                                    const ELTS: usize = ::bitvec::mem::elts::<
                                        usize,
                                    >({
                                        const LEN: usize = 0 + 1;
                                        LEN
                                    });
                                    type This = ::bitvec::array::BitArray<[usize; ELTS], Lsb0>;
                                    This::new({
                                        const LEN: usize = ::bitvec::mem::elts::<
                                            usize,
                                        >({
                                            const LEN: usize = 0 + 1;
                                            LEN
                                        });
                                        let out: [usize; LEN];
                                        #[cfg(target_pointer_width = "64")]
                                        {
                                            out = [
                                                {
                                                    const ELEM: u64 = u64::from_le_bytes([
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            64 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                        ::bitvec::macros::internal::u8_from_le_bits(
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                            0 != 0,
                                                        ),
                                                    ]);
                                                    ::bitvec::mem::BitElement::<usize>::new(ELEM as usize).elem
                                                },
                                            ];
                                        }
                                        out
                                    })
                                }[..BITS]
                            }),
                            postindex: false,
                            rt_unknown: false,
                            is_signed: false,
                            t: rt.load(),
                            t2: rt2.load(),
                            tagchecked: true,
                            wb_unknown: false,
                            wback: true,
                        })
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "****FAKE DECODE!!!****memory_pair_general_post_idx semantics",
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
            pub use memory_pair_general_post_idx::MemoryPairGeneralPostIdx;
            mod memory_literal_general {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct MemoryLiteralGeneral {
                    raw: u32,
                    memop: MemOp,
                    nontemporal: bool,
                    offset: BitVec,
                    is_signed: bool,
                    size: usize,
                    t: usize,
                    tagchecked: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for MemoryLiteralGeneral {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "raw",
                            "memop",
                            "nontemporal",
                            "offset",
                            "is_signed",
                            "size",
                            "t",
                            "tagchecked",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.raw,
                            &self.memop,
                            &self.nontemporal,
                            &self.offset,
                            &self.is_signed,
                            &self.size,
                            &self.t,
                            &&self.tagchecked,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "MemoryLiteralGeneral",
                            names,
                            values,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for MemoryLiteralGeneral {
                    #[inline]
                    fn clone(&self) -> MemoryLiteralGeneral {
                        MemoryLiteralGeneral {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            memop: ::core::clone::Clone::clone(&self.memop),
                            nontemporal: ::core::clone::Clone::clone(&self.nontemporal),
                            offset: ::core::clone::Clone::clone(&self.offset),
                            is_signed: ::core::clone::Clone::clone(&self.is_signed),
                            size: ::core::clone::Clone::clone(&self.size),
                            t: ::core::clone::Clone::clone(&self.t),
                            tagchecked: ::core::clone::Clone::clone(&self.tagchecked),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for MemoryLiteralGeneral {
                    const FIXEDBITS: a64::InsnSize = 0b00011000000000000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b00111111000000000000000000000000;
                }
                impl Instruction<Arm> for MemoryLiteralGeneral {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let rt = bits.get(0..5)?;
                        let imm19 = bits.get(5..24)?;
                        let opc_0 = *bits.get(30)?;
                        let opc_1 = *bits.get(31)?;
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("memory_literal_general decode"),
                                ),
                            );
                        }
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("memory_literal_general semantics"),
                                ),
                            );
                        }
                    }
                }
            }
            pub use memory_literal_general::MemoryLiteralGeneral;
        }
        pub mod branch_exc_sys {
            mod branch_unconditional_immediate {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct BranchUnconditionalImmediate {
                    raw: u32,
                    branch_type: BranchType,
                    offset: BitVec,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for BranchUnconditionalImmediate {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "BranchUnconditionalImmediate",
                            "raw",
                            &self.raw,
                            "branch_type",
                            &self.branch_type,
                            "offset",
                            &&self.offset,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for BranchUnconditionalImmediate {
                    #[inline]
                    fn clone(&self) -> BranchUnconditionalImmediate {
                        BranchUnconditionalImmediate {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            branch_type: ::core::clone::Clone::clone(&self.branch_type),
                            offset: ::core::clone::Clone::clone(&self.offset),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for BranchUnconditionalImmediate {
                    const FIXEDBITS: a64::InsnSize = 0b00010100000000000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b01111100000000000000000000000000;
                }
                impl Instruction<Arm> for BranchUnconditionalImmediate {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let imm26 = bits.get(0..26)?;
                        let op = *bits.get(31)?;
                        let branch_type = if op {
                            BranchType::DIRCALL
                        } else {
                            BranchType::DIR
                        };
                        let mut offset = BitVec::with_capacity(64);
                        offset
                            .extend_from_bitslice({
                                &{
                                    {
                                        use ::bitvec::macros::internal::core;
                                        type Mem = <usize as ::bitvec::store::BitStore>::Mem;
                                        const ELTS: usize = ::bitvec::mem::elts::<usize>(2);
                                        const ELEM: Mem = {
                                            type Mem = <usize as ::bitvec::store::BitStore>::Mem;
                                            if 0 != 0 {
                                                <Mem as ::bitvec::mem::BitRegister>::ALL
                                            } else {
                                                <Mem as ::bitvec::macros::internal::funty::Integral>::ZERO
                                            }
                                        };
                                        const DATA: [Mem; ELTS] = [ELEM; ELTS];
                                        type This = ::bitvec::array::BitArray<
                                            [usize; ELTS],
                                            ::bitvec::order::Lsb0,
                                        >;
                                        unsafe { core::mem::transmute::<_, This>(DATA) }
                                    }
                                }[..2]
                            });
                        offset.extend_from_bitslice(imm26);
                        offset.resize(64, *imm26.last().unwrap());
                        Some(Self { raw, branch_type, offset })
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("branch_unconditional_immediate semantics"),
                                ),
                            );
                        }
                    }
                }
            }
            pub use branch_unconditional_immediate::BranchUnconditionalImmediate;
            mod branch_conditional_cond {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct BranchConditionalCond {
                    raw: u32,
                    condition: CondCode,
                    offset: BitVec,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for BranchConditionalCond {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "BranchConditionalCond",
                            "raw",
                            &self.raw,
                            "condition",
                            &self.condition,
                            "offset",
                            &&self.offset,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for BranchConditionalCond {
                    #[inline]
                    fn clone(&self) -> BranchConditionalCond {
                        BranchConditionalCond {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            condition: ::core::clone::Clone::clone(&self.condition),
                            offset: ::core::clone::Clone::clone(&self.offset),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for BranchConditionalCond {
                    const FIXEDBITS: a64::InsnSize = 0b01010100000000000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b11111111000000000000000000010000;
                }
                impl Instruction<Arm> for BranchConditionalCond {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let cond = bits.get(0..4)?;
                        let imm19 = bits.get(5..24)?;
                        let mut offset = BitVec::with_capacity(64);
                        offset
                            .extend_from_bitslice({
                                &{
                                    {
                                        use ::bitvec::macros::internal::core;
                                        type Mem = <usize as ::bitvec::store::BitStore>::Mem;
                                        const ELTS: usize = ::bitvec::mem::elts::<usize>(2);
                                        const ELEM: Mem = {
                                            type Mem = <usize as ::bitvec::store::BitStore>::Mem;
                                            if 0 != 0 {
                                                <Mem as ::bitvec::mem::BitRegister>::ALL
                                            } else {
                                                <Mem as ::bitvec::macros::internal::funty::Integral>::ZERO
                                            }
                                        };
                                        const DATA: [Mem; ELTS] = [ELEM; ELTS];
                                        type This = ::bitvec::array::BitArray<
                                            [usize; ELTS],
                                            ::bitvec::order::Lsb0,
                                        >;
                                        unsafe { core::mem::transmute::<_, This>(DATA) }
                                    }
                                }[..2]
                            });
                        offset.extend_from_bitslice(imm19);
                        offset.resize(64, imm19[18]);
                        Some(Self {
                            raw,
                            condition: CondCode::from(cond.load::<u8>()),
                            offset,
                        })
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("branch_conditional_cond semantics"),
                                ),
                            );
                        }
                    }
                }
            }
            pub use branch_conditional_cond::BranchConditionalCond;
            mod branch_unconditional_register {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct BranchUnconditionalRegister {
                    raw: u32,
                    branch_type: BranchType,
                    m: usize,
                    n: usize,
                    pac: bool,
                    source_is_sp: bool,
                    use_key_a: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for BranchUnconditionalRegister {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "raw",
                            "branch_type",
                            "m",
                            "n",
                            "pac",
                            "source_is_sp",
                            "use_key_a",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.raw,
                            &self.branch_type,
                            &self.m,
                            &self.n,
                            &self.pac,
                            &self.source_is_sp,
                            &&self.use_key_a,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "BranchUnconditionalRegister",
                            names,
                            values,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for BranchUnconditionalRegister {
                    #[inline]
                    fn clone(&self) -> BranchUnconditionalRegister {
                        BranchUnconditionalRegister {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            branch_type: ::core::clone::Clone::clone(&self.branch_type),
                            m: ::core::clone::Clone::clone(&self.m),
                            n: ::core::clone::Clone::clone(&self.n),
                            pac: ::core::clone::Clone::clone(&self.pac),
                            source_is_sp: ::core::clone::Clone::clone(
                                &self.source_is_sp,
                            ),
                            use_key_a: ::core::clone::Clone::clone(&self.use_key_a),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for BranchUnconditionalRegister {
                    const FIXEDBITS: a64::InsnSize = 0b11010110000111110000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b11111110100111111111000000000000;
                }
                impl Instruction<Arm> for BranchUnconditionalRegister {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let rm = bits.get(0..5)?;
                        let rn = bits.get(5..10)?;
                        let m = *bits.get(10)?;
                        let a = *bits.get(11)?;
                        let op_0 = *bits.get(21)?;
                        let op_1 = *bits.get(22)?;
                        let z = *bits.get(24)?;
                        let rm = rm.load();
                        let pac = a == true;
                        let use_key_a = m == false;
                        let source_is_sp = z == true && rm == 31;
                        Some(Self {
                            raw,
                            branch_type: BranchType::DIR,
                            m: rm,
                            n: rn.load(),
                            pac,
                            source_is_sp,
                            use_key_a,
                        })
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "***FAKE DECODE***branch_unconditional_register semantics",
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
            pub use branch_unconditional_register::BranchUnconditionalRegister;
            mod system_hints {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct SystemHints {
                    raw: u32,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for SystemHints {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "SystemHints",
                            "raw",
                            &&self.raw,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for SystemHints {
                    #[inline]
                    fn clone(&self) -> SystemHints {
                        SystemHints {
                            raw: ::core::clone::Clone::clone(&self.raw),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for SystemHints {
                    const FIXEDBITS: a64::InsnSize = 0b11010101000000110010000000011111;
                    const FIXEDMASK: a64::InsnSize = 0b11111111111111111111000000011111;
                }
                impl Instruction<Arm> for SystemHints {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let op2 = bits.get(5..8)?;
                        let crm = bits.get(8..12)?;
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("system_hints decode"),
                                ),
                            );
                        }
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!("system_hints semantics"),
                                ),
                            );
                        }
                    }
                }
            }
            pub use system_hints::SystemHints;
            mod system_register_system {
                use crate::insns::helpers::*;
                use crate::insns::{a64, disas};
                use crate::Arm;
                use bitvec::prelude::*;
                use zinq::insn::semantics::*;
                use zinq::insn::syntax::Decodable;
                use zinq::insn::Instruction;
                pub struct SystemRegisterSystem {
                    raw: u32,
                    read: bool,
                    sys_crm: usize,
                    sys_crn: usize,
                    sys_op0: usize,
                    sys_op1: usize,
                    sys_op2: usize,
                    t: usize,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for SystemRegisterSystem {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "raw",
                            "read",
                            "sys_crm",
                            "sys_crn",
                            "sys_op0",
                            "sys_op1",
                            "sys_op2",
                            "t",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.raw,
                            &self.read,
                            &self.sys_crm,
                            &self.sys_crn,
                            &self.sys_op0,
                            &self.sys_op1,
                            &self.sys_op2,
                            &&self.t,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "SystemRegisterSystem",
                            names,
                            values,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for SystemRegisterSystem {
                    #[inline]
                    fn clone(&self) -> SystemRegisterSystem {
                        SystemRegisterSystem {
                            raw: ::core::clone::Clone::clone(&self.raw),
                            read: ::core::clone::Clone::clone(&self.read),
                            sys_crm: ::core::clone::Clone::clone(&self.sys_crm),
                            sys_crn: ::core::clone::Clone::clone(&self.sys_crn),
                            sys_op0: ::core::clone::Clone::clone(&self.sys_op0),
                            sys_op1: ::core::clone::Clone::clone(&self.sys_op1),
                            sys_op2: ::core::clone::Clone::clone(&self.sys_op2),
                            t: ::core::clone::Clone::clone(&self.t),
                        }
                    }
                }
                impl Decodable<a64::InsnSize> for SystemRegisterSystem {
                    const FIXEDBITS: a64::InsnSize = 0b11010101000100000000000000000000;
                    const FIXEDMASK: a64::InsnSize = 0b11111111110100000000000000000000;
                }
                impl Instruction<Arm> for SystemRegisterSystem {
                    type InsnSize = a64::InsnSize;
                    fn decode(bits: &BitSlice) -> Option<Self> {
                        let raw = bits.get(0..32)?.load::<a64::InsnSize>();
                        let rt = bits.get(0..5)?;
                        let op2_0 = *bits.get(5)?;
                        let op2_1 = *bits.get(6)?;
                        let op2_2 = *bits.get(7)?;
                        let crm = bits.get(8..12)?;
                        let crn = bits.get(12..16)?;
                        let op1_0 = *bits.get(16)?;
                        let op1_1 = *bits.get(17)?;
                        let op1_2 = *bits.get(18)?;
                        let o0 = *bits.get(19)?;
                        let l = *bits.get(21)?;
                        Some(Self {
                            raw,
                            read: false,
                            sys_crm: 0,
                            sys_crn: 0,
                            sys_op0: 0,
                            sys_op1: 0,
                            sys_op2: 0,
                            t: 0,
                        })
                    }
                    fn assemble(&self) -> a64::InsnSize {
                        self.raw
                    }
                    fn disassemble(&self, proc: &Arm) -> String {
                        disas::a64(self.raw, proc)
                    }
                    fn size(&self) -> usize {
                        4
                    }
                    fn semantics<'p>(&self, proc: &'p Arm, code: &mut IrCtx<'p>) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "not yet implemented: {0}",
                                    format_args!(
                                        "***FAKE!!!***system_register_system semantics",
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
            pub use system_register_system::SystemRegisterSystem;
        }
    }
    impl ArmInstruction {
        pub fn fake_decode(bits: &bitvec::prelude::BitSlice) -> Option<String> {
            let raw_32 = bitvec::field::BitField::load::<u32>(bits.get(0..32).unwrap());
            if <a64::data_proc::imm::arithmetic::Add as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Add as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmArithmeticAdd"));
            }
            if <a64::data_proc::imm::arithmetic::Adds as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Adds as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmArithmeticAdds"));
            }
            if <a64::data_proc::imm::arithmetic::Sub as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Sub as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmArithmeticSub"));
            }
            if <a64::data_proc::imm::arithmetic::Subs as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Subs as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmArithmeticSubs"));
            }
            if <a64::data_proc::imm::logical::IntegerLogicalImmediate as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::logical::IntegerLogicalImmediate as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(
                    String::from("A64DataProcImmLogicalIntegerLogicalImmediate"),
                );
            }
            if <a64::data_proc::imm::pc_rel::Adr as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::pc_rel::Adr as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmPcRelAdr"));
            }
            if <a64::data_proc::imm::pc_rel::Adrp as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::pc_rel::Adrp as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64DataProcImmPcRelAdrp"));
            }
            if <a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(
                    String::from(
                        "A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg",
                    ),
                );
            }
            if <a64::branch_exc_sys::BranchUnconditionalImmediate as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchUnconditionalImmediate as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64BranchExcSysBranchUnconditionalImmediate"));
            }
            if <a64::branch_exc_sys::BranchConditionalCond as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchConditionalCond as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64BranchExcSysBranchConditionalCond"));
            }
            if <a64::branch_exc_sys::BranchUnconditionalRegister as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchUnconditionalRegister as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64BranchExcSysBranchUnconditionalRegister"));
            }
            if <a64::load_store::MemoryPairGeneralPostIdx as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemoryPairGeneralPostIdx as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64LoadStoreMemoryPairGeneralPostIdx"));
            }
            if <a64::load_store::MemoryLiteralGeneral as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemoryLiteralGeneral as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64LoadStoreMemoryLiteralGeneral"));
            }
            if <a64::load_store::MemorySingleGeneralImmediateSignedPostIndex as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemorySingleGeneralImmediateSignedPostIndex as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(
                    String::from(
                        "A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex",
                    ),
                );
            }
            if <a64::branch_exc_sys::SystemHints as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::SystemHints as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64BranchExcSysSystemHints"));
            }
            if <a64::branch_exc_sys::SystemRegisterSystem as zinq::insn::syntax::Decodable<
                u32,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::SystemRegisterSystem as zinq::insn::syntax::Decodable<
                    u32,
                >>::FIXEDBITS
            {
                return Some(String::from("A64BranchExcSysSystemRegisterSystem"));
            }
            return None;
        }
    }
    pub enum ArmInstruction {
        A64DataProcImmArithmeticAdd(a64::data_proc::imm::arithmetic::Add),
        A64DataProcImmArithmeticAdds(a64::data_proc::imm::arithmetic::Adds),
        A64DataProcImmArithmeticSub(a64::data_proc::imm::arithmetic::Sub),
        A64DataProcImmArithmeticSubs(a64::data_proc::imm::arithmetic::Subs),
        A64DataProcImmLogicalIntegerLogicalImmediate(
            a64::data_proc::imm::logical::IntegerLogicalImmediate,
        ),
        A64DataProcImmPcRelAdr(a64::data_proc::imm::pc_rel::Adr),
        A64DataProcImmPcRelAdrp(a64::data_proc::imm::pc_rel::Adrp),
        A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
            a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg,
        ),
        A64BranchExcSysBranchUnconditionalImmediate(
            a64::branch_exc_sys::BranchUnconditionalImmediate,
        ),
        A64BranchExcSysBranchConditionalCond(a64::branch_exc_sys::BranchConditionalCond),
        A64BranchExcSysBranchUnconditionalRegister(
            a64::branch_exc_sys::BranchUnconditionalRegister,
        ),
        A64LoadStoreMemoryPairGeneralPostIdx(a64::load_store::MemoryPairGeneralPostIdx),
        A64LoadStoreMemoryLiteralGeneral(a64::load_store::MemoryLiteralGeneral),
        A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
            a64::load_store::MemorySingleGeneralImmediateSignedPostIndex,
        ),
        A64BranchExcSysSystemHints(a64::branch_exc_sys::SystemHints),
        A64BranchExcSysSystemRegisterSystem(a64::branch_exc_sys::SystemRegisterSystem),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArmInstruction {
        #[inline]
        fn clone(&self) -> ArmInstruction {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(__self_0) => {
                    ArmInstruction::A64DataProcImmArithmeticAdd(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticAdds(__self_0) => {
                    ArmInstruction::A64DataProcImmArithmeticAdds(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticSub(__self_0) => {
                    ArmInstruction::A64DataProcImmArithmeticSub(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticSubs(__self_0) => {
                    ArmInstruction::A64DataProcImmArithmeticSubs(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(
                    __self_0,
                ) => {
                    ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmPcRelAdr(__self_0) => {
                    ArmInstruction::A64DataProcImmPcRelAdr(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcImmPcRelAdrp(__self_0) => {
                    ArmInstruction::A64DataProcImmPcRelAdrp(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    __self_0,
                ) => {
                    ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(__self_0) => {
                    ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(__self_0) => {
                    ArmInstruction::A64BranchExcSysBranchConditionalCond(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(__self_0) => {
                    ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(__self_0) => {
                    ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(__self_0) => {
                    ArmInstruction::A64LoadStoreMemoryLiteralGeneral(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    __self_0,
                ) => {
                    ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64BranchExcSysSystemHints(__self_0) => {
                    ArmInstruction::A64BranchExcSysSystemHints(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(__self_0) => {
                    ArmInstruction::A64BranchExcSysSystemRegisterSystem(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArmInstruction {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmArithmeticAdd",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticAdds(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmArithmeticAdds",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticSub(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmArithmeticSub",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmArithmeticSubs(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmArithmeticSubs",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(
                    __self_0,
                ) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmLogicalIntegerLogicalImmediate",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmPcRelAdr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmPcRelAdr",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcImmPcRelAdrp(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcImmPcRelAdrp",
                        &__self_0,
                    )
                }
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    __self_0,
                ) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg",
                        &__self_0,
                    )
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64BranchExcSysBranchUnconditionalImmediate",
                        &__self_0,
                    )
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64BranchExcSysBranchConditionalCond",
                        &__self_0,
                    )
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64BranchExcSysBranchUnconditionalRegister",
                        &__self_0,
                    )
                }
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64LoadStoreMemoryPairGeneralPostIdx",
                        &__self_0,
                    )
                }
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64LoadStoreMemoryLiteralGeneral",
                        &__self_0,
                    )
                }
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    __self_0,
                ) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex",
                        &__self_0,
                    )
                }
                ArmInstruction::A64BranchExcSysSystemHints(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64BranchExcSysSystemHints",
                        &__self_0,
                    )
                }
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "A64BranchExcSysSystemRegisterSystem",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl zinq::insn::Instruction<Arm> for ArmInstruction {
        type InsnSize = u32;
        fn decode(bits: &bitvec::prelude::BitSlice) -> Option<Self> {
            let raw_32 = bitvec::field::BitField::load::<
                Self::InsnSize,
            >(bits.get(0..32).unwrap());
            if <a64::data_proc::imm::arithmetic::Add as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Add as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::arithmetic::Add as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmArithmeticAdd(insn),
                    ));
            }
            if <a64::data_proc::imm::arithmetic::Adds as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Adds as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::arithmetic::Adds as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmArithmeticAdds(insn),
                    ));
            }
            if <a64::data_proc::imm::arithmetic::Sub as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Sub as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::arithmetic::Sub as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmArithmeticSub(insn),
                    ));
            }
            if <a64::data_proc::imm::arithmetic::Subs as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::arithmetic::Subs as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::arithmetic::Subs as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmArithmeticSubs(insn),
                    ));
            }
            if <a64::data_proc::imm::logical::IntegerLogicalImmediate as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::logical::IntegerLogicalImmediate as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::logical::IntegerLogicalImmediate as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(
                            insn,
                        ),
                    ));
            }
            if <a64::data_proc::imm::pc_rel::Adr as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::pc_rel::Adr as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::pc_rel::Adr as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(ArmInstruction::A64DataProcImmPcRelAdr(insn)));
            }
            if <a64::data_proc::imm::pc_rel::Adrp as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::imm::pc_rel::Adrp as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::imm::pc_rel::Adrp as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcImmPcRelAdrp(insn),
                    ));
            }
            if <a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::data_proc::reg::arithmetic_shift::IntegerArithmeticAddSubShiftedreg as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                            insn,
                        ),
                    ));
            }
            if <a64::branch_exc_sys::BranchUnconditionalImmediate as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchUnconditionalImmediate as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::branch_exc_sys::BranchUnconditionalImmediate as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(insn),
                    ));
            }
            if <a64::branch_exc_sys::BranchConditionalCond as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchConditionalCond as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::branch_exc_sys::BranchConditionalCond as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64BranchExcSysBranchConditionalCond(insn),
                    ));
            }
            if <a64::branch_exc_sys::BranchUnconditionalRegister as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::BranchUnconditionalRegister as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::branch_exc_sys::BranchUnconditionalRegister as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(insn),
                    ));
            }
            if <a64::load_store::MemoryPairGeneralPostIdx as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemoryPairGeneralPostIdx as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::load_store::MemoryPairGeneralPostIdx as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(insn),
                    ));
            }
            if <a64::load_store::MemoryLiteralGeneral as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemoryLiteralGeneral as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::load_store::MemoryLiteralGeneral as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64LoadStoreMemoryLiteralGeneral(insn),
                    ));
            }
            if <a64::load_store::MemorySingleGeneralImmediateSignedPostIndex as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::load_store::MemorySingleGeneralImmediateSignedPostIndex as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::load_store::MemorySingleGeneralImmediateSignedPostIndex as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                            insn,
                        ),
                    ));
            }
            if <a64::branch_exc_sys::SystemHints as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::SystemHints as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::branch_exc_sys::SystemHints as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64BranchExcSysSystemHints(insn),
                    ));
            }
            if <a64::branch_exc_sys::SystemRegisterSystem as zinq::insn::syntax::Decodable<
                Self::InsnSize,
            >>::FIXEDMASK & raw_32
                == <a64::branch_exc_sys::SystemRegisterSystem as zinq::insn::syntax::Decodable<
                    Self::InsnSize,
                >>::FIXEDBITS
            {
                return <a64::branch_exc_sys::SystemRegisterSystem as zinq::insn::Instruction<
                    Arm,
                >>::decode(bits)
                    .and_then(|insn| Some(
                        ArmInstruction::A64BranchExcSysSystemRegisterSystem(insn),
                    ));
            }
            return None;
        }
        fn assemble(&self) -> Self::InsnSize {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(i) => i.assemble(),
                ArmInstruction::A64DataProcImmArithmeticAdds(i) => i.assemble(),
                ArmInstruction::A64DataProcImmArithmeticSub(i) => i.assemble(),
                ArmInstruction::A64DataProcImmArithmeticSubs(i) => i.assemble(),
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(i) => {
                    i.assemble()
                }
                ArmInstruction::A64DataProcImmPcRelAdr(i) => i.assemble(),
                ArmInstruction::A64DataProcImmPcRelAdrp(i) => i.assemble(),
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    i,
                ) => i.assemble(),
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(i) => {
                    i.assemble()
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(i) => i.assemble(),
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(i) => {
                    i.assemble()
                }
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(i) => i.assemble(),
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(i) => i.assemble(),
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    i,
                ) => i.assemble(),
                ArmInstruction::A64BranchExcSysSystemHints(i) => i.assemble(),
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(i) => i.assemble(),
            }
        }
        fn disassemble(&self, proc: &Arm) -> String {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcImmArithmeticAdds(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcImmArithmeticSub(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcImmArithmeticSubs(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64DataProcImmPcRelAdr(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcImmPcRelAdrp(i) => i.disassemble(proc),
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    i,
                ) => i.disassemble(proc),
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(i) => {
                    i.disassemble(proc)
                }
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    i,
                ) => i.disassemble(proc),
                ArmInstruction::A64BranchExcSysSystemHints(i) => i.disassemble(proc),
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(i) => {
                    i.disassemble(proc)
                }
            }
        }
        fn size(&self) -> usize {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(i) => i.size(),
                ArmInstruction::A64DataProcImmArithmeticAdds(i) => i.size(),
                ArmInstruction::A64DataProcImmArithmeticSub(i) => i.size(),
                ArmInstruction::A64DataProcImmArithmeticSubs(i) => i.size(),
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(i) => {
                    i.size()
                }
                ArmInstruction::A64DataProcImmPcRelAdr(i) => i.size(),
                ArmInstruction::A64DataProcImmPcRelAdrp(i) => i.size(),
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    i,
                ) => i.size(),
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(i) => {
                    i.size()
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(i) => i.size(),
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(i) => i.size(),
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(i) => i.size(),
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(i) => i.size(),
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    i,
                ) => i.size(),
                ArmInstruction::A64BranchExcSysSystemHints(i) => i.size(),
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(i) => i.size(),
            }
        }
        fn semantics<'p>(
            &self,
            proc: &'p Arm,
            code: &mut zinq::insn::semantics::IrCtx<'p>,
        ) {
            match self {
                ArmInstruction::A64DataProcImmArithmeticAdd(i) => i.semantics(proc, code),
                ArmInstruction::A64DataProcImmArithmeticAdds(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64DataProcImmArithmeticSub(i) => i.semantics(proc, code),
                ArmInstruction::A64DataProcImmArithmeticSubs(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64DataProcImmLogicalIntegerLogicalImmediate(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64DataProcImmPcRelAdr(i) => i.semantics(proc, code),
                ArmInstruction::A64DataProcImmPcRelAdrp(i) => i.semantics(proc, code),
                ArmInstruction::A64DataProcRegArithmeticShiftIntegerArithmeticAddSubShiftedreg(
                    i,
                ) => i.semantics(proc, code),
                ArmInstruction::A64BranchExcSysBranchUnconditionalImmediate(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64BranchExcSysBranchConditionalCond(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64BranchExcSysBranchUnconditionalRegister(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64LoadStoreMemoryPairGeneralPostIdx(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64LoadStoreMemoryLiteralGeneral(i) => {
                    i.semantics(proc, code)
                }
                ArmInstruction::A64LoadStoreMemorySingleGeneralImmediateSignedPostIndex(
                    i,
                ) => i.semantics(proc, code),
                ArmInstruction::A64BranchExcSysSystemHints(i) => i.semantics(proc, code),
                ArmInstruction::A64BranchExcSysSystemRegisterSystem(i) => {
                    i.semantics(proc, code)
                }
            }
        }
    }
    impl zinq::insn::syntax::Decodable<u32> for ArmInstruction {
        const FIXEDBITS: u32 = 0;
        const FIXEDMASK: u32 = 0;
    }
}
use insns::ArmInstruction;
mod variants {
    use std::fmt;
    pub enum Version {
        Armv8p0a,
        Armv8p1a,
        Armv8p2a,
        Armv8p3a,
        Armv8p4a,
        Armv8p5a,
        Armv8p6a,
        Armv8p7a,
        Armv8p8a,
        Armv8p9a,
        Armv9p0a,
        Armv9p1a,
        Armv9p2a,
        Armv9p3a,
        Armv9p4a,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Version {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Version::Armv8p0a => "Armv8p0a",
                    Version::Armv8p1a => "Armv8p1a",
                    Version::Armv8p2a => "Armv8p2a",
                    Version::Armv8p3a => "Armv8p3a",
                    Version::Armv8p4a => "Armv8p4a",
                    Version::Armv8p5a => "Armv8p5a",
                    Version::Armv8p6a => "Armv8p6a",
                    Version::Armv8p7a => "Armv8p7a",
                    Version::Armv8p8a => "Armv8p8a",
                    Version::Armv8p9a => "Armv8p9a",
                    Version::Armv9p0a => "Armv9p0a",
                    Version::Armv9p1a => "Armv9p1a",
                    Version::Armv9p2a => "Armv9p2a",
                    Version::Armv9p3a => "Armv9p3a",
                    Version::Armv9p4a => "Armv9p4a",
                },
            )
        }
    }
    impl fmt::Display for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Version::*;
            match self {
                Armv8p0a => f.write_fmt(format_args!("Armv8.0-a")),
                Armv8p1a => f.write_fmt(format_args!("Armv8.1-a")),
                Armv8p2a => f.write_fmt(format_args!("Armv8.2-a")),
                Armv8p3a => f.write_fmt(format_args!("Armv8.3-a")),
                Armv8p4a => f.write_fmt(format_args!("Armv8.4-a")),
                Armv8p5a => f.write_fmt(format_args!("Armv8.5-a")),
                Armv8p6a => f.write_fmt(format_args!("Armv8.6-a")),
                Armv8p7a => f.write_fmt(format_args!("Armv8.7-a")),
                Armv8p8a => f.write_fmt(format_args!("Armv8.8-a")),
                Armv8p9a => f.write_fmt(format_args!("Armv8.9-a")),
                Armv9p0a => f.write_fmt(format_args!("Armv9.0-a")),
                Armv9p1a => f.write_fmt(format_args!("Armv9.1-a")),
                Armv9p2a => f.write_fmt(format_args!("Armv9.2-a")),
                Armv9p3a => f.write_fmt(format_args!("Armv9.3-a")),
                Armv9p4a => f.write_fmt(format_args!("Armv9.4-a")),
            }
        }
    }
    impl Version {
        fn has(&self, feat: Feature) -> bool {
            use Version::*;
            match self {
                Armv8p0a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p1a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p2a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p3a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p4a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p5a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p6a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p7a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p8a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv8p9a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv9p0a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv9p1a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv9p2a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv9p3a => {
                    ::core::panicking::panic_fmt(
                        format_args!("Feature list not supported yet for {0}", self),
                    );
                }
                Armv9p4a => v9p4a_has_feat(feat),
            }
        }
    }
    enum Feature {
        FeatAA32EL0,
        FeatAA32EL1,
        FeatAA32EL2,
        FeatAA32EL3,
        FeatAA64EL0,
        FeatAA64EL1,
        FeatAA64EL2,
        FeatAA64EL3,
        FeatEL0,
        FeatEL1,
        FeatEL2,
        FeatEL3,
        FeatAES,
        FeatAdvSIMD,
        FeatCSV2_1p1,
        FeatCSV2_1p2,
        FeatCSV2_2,
        FeatCSV2_3,
        FeatDoubleLock,
        FeatETMv4,
        FeatETMv4p1,
        FeatETMv4p2,
        FeatETMv4p3,
        FeatETMv4p4,
        FeatETMv4p5,
        FeatETMv4p6,
        FeatETS2,
        FeatFP,
        FeatGICv3,
        FeatGicv3Legacy,
        FeatGicv3Tdir,
        FeatGICv3p1,
        FeatGICv4,
        FeatGICv4p1,
        FeatIVIPT,
        FeatPCSRv8,
        FeatPMULL,
        FeatPMUv3,
        FeatPmuv3Ext,
        FeatPmuv3Ext32,
        FeatSHA1,
        FeatSHA256,
        FeatTrcExt,
        FeatTrcSr,
        FeatnTLBPA,
        FeatCRC32,
        FeatDebugv8p1,
        FeatHAFDBS,
        FeatHPDS,
        FeatLOR,
        FeatLSE,
        FeatPAN,
        FeatPMUv3p1,
        FeatRDM,
        FeatVHE,
        FeatVMID16,
        FeatAA32BF16,
        FeatAA32HPD,
        FeatAA32I8MM,
        FeatASMv8p2,
        FeatDPB,
        FeatDebugv8p2,
        FeatEDHSR,
        FeatF32MM,
        FeatF64MM,
        FeatFP16,
        FeatHPDS2,
        FeatI8MM,
        FeatIESB,
        FeatLPA,
        FeatLSMAOC,
        FeatLVA,
        FeatMPAM,
        FeatPAN2,
        FeatPCSRv8p2,
        FeatRAS,
        FeatSHA3,
        FeatSHA512,
        FeatSM3,
        FeatSM4,
        FeatSPE,
        FeatSVE,
        FeatTTCNP,
        FeatUAO,
        FeatVPIPT,
        FeatXNX,
        FeatCCIDX,
        FeatCONSTPACFIELD,
        FeatEPAC,
        FeatFCMA,
        FeatFPAC,
        FeatFPACCOMBINE,
        FeatJSCVT,
        FeatLRCPC,
        FeatNV,
        FeatPACIMP,
        FeatPACQARMA3,
        FeatPACQARMA5,
        FeatPAuth,
        FeatSPEv1p1,
        FeatAMUv1,
        FeatBBM,
        FeatCNTSC,
        FeatDIT,
        FeatDebugv8p4,
        FeatDotProd,
        FeatDoubleFault,
        FeatFHM,
        FeatFlagM,
        FeatIDST,
        FeatLRCPC2,
        FeatLSE2,
        FeatNV2,
        FeatPMUv3p4,
        FeatRASSAv1p1,
        FeatRASv1p1,
        FeatS2FWB,
        FeatSEL2,
        FeatTLBIOS,
        FeatTLBIRANGE,
        FeatTRF,
        FeatTTL,
        FeatTTST,
        FeatBTI,
        FeatCSV2,
        FeatCSV3,
        FeatDPB2,
        FeatE0PD,
        FeatEVT,
        FeatExS,
        FeatFRINTTS,
        FeatFlagM2,
        FeatGTG,
        FeatMTE,
        FeatMTE2,
        FeatPMUv3p5,
        FeatRNG,
        FeatRngTrap,
        FeatSB,
        FeatSPECRES,
        FeatSSBS,
        FeatSSBS2,
        FeatAMUv1p1,
        FeatBF16,
        FeatDGH,
        FeatECV,
        FeatFGT,
        FeatHPMN0,
        FeatMPAMv0p1,
        FeatMPAMv1p1,
        FeatMTPMU,
        FeatPAuth2,
        FeatTWED,
        FeatAFP,
        FeatEBF16,
        FeatHCX,
        FeatLPA2,
        FeatLS64,
        FeatLs64Accdata,
        FeatLs64V,
        FeatMTE3,
        FeatPAN3,
        FeatPMUv3p7,
        FeatRPRES,
        FeatSPEv1p2,
        FeatWFxT,
        FeatXS,
        FeatCMOW,
        FeatDebugv8p8,
        FeatGicv3Nmi,
        FeatHBC,
        FeatMOPS,
        FeatNMI,
        FeatPmuv3Ext64,
        FeatPmuv3Th,
        FeatPMUv3p8,
        FeatSCTLR2,
        FeatSPEv1p3,
        FeatTCR2,
        FeatTIDCP1,
        FeatADERR,
        FeatAIE,
        FeatANERR,
        FeatCLRBHB,
        FeatCSSC,
        FeatDebugv8p9,
        FeatDoubleFault2,
        FeatECBHB,
        FeatFGT2,
        FeatHAFT,
        FeatLRCPC3,
        FeatMTE4,
        FeatMteAsymFault,
        FeatMteAsync,
        FeatMteCanonicalTags,
        FeatMteNoAddressTags,
        FeatMtePerm,
        FeatMteStoreOnly,
        FeatMteTaggedFar,
        FeatPCSRv8p9,
        FeatPFAR,
        FeatPmuv3Edge,
        FeatPmuv3Icntr,
        FeatPmuv3Ss,
        FeatPMUv3p9,
        FeatPRFMSLC,
        FeatRASSAv2,
        FeatRASv2,
        FeatRPRFM,
        FeatS1PIE,
        FeatS1POE,
        FeatS2PIE,
        FeatS2POE,
        FeatSPECRES2,
        FeatSpeCrr,
        FeatSpeFds,
        FeatSPEv1p4,
        FeatSPMU,
        FeatTHE,
        FeatDoPD,
        FeatETE,
        FeatSVE2,
        FeatSveAes,
        FeatSveBitPerm,
        FeatSvePmull128,
        FeatSveSha3,
        FeatSveSm4,
        FeatTME,
        FeatTRBE,
        FeatETEv1p1,
        FeatBRBE,
        FeatETEv1p2,
        FeatRME,
        FeatSME,
        FeatSmeF64f64,
        FeatSmeFa64,
        FeatSmeI16i64,
        FeatBRBEv1p1,
        FeatMEC,
        FeatSME2,
        FeatABLE,
        FeatCHK,
        FeatD128,
        FeatEBEP,
        FeatETEv1p3,
        FeatGCS,
        FeatITE,
        FeatLSE128,
        FeatLVA3,
        FeatSEBEP,
        FeatSME2p1,
        FeatSmeF16f16,
        FeatSVE2p1,
        FeatSveB16b16,
        FeatSYSINSTR128,
        FeatSYSREG128,
        FeatTrbeExt,
        FeatTrbeMpam,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Feature {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Feature::FeatAA32EL0 => "FeatAA32EL0",
                    Feature::FeatAA32EL1 => "FeatAA32EL1",
                    Feature::FeatAA32EL2 => "FeatAA32EL2",
                    Feature::FeatAA32EL3 => "FeatAA32EL3",
                    Feature::FeatAA64EL0 => "FeatAA64EL0",
                    Feature::FeatAA64EL1 => "FeatAA64EL1",
                    Feature::FeatAA64EL2 => "FeatAA64EL2",
                    Feature::FeatAA64EL3 => "FeatAA64EL3",
                    Feature::FeatEL0 => "FeatEL0",
                    Feature::FeatEL1 => "FeatEL1",
                    Feature::FeatEL2 => "FeatEL2",
                    Feature::FeatEL3 => "FeatEL3",
                    Feature::FeatAES => "FeatAES",
                    Feature::FeatAdvSIMD => "FeatAdvSIMD",
                    Feature::FeatCSV2_1p1 => "FeatCSV2_1p1",
                    Feature::FeatCSV2_1p2 => "FeatCSV2_1p2",
                    Feature::FeatCSV2_2 => "FeatCSV2_2",
                    Feature::FeatCSV2_3 => "FeatCSV2_3",
                    Feature::FeatDoubleLock => "FeatDoubleLock",
                    Feature::FeatETMv4 => "FeatETMv4",
                    Feature::FeatETMv4p1 => "FeatETMv4p1",
                    Feature::FeatETMv4p2 => "FeatETMv4p2",
                    Feature::FeatETMv4p3 => "FeatETMv4p3",
                    Feature::FeatETMv4p4 => "FeatETMv4p4",
                    Feature::FeatETMv4p5 => "FeatETMv4p5",
                    Feature::FeatETMv4p6 => "FeatETMv4p6",
                    Feature::FeatETS2 => "FeatETS2",
                    Feature::FeatFP => "FeatFP",
                    Feature::FeatGICv3 => "FeatGICv3",
                    Feature::FeatGicv3Legacy => "FeatGicv3Legacy",
                    Feature::FeatGicv3Tdir => "FeatGicv3Tdir",
                    Feature::FeatGICv3p1 => "FeatGICv3p1",
                    Feature::FeatGICv4 => "FeatGICv4",
                    Feature::FeatGICv4p1 => "FeatGICv4p1",
                    Feature::FeatIVIPT => "FeatIVIPT",
                    Feature::FeatPCSRv8 => "FeatPCSRv8",
                    Feature::FeatPMULL => "FeatPMULL",
                    Feature::FeatPMUv3 => "FeatPMUv3",
                    Feature::FeatPmuv3Ext => "FeatPmuv3Ext",
                    Feature::FeatPmuv3Ext32 => "FeatPmuv3Ext32",
                    Feature::FeatSHA1 => "FeatSHA1",
                    Feature::FeatSHA256 => "FeatSHA256",
                    Feature::FeatTrcExt => "FeatTrcExt",
                    Feature::FeatTrcSr => "FeatTrcSr",
                    Feature::FeatnTLBPA => "FeatnTLBPA",
                    Feature::FeatCRC32 => "FeatCRC32",
                    Feature::FeatDebugv8p1 => "FeatDebugv8p1",
                    Feature::FeatHAFDBS => "FeatHAFDBS",
                    Feature::FeatHPDS => "FeatHPDS",
                    Feature::FeatLOR => "FeatLOR",
                    Feature::FeatLSE => "FeatLSE",
                    Feature::FeatPAN => "FeatPAN",
                    Feature::FeatPMUv3p1 => "FeatPMUv3p1",
                    Feature::FeatRDM => "FeatRDM",
                    Feature::FeatVHE => "FeatVHE",
                    Feature::FeatVMID16 => "FeatVMID16",
                    Feature::FeatAA32BF16 => "FeatAA32BF16",
                    Feature::FeatAA32HPD => "FeatAA32HPD",
                    Feature::FeatAA32I8MM => "FeatAA32I8MM",
                    Feature::FeatASMv8p2 => "FeatASMv8p2",
                    Feature::FeatDPB => "FeatDPB",
                    Feature::FeatDebugv8p2 => "FeatDebugv8p2",
                    Feature::FeatEDHSR => "FeatEDHSR",
                    Feature::FeatF32MM => "FeatF32MM",
                    Feature::FeatF64MM => "FeatF64MM",
                    Feature::FeatFP16 => "FeatFP16",
                    Feature::FeatHPDS2 => "FeatHPDS2",
                    Feature::FeatI8MM => "FeatI8MM",
                    Feature::FeatIESB => "FeatIESB",
                    Feature::FeatLPA => "FeatLPA",
                    Feature::FeatLSMAOC => "FeatLSMAOC",
                    Feature::FeatLVA => "FeatLVA",
                    Feature::FeatMPAM => "FeatMPAM",
                    Feature::FeatPAN2 => "FeatPAN2",
                    Feature::FeatPCSRv8p2 => "FeatPCSRv8p2",
                    Feature::FeatRAS => "FeatRAS",
                    Feature::FeatSHA3 => "FeatSHA3",
                    Feature::FeatSHA512 => "FeatSHA512",
                    Feature::FeatSM3 => "FeatSM3",
                    Feature::FeatSM4 => "FeatSM4",
                    Feature::FeatSPE => "FeatSPE",
                    Feature::FeatSVE => "FeatSVE",
                    Feature::FeatTTCNP => "FeatTTCNP",
                    Feature::FeatUAO => "FeatUAO",
                    Feature::FeatVPIPT => "FeatVPIPT",
                    Feature::FeatXNX => "FeatXNX",
                    Feature::FeatCCIDX => "FeatCCIDX",
                    Feature::FeatCONSTPACFIELD => "FeatCONSTPACFIELD",
                    Feature::FeatEPAC => "FeatEPAC",
                    Feature::FeatFCMA => "FeatFCMA",
                    Feature::FeatFPAC => "FeatFPAC",
                    Feature::FeatFPACCOMBINE => "FeatFPACCOMBINE",
                    Feature::FeatJSCVT => "FeatJSCVT",
                    Feature::FeatLRCPC => "FeatLRCPC",
                    Feature::FeatNV => "FeatNV",
                    Feature::FeatPACIMP => "FeatPACIMP",
                    Feature::FeatPACQARMA3 => "FeatPACQARMA3",
                    Feature::FeatPACQARMA5 => "FeatPACQARMA5",
                    Feature::FeatPAuth => "FeatPAuth",
                    Feature::FeatSPEv1p1 => "FeatSPEv1p1",
                    Feature::FeatAMUv1 => "FeatAMUv1",
                    Feature::FeatBBM => "FeatBBM",
                    Feature::FeatCNTSC => "FeatCNTSC",
                    Feature::FeatDIT => "FeatDIT",
                    Feature::FeatDebugv8p4 => "FeatDebugv8p4",
                    Feature::FeatDotProd => "FeatDotProd",
                    Feature::FeatDoubleFault => "FeatDoubleFault",
                    Feature::FeatFHM => "FeatFHM",
                    Feature::FeatFlagM => "FeatFlagM",
                    Feature::FeatIDST => "FeatIDST",
                    Feature::FeatLRCPC2 => "FeatLRCPC2",
                    Feature::FeatLSE2 => "FeatLSE2",
                    Feature::FeatNV2 => "FeatNV2",
                    Feature::FeatPMUv3p4 => "FeatPMUv3p4",
                    Feature::FeatRASSAv1p1 => "FeatRASSAv1p1",
                    Feature::FeatRASv1p1 => "FeatRASv1p1",
                    Feature::FeatS2FWB => "FeatS2FWB",
                    Feature::FeatSEL2 => "FeatSEL2",
                    Feature::FeatTLBIOS => "FeatTLBIOS",
                    Feature::FeatTLBIRANGE => "FeatTLBIRANGE",
                    Feature::FeatTRF => "FeatTRF",
                    Feature::FeatTTL => "FeatTTL",
                    Feature::FeatTTST => "FeatTTST",
                    Feature::FeatBTI => "FeatBTI",
                    Feature::FeatCSV2 => "FeatCSV2",
                    Feature::FeatCSV3 => "FeatCSV3",
                    Feature::FeatDPB2 => "FeatDPB2",
                    Feature::FeatE0PD => "FeatE0PD",
                    Feature::FeatEVT => "FeatEVT",
                    Feature::FeatExS => "FeatExS",
                    Feature::FeatFRINTTS => "FeatFRINTTS",
                    Feature::FeatFlagM2 => "FeatFlagM2",
                    Feature::FeatGTG => "FeatGTG",
                    Feature::FeatMTE => "FeatMTE",
                    Feature::FeatMTE2 => "FeatMTE2",
                    Feature::FeatPMUv3p5 => "FeatPMUv3p5",
                    Feature::FeatRNG => "FeatRNG",
                    Feature::FeatRngTrap => "FeatRngTrap",
                    Feature::FeatSB => "FeatSB",
                    Feature::FeatSPECRES => "FeatSPECRES",
                    Feature::FeatSSBS => "FeatSSBS",
                    Feature::FeatSSBS2 => "FeatSSBS2",
                    Feature::FeatAMUv1p1 => "FeatAMUv1p1",
                    Feature::FeatBF16 => "FeatBF16",
                    Feature::FeatDGH => "FeatDGH",
                    Feature::FeatECV => "FeatECV",
                    Feature::FeatFGT => "FeatFGT",
                    Feature::FeatHPMN0 => "FeatHPMN0",
                    Feature::FeatMPAMv0p1 => "FeatMPAMv0p1",
                    Feature::FeatMPAMv1p1 => "FeatMPAMv1p1",
                    Feature::FeatMTPMU => "FeatMTPMU",
                    Feature::FeatPAuth2 => "FeatPAuth2",
                    Feature::FeatTWED => "FeatTWED",
                    Feature::FeatAFP => "FeatAFP",
                    Feature::FeatEBF16 => "FeatEBF16",
                    Feature::FeatHCX => "FeatHCX",
                    Feature::FeatLPA2 => "FeatLPA2",
                    Feature::FeatLS64 => "FeatLS64",
                    Feature::FeatLs64Accdata => "FeatLs64Accdata",
                    Feature::FeatLs64V => "FeatLs64V",
                    Feature::FeatMTE3 => "FeatMTE3",
                    Feature::FeatPAN3 => "FeatPAN3",
                    Feature::FeatPMUv3p7 => "FeatPMUv3p7",
                    Feature::FeatRPRES => "FeatRPRES",
                    Feature::FeatSPEv1p2 => "FeatSPEv1p2",
                    Feature::FeatWFxT => "FeatWFxT",
                    Feature::FeatXS => "FeatXS",
                    Feature::FeatCMOW => "FeatCMOW",
                    Feature::FeatDebugv8p8 => "FeatDebugv8p8",
                    Feature::FeatGicv3Nmi => "FeatGicv3Nmi",
                    Feature::FeatHBC => "FeatHBC",
                    Feature::FeatMOPS => "FeatMOPS",
                    Feature::FeatNMI => "FeatNMI",
                    Feature::FeatPmuv3Ext64 => "FeatPmuv3Ext64",
                    Feature::FeatPmuv3Th => "FeatPmuv3Th",
                    Feature::FeatPMUv3p8 => "FeatPMUv3p8",
                    Feature::FeatSCTLR2 => "FeatSCTLR2",
                    Feature::FeatSPEv1p3 => "FeatSPEv1p3",
                    Feature::FeatTCR2 => "FeatTCR2",
                    Feature::FeatTIDCP1 => "FeatTIDCP1",
                    Feature::FeatADERR => "FeatADERR",
                    Feature::FeatAIE => "FeatAIE",
                    Feature::FeatANERR => "FeatANERR",
                    Feature::FeatCLRBHB => "FeatCLRBHB",
                    Feature::FeatCSSC => "FeatCSSC",
                    Feature::FeatDebugv8p9 => "FeatDebugv8p9",
                    Feature::FeatDoubleFault2 => "FeatDoubleFault2",
                    Feature::FeatECBHB => "FeatECBHB",
                    Feature::FeatFGT2 => "FeatFGT2",
                    Feature::FeatHAFT => "FeatHAFT",
                    Feature::FeatLRCPC3 => "FeatLRCPC3",
                    Feature::FeatMTE4 => "FeatMTE4",
                    Feature::FeatMteAsymFault => "FeatMteAsymFault",
                    Feature::FeatMteAsync => "FeatMteAsync",
                    Feature::FeatMteCanonicalTags => "FeatMteCanonicalTags",
                    Feature::FeatMteNoAddressTags => "FeatMteNoAddressTags",
                    Feature::FeatMtePerm => "FeatMtePerm",
                    Feature::FeatMteStoreOnly => "FeatMteStoreOnly",
                    Feature::FeatMteTaggedFar => "FeatMteTaggedFar",
                    Feature::FeatPCSRv8p9 => "FeatPCSRv8p9",
                    Feature::FeatPFAR => "FeatPFAR",
                    Feature::FeatPmuv3Edge => "FeatPmuv3Edge",
                    Feature::FeatPmuv3Icntr => "FeatPmuv3Icntr",
                    Feature::FeatPmuv3Ss => "FeatPmuv3Ss",
                    Feature::FeatPMUv3p9 => "FeatPMUv3p9",
                    Feature::FeatPRFMSLC => "FeatPRFMSLC",
                    Feature::FeatRASSAv2 => "FeatRASSAv2",
                    Feature::FeatRASv2 => "FeatRASv2",
                    Feature::FeatRPRFM => "FeatRPRFM",
                    Feature::FeatS1PIE => "FeatS1PIE",
                    Feature::FeatS1POE => "FeatS1POE",
                    Feature::FeatS2PIE => "FeatS2PIE",
                    Feature::FeatS2POE => "FeatS2POE",
                    Feature::FeatSPECRES2 => "FeatSPECRES2",
                    Feature::FeatSpeCrr => "FeatSpeCrr",
                    Feature::FeatSpeFds => "FeatSpeFds",
                    Feature::FeatSPEv1p4 => "FeatSPEv1p4",
                    Feature::FeatSPMU => "FeatSPMU",
                    Feature::FeatTHE => "FeatTHE",
                    Feature::FeatDoPD => "FeatDoPD",
                    Feature::FeatETE => "FeatETE",
                    Feature::FeatSVE2 => "FeatSVE2",
                    Feature::FeatSveAes => "FeatSveAes",
                    Feature::FeatSveBitPerm => "FeatSveBitPerm",
                    Feature::FeatSvePmull128 => "FeatSvePmull128",
                    Feature::FeatSveSha3 => "FeatSveSha3",
                    Feature::FeatSveSm4 => "FeatSveSm4",
                    Feature::FeatTME => "FeatTME",
                    Feature::FeatTRBE => "FeatTRBE",
                    Feature::FeatETEv1p1 => "FeatETEv1p1",
                    Feature::FeatBRBE => "FeatBRBE",
                    Feature::FeatETEv1p2 => "FeatETEv1p2",
                    Feature::FeatRME => "FeatRME",
                    Feature::FeatSME => "FeatSME",
                    Feature::FeatSmeF64f64 => "FeatSmeF64f64",
                    Feature::FeatSmeFa64 => "FeatSmeFa64",
                    Feature::FeatSmeI16i64 => "FeatSmeI16i64",
                    Feature::FeatBRBEv1p1 => "FeatBRBEv1p1",
                    Feature::FeatMEC => "FeatMEC",
                    Feature::FeatSME2 => "FeatSME2",
                    Feature::FeatABLE => "FeatABLE",
                    Feature::FeatCHK => "FeatCHK",
                    Feature::FeatD128 => "FeatD128",
                    Feature::FeatEBEP => "FeatEBEP",
                    Feature::FeatETEv1p3 => "FeatETEv1p3",
                    Feature::FeatGCS => "FeatGCS",
                    Feature::FeatITE => "FeatITE",
                    Feature::FeatLSE128 => "FeatLSE128",
                    Feature::FeatLVA3 => "FeatLVA3",
                    Feature::FeatSEBEP => "FeatSEBEP",
                    Feature::FeatSME2p1 => "FeatSME2p1",
                    Feature::FeatSmeF16f16 => "FeatSmeF16f16",
                    Feature::FeatSVE2p1 => "FeatSVE2p1",
                    Feature::FeatSveB16b16 => "FeatSveB16b16",
                    Feature::FeatSYSINSTR128 => "FeatSYSINSTR128",
                    Feature::FeatSYSREG128 => "FeatSYSREG128",
                    Feature::FeatTrbeExt => "FeatTrbeExt",
                    Feature::FeatTrbeMpam => "FeatTrbeMpam",
                },
            )
        }
    }
    fn v9p4a_has_feat(feat: Feature) -> bool {
        use Feature::*;
        match feat {
            FeatAA32EL0 => true,
            FeatAA32EL1 => false,
            FeatAA32EL2 => false,
            FeatAA32EL3 => false,
            FeatAA64EL0 => true,
            FeatAA64EL1 => true,
            FeatAA64EL2 => true,
            FeatAA64EL3 => true,
            FeatEL0 => true,
            FeatEL1 => true,
            FeatEL2 => true,
            FeatEL3 => true,
            FeatAES => true,
            FeatAdvSIMD => true,
            FeatCSV2_1p1 => true,
            FeatCSV2_1p2 => true,
            FeatCSV2_2 => true,
            FeatCSV2_3 => true,
            FeatDoubleLock => true,
            FeatETMv4 => false,
            FeatETMv4p1 => true,
            FeatETMv4p2 => true,
            FeatETMv4p3 => true,
            FeatETMv4p4 => true,
            FeatETMv4p5 => true,
            FeatETMv4p6 => true,
            FeatETS2 => true,
            FeatFP => true,
            FeatGICv3 => true,
            FeatGicv3Legacy => true,
            FeatGicv3Tdir => true,
            FeatGICv3p1 => true,
            FeatGICv4 => true,
            FeatGICv4p1 => true,
            FeatIVIPT => true,
            FeatPCSRv8 => true,
            FeatPMULL => true,
            FeatPMUv3 => true,
            FeatPmuv3Ext => true,
            FeatPmuv3Ext32 => true,
            FeatSHA1 => true,
            FeatSHA256 => true,
            FeatTrcExt => true,
            FeatTrcSr => true,
            FeatnTLBPA => true,
            FeatCRC32 => true,
            FeatDebugv8p1 => true,
            FeatHAFDBS => true,
            FeatHPDS => true,
            FeatLOR => true,
            FeatLSE => true,
            FeatPAN => true,
            FeatPMUv3p1 => true,
            FeatRDM => true,
            FeatVHE => true,
            FeatVMID16 => true,
            FeatAA32BF16 => true,
            FeatAA32HPD => true,
            FeatAA32I8MM => true,
            FeatASMv8p2 => true,
            FeatDPB => true,
            FeatDebugv8p2 => true,
            FeatEDHSR => true,
            FeatF32MM => true,
            FeatF64MM => true,
            FeatFP16 => true,
            FeatHPDS2 => true,
            FeatI8MM => true,
            FeatIESB => true,
            FeatLPA => true,
            FeatLSMAOC => true,
            FeatLVA => true,
            FeatMPAM => true,
            FeatPAN2 => true,
            FeatPCSRv8p2 => true,
            FeatRAS => true,
            FeatSHA3 => true,
            FeatSHA512 => true,
            FeatSM3 => true,
            FeatSM4 => true,
            FeatSPE => true,
            FeatSVE => true,
            FeatTTCNP => true,
            FeatUAO => true,
            FeatVPIPT => true,
            FeatXNX => true,
            FeatCCIDX => true,
            FeatCONSTPACFIELD => false,
            FeatEPAC => false,
            FeatFCMA => true,
            FeatFPAC => true,
            FeatFPACCOMBINE => true,
            FeatJSCVT => true,
            FeatLRCPC => true,
            FeatNV => true,
            FeatPACIMP => false,
            FeatPACQARMA3 => false,
            FeatPACQARMA5 => true,
            FeatPAuth => true,
            FeatSPEv1p1 => true,
            FeatAMUv1 => true,
            FeatBBM => true,
            FeatCNTSC => true,
            FeatDIT => true,
            FeatDebugv8p4 => true,
            FeatDotProd => true,
            FeatDoubleFault => true,
            FeatFHM => true,
            FeatFlagM => true,
            FeatIDST => true,
            FeatLRCPC2 => true,
            FeatLSE2 => true,
            FeatNV2 => true,
            FeatPMUv3p4 => true,
            FeatRASSAv1p1 => true,
            FeatRASv1p1 => true,
            FeatS2FWB => true,
            FeatSEL2 => true,
            FeatTLBIOS => true,
            FeatTLBIRANGE => true,
            FeatTRF => true,
            FeatTTL => true,
            FeatTTST => true,
            FeatBTI => true,
            FeatCSV2 => true,
            FeatCSV3 => true,
            FeatDPB2 => true,
            FeatE0PD => true,
            FeatEVT => true,
            FeatExS => true,
            FeatFRINTTS => true,
            FeatFlagM2 => true,
            FeatGTG => true,
            FeatMTE => true,
            FeatMTE2 => true,
            FeatPMUv3p5 => true,
            FeatRNG => true,
            FeatRngTrap => true,
            FeatSB => true,
            FeatSPECRES => true,
            FeatSSBS => true,
            FeatSSBS2 => true,
            FeatAMUv1p1 => true,
            FeatBF16 => true,
            FeatDGH => true,
            FeatECV => true,
            FeatFGT => true,
            FeatHPMN0 => true,
            FeatMPAMv0p1 => true,
            FeatMPAMv1p1 => true,
            FeatMTPMU => true,
            FeatPAuth2 => true,
            FeatTWED => true,
            FeatAFP => true,
            FeatEBF16 => true,
            FeatHCX => true,
            FeatLPA2 => true,
            FeatLS64 => true,
            FeatLs64Accdata => true,
            FeatLs64V => true,
            FeatMTE3 => true,
            FeatPAN3 => true,
            FeatPMUv3p7 => true,
            FeatRPRES => true,
            FeatSPEv1p2 => true,
            FeatWFxT => true,
            FeatXS => true,
            FeatCMOW => true,
            FeatDebugv8p8 => true,
            FeatGicv3Nmi => true,
            FeatHBC => true,
            FeatMOPS => true,
            FeatNMI => true,
            FeatPmuv3Ext64 => true,
            FeatPmuv3Th => true,
            FeatPMUv3p8 => true,
            FeatSCTLR2 => true,
            FeatSPEv1p3 => true,
            FeatTCR2 => true,
            FeatTIDCP1 => true,
            FeatADERR => true,
            FeatAIE => true,
            FeatANERR => true,
            FeatCLRBHB => true,
            FeatCSSC => true,
            FeatDebugv8p9 => true,
            FeatDoubleFault2 => true,
            FeatECBHB => true,
            FeatFGT2 => true,
            FeatHAFT => true,
            FeatLRCPC3 => true,
            FeatMTE4 => true,
            FeatMteAsymFault => true,
            FeatMteAsync => true,
            FeatMteCanonicalTags => true,
            FeatMteNoAddressTags => true,
            FeatMtePerm => true,
            FeatMteStoreOnly => true,
            FeatMteTaggedFar => true,
            FeatPCSRv8p9 => true,
            FeatPFAR => true,
            FeatPmuv3Edge => true,
            FeatPmuv3Icntr => true,
            FeatPmuv3Ss => true,
            FeatPMUv3p9 => true,
            FeatPRFMSLC => true,
            FeatRASSAv2 => true,
            FeatRASv2 => true,
            FeatRPRFM => true,
            FeatS1PIE => true,
            FeatS1POE => true,
            FeatS2PIE => true,
            FeatS2POE => true,
            FeatSPECRES2 => true,
            FeatSpeCrr => true,
            FeatSpeFds => true,
            FeatSPEv1p4 => true,
            FeatSPMU => true,
            FeatTHE => true,
            FeatDoPD => true,
            FeatETE => true,
            FeatSVE2 => true,
            FeatSveAes => true,
            FeatSveBitPerm => true,
            FeatSvePmull128 => true,
            FeatSveSha3 => true,
            FeatSveSm4 => true,
            FeatTME => true,
            FeatTRBE => true,
            FeatETEv1p1 => true,
            FeatBRBE => true,
            FeatETEv1p2 => true,
            FeatRME => true,
            FeatSME => true,
            FeatSmeF64f64 => true,
            FeatSmeFa64 => true,
            FeatSmeI16i64 => true,
            FeatBRBEv1p1 => true,
            FeatMEC => true,
            FeatSME2 => true,
            FeatABLE => true,
            FeatCHK => true,
            FeatD128 => true,
            FeatEBEP => true,
            FeatETEv1p3 => true,
            FeatGCS => true,
            FeatITE => true,
            FeatLSE128 => true,
            FeatLVA3 => true,
            FeatSEBEP => true,
            FeatSME2p1 => true,
            FeatSmeF16f16 => true,
            FeatSVE2p1 => true,
            FeatSveB16b16 => true,
            FeatSYSINSTR128 => true,
            FeatSYSREG128 => true,
            FeatTrbeExt => true,
            FeatTrbeMpam => true,
        }
    }
}
pub use variants::Version;
type Flag = ::bitvec::array::BitArray<
    [usize; ::bitvec::mem::elts::<usize>(1)],
    ::bitvec::order::Lsb0,
>;
type El = ::bitvec::array::BitArray<
    [usize; ::bitvec::mem::elts::<usize>(2)],
    ::bitvec::order::Lsb0,
>;
type Reg64 = ::bitvec::array::BitArray<
    [usize; ::bitvec::mem::elts::<usize>(64)],
    ::bitvec::order::Lsb0,
>;
pub struct Arm {
    r: [Reg64; 31],
    sp: [Reg64; 4],
    pc: Reg64,
    pstate: Pstate,
    version: Version,
    endian: Endian,
}
#[automatically_derived]
impl ::core::fmt::Debug for Arm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &["r", "sp", "pc", "pstate", "version", "endian"];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.r,
            &self.sp,
            &self.pc,
            &self.pstate,
            &self.version,
            &&self.endian,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Arm", names, values)
    }
}
impl Arm {
    /// Initialize an Arm CPU of the given version
    pub fn new(v: Version) -> Self {
        Self {
            r: [Reg64::ZERO; 31],
            sp: [Reg64::ZERO; 4],
            pc: Reg64::ZERO,
            pstate: Pstate::new(),
            version: v,
            endian: Endian::Little,
        }
    }
    /// Initialize a big endian Arm CPU of the given version
    pub fn new_be(v: Version) -> Self {
        Self {
            r: [Reg64::ZERO; 31],
            sp: [Reg64::ZERO; 4],
            pc: Reg64::ZERO,
            pstate: Pstate::new(),
            version: v,
            endian: Endian::Big,
        }
    }
    /// Get a reference to the SP based on EL
    pub fn sp(&self) -> &Reg64 {
        if self.pstate.sp[0] {
            &self.sp[0]
        } else {
            let (hi, lo) = (self.pstate.el[1], self.pstate.el[0]);
            match (hi, lo) {
                (false, false) => &self.sp[0],
                (false, true) => &self.sp[1],
                (true, false) => &self.sp[2],
                (true, true) => &self.sp[3],
            }
        }
    }
    /// Get a reference to a general purpose register
    pub fn x(&self, index: usize) -> Option<u64> {
        self.r.get(index).and_then(|x| Some(x.load()))
    }
    /// Get a mutable reference to a general purpose register
    pub fn set_x(&mut self, index: usize, val: u64) -> Option<u64> {
        self.r
            .get_mut(index)
            .and_then(|x| {
                x.store(val);
                Some(val)
            })
    }
    /// Get a reference to the lower 32-bits of a general purpose register
    pub fn w(&self, index: usize) -> Option<u32> {
        self.r.get(index).and_then(|x| Some(&x[0..32])).and_then(|w| Some(w.load()))
    }
    /// Get a mutable reference to the lower 32-bits of a general purpose register
    pub fn set_w(&mut self, index: usize, val: u32) -> Option<u32> {
        self.r
            .get_mut(index)
            .and_then(|x| Some(&mut x[0..32]))
            .and_then(|w| {
                w.store(val);
                Some(val)
            })
    }
    /// Return the value of the negative condition flag (N)
    pub fn n(&self) -> bool {
        self.pstate.n[0]
    }
    /// Return the value of the zero condition flag (Z)
    pub fn z(&self) -> bool {
        self.pstate.z[0]
    }
    /// Return the value of the carry condition flag (C)
    pub fn c(&self) -> bool {
        self.pstate.c[0]
    }
    /// Return the value of the overflow condition flag (V)
    pub fn v(&self) -> bool {
        self.pstate.v[0]
    }
}
impl Processor for Arm {
    type Insn = ArmInstruction;
    fn name(&self) -> String {
        {
            let res = ::alloc::fmt::format(format_args!("{0}", self.version));
            res
        }
    }
    fn ip(&self) -> usize {
        self.pc.load()
    }
    fn set_ip(&mut self, addr: usize) {
        self.pc.store(addr)
    }
    fn fetch_decode(&self, addr: usize, mem: &[u8]) -> Result<ArmInstruction> {
        let insn: [u8; 4] = mem
            .get(addr..addr + 4)
            .ok_or_else(|| Error({
                let res = ::alloc::fmt::format(
                    format_args!("Failed to fetch 4 bytes from {0}", addr),
                );
                res
            }))?
            .try_into()
            .unwrap();
        let insn = match self.endian {
            Endian::Little => u32::from_le_bytes(insn),
            Endian::Big => {
                ::core::panicking::panic_fmt(format_args!("Big Endian ARM Unsupported"));
            }
        } as usize;
        ArmInstruction::decode(insn.view_bits())
            .ok_or_else(|| {
                Error({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Failed to decode the 4 bytes at {0} into an instruction",
                            addr,
                        ),
                    );
                    res
                })
            })
    }
}
struct Pstate {
    n: Flag,
    z: Flag,
    c: Flag,
    v: Flag,
    el: El,
    sp: Flag,
}
#[automatically_derived]
impl ::core::fmt::Debug for Pstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &["n", "z", "c", "v", "el", "sp"];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.n,
            &self.z,
            &self.c,
            &self.v,
            &self.el,
            &&self.sp,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Pstate", names, values)
    }
}
impl Pstate {
    fn new() -> Self {
        Self {
            n: Flag::ZERO,
            z: Flag::ZERO,
            c: Flag::ZERO,
            v: Flag::ZERO,
            el: El::ZERO,
            sp: Flag::ZERO,
        }
    }
}
