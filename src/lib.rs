#![feature(bigint_helper_methods)]
#![feature(generic_const_exprs)]

/*
 * NIGHTLY behavior:
 * 1. big_int_math for proper "full-adders" in pure Rust language functions (probably compiles directly to LLVM UAdd/etc. intrinsics)
 * 2. generic_const_exprs for use of a generic's associated constants in a const generic function signature. These are used to check valid Register/RegisterView
 * definitions
 */

pub mod core {
    pub mod model;
}
pub use core::model;

pub mod std {
    pub mod arm;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use capstone::{
        arch::{arm64::ArchMode, BuildsCapstone, BuildsCapstoneEndian},
        Capstone, Endian,
    };

    use crate::core::model::*;
    use crate::std::arm::*;
    use exec::StepEmu;

    #[test]
    fn sail_linux_boot() {
        // Define
        let mut vm = System::new("Sail ARM Machine");
        let sysbus = vm.sysbus();
        // These devices are detailed in sail.dts
        let cpu0 = vm.add_proc("cpu0", ArmCpu::v9p4a());
        let intc = vm.add_dev(
            "interrupt-controller",
            Gicv2::new(sysbus, 0x2c001000, [&cpu0]),
        );
        vm.add_dev("timer", GTimer::new(cpu0.id(), &intc, 13, 14, 11, 10));
        vm.add_dev(
            "serial0",
            PL011::new(sysbus, 0x3c000000, intc.intr_in("PPI", 5)),
        );
        vm.add_ram("memory", 0x80000000, 0x9000000);
        // These devices are undocumented but used by bootloader.bin and are easily RE'd
        vm.add_dev("TUBE", Tube::new(sysbus, 0x13000000));
        vm.add_ram("boot-memory", 0xdead0000, 0x20000);

        // Initialize
        vm.init_mem(0x80000000, &fs::read("tests/sail/bootloader.bin").unwrap());
        vm.init_mem(0x81000000, &fs::read("tests/sail/sail.dtb").unwrap());
        vm.init_mem(0x82080000, &fs::read("tests/sail/Image").unwrap());
        init_cpu(vm.proc_mut(cpu0.id()).unwrap(), 0x80000000);

        // Instrument and run
        let cap = Capstone::new()
            .arm64()
            .mode(ArchMode::Arm)
            .endian(Endian::Little)
            .build()
            .expect("Unable to build Arm64 Capstone instance");

        let mut emu = StepEmu::new(vm);
        emu.on_step(|time, cpu| {
            println!("Time = {time}");
        });
        emu.on_insn(|ip_va, insn, _ir_g, _cpu| {
            println!("{0}", disasm(insn, ip_va, &cap));
        });
        emu.run(None);
    }

    // Undocumented device that the Sail bootoader uses. Appers to behave as a stdout serial port
    struct Tube {
        bus: MemParent,
        base: usize,
    }

    impl Tube {
        fn new(bus: MemParent, base: usize) -> Self {
            Self { bus, base }
        }

        fn mmio_read(&mut self, _: usize, _: usize, _: &mut DevCtx) -> MemReadResult {
            MemReadResult(None)
        }

        fn mmio_write(&mut self, _: usize, data: &[u8], _: &mut DevCtx) -> MemWriteResult {
            if data.len() > 1 {
                panic!("Tried to write to TUBE with more than one byte",);
            } else {
                print!("TUBE WRITE: {0}", *data.first().unwrap() as char);
                MemWriteResult(Some(()))
            }
        }
    }

    impl Device for Tube {
        fn declare(&self, decl: &mut DevDecl<Self>)
        where
            Self: Sized,
        {
            decl.mmio(
                "Tube-reg",
                self.bus,
                self.base,
                1,
                Self::mmio_read,
                Self::mmio_write,
            );
        }
    }

    fn init_cpu(cpu: &mut ProcData<ArmCpu>, pc: u64) {
        cpu.write::<PC>(pc);
        cpu.write::<pstate::EL>(EL::EL3.as_num());
        cpu.set_rng(sail_rng);

        // Values copied from Sail
        // TODO: ArmCpu should set most of these itself on initialization
        cpu.write::<aarch64::CLIDR_EL1>(0x0000000000000023);
        cpu.write::<aarch64::CTR_EL0>(0x0000000484448004);
        cpu.write::<aarch64::DCZID_EL0>(0x0000000000000008);
        cpu.write::<aarch64::GMID_EL1>(0x0000000000000004);
        cpu.write::<aarch64::ID_AA64DFR0_EL1>(0x112101f5e1e1e91b);
        cpu.write::<aarch64::ID_AA64DFR1_EL1>(0x3f0111113f3f3f1f);
        cpu.write::<aarch64::ID_AA64ISAR0_EL1>(0x1221111110312120);
        cpu.write::<aarch64::ID_AA64ISAR1_EL1>(0x3111221101311052);
        cpu.write::<aarch64::ID_AA64ISAR2_EL1>(0x0011011110110012);
        cpu.write::<aarch64::ID_AA64MMFR0_EL1>(0x2200132310201127);
        cpu.write::<aarch64::ID_AA64MMFR1_EL1>(0x1111112110312123);
        cpu.write::<aarch64::ID_AA64PFR0_EL1>(0x1311211130111112);
        cpu.write::<aarch64::ID_AA64PFR1_EL1>(0x1111101012010321);
        cpu.write::<aarch64::ID_AA64SMFR0_EL1>(0x82f15cff00000000);
        cpu.write::<aarch64::ID_AA64ZFR0_EL1>(0x0110110101210022);
        cpu.write::<aarch64::ID_DFR0_EL1>(0x00000000190110bb);
        cpu.write::<aarch64::ID_DFR1_EL1>(0x0000000000000011);
        cpu.write::<aarch64::ID_ISAR0_EL1>(0x0000000002101110);
        cpu.write::<aarch64::ID_ISAR1_EL1>(0x0000000013112111);
        cpu.write::<aarch64::ID_ISAR2_EL1>(0x0000000021232042);
        cpu.write::<aarch64::ID_ISAR3_EL1>(0x0000000001112131);
        cpu.write::<aarch64::ID_ISAR4_EL1>(0x0000000000010142);
        cpu.write::<aarch64::ID_ISAR5_EL1>(0x0000000011011121);
        cpu.write::<aarch64::ID_ISAR6_EL1>(0x0000000011121111);
        cpu.write::<aarch64::ID_MMFR0_EL1>(0x0000000010201105);
        cpu.write::<aarch64::ID_MMFR1_EL1>(0x0000000020000000);
        cpu.write::<aarch64::ID_MMFR2_EL1>(0x0000000001260000);
        cpu.write::<aarch64::ID_MMFR3_EL1>(0x0000000002122211);
        cpu.write::<aarch64::ID_MMFR5_EL1>(0x0000000000000012);
        cpu.write::<aarch64::ID_PFR0_EL1>(0x0000000031210131);
        cpu.write::<aarch64::ID_PFR1_EL1>(0x0000000000020000);
        cpu.write::<aarch64::ID_PFR2_EL1>(0x0000000000000011);
        cpu.write::<aarch64::MIDR_EL1>(0x00000000410fd0f0);
        cpu.write::<aarch64::MPIDR_EL1>(0x0000000080000000);
        cpu.write::<aarch64::MVFR0_EL1>(0x0000000010111222);
        cpu.write::<aarch64::MVFR1_EL1>(0x0000000013211111);
        cpu.write::<aarch64::MVFR2_EL1>(0x0000000000000043);
        cpu.write::<aarch64::PAR_EL1>(0x0000000000000800);
        cpu.write::<aarch64::PMCR_EL0>(0x000000000000f800);
        cpu.write::<aarch64::REVIDR_EL1>(0x0000000000000000);
        cpu.write::<aarch64::SCTLR_EL1>(0x0000000000400800);
        cpu.write::<aarch64::ID_AA64MMFR2_EL1>(0x1221011112121111);
        cpu.write::<aarch64::ID_MMFR4_EL1>(0x0000000001121110);
    }

    // Trace checking code
    fn disasm(insn: u32, pc: usize, cap: &Capstone) -> String {
        let bytes = insn.to_le_bytes();
        let disas = cap
            .disasm_all(&bytes, pc as u64)
            .expect("Unable to disassemble instruction");
        let disas = disas
            .first()
            .and_then(|insn| Some(format!("{insn}")))
            .unwrap_or_else(|| {
                String::from("Instruction was disassembled but could not be unpacked from Capstone")
            });
        disas
    }

    // Emulated Sail RNG
    static mut SAIL_RNG_INDEX: usize = 0;
    fn sail_rng() -> u64 {
        unsafe {
            let ret = if SAIL_RNG_INDEX == 0 {
                0x842a5a76d8bad7f9
            } else if SAIL_RNG_INDEX == 1 {
                0xa7787229e009f7b7
            } else if SAIL_RNG_INDEX == 2 {
                0xd002150fdbd6fc5e
            } else if SAIL_RNG_INDEX == 3 {
                0x0c4d4c1f4b9233bc
            }
            // 5495372
            else if SAIL_RNG_INDEX == 4 {
                0x2dc3a14975601a34
            }
            // 5495449
            else if SAIL_RNG_INDEX == 5 {
                0xde4f14b14de070b9
            }
            // 5495526
            else if SAIL_RNG_INDEX == 6 {
                0xf50e248e6e3f49ae
            }
            // 5495603
            else if SAIL_RNG_INDEX == 7 {
                0x04ab9d19e40d5692
            }
            // 11595235
            else if SAIL_RNG_INDEX == 8 {
                0x664e5407210efd43
            }
            // 11595258
            else if SAIL_RNG_INDEX == 9 {
                0x7b1ae49d4df087a1
            }
            // 11595281
            else if SAIL_RNG_INDEX == 10 {
                0x0269e18b6669f68c
            }
            // 11595304
            else if SAIL_RNG_INDEX == 11 {
                0x01cbb471bfc3a034
            }
            // 11607183
            else if SAIL_RNG_INDEX == 12 {
                0x1f883c2360a7328a
            }
            // 11607206
            else if SAIL_RNG_INDEX == 13 {
                0x7c059d8d6a15597b
            }
            // 11607229
            else if SAIL_RNG_INDEX == 14 {
                0xbf9b63bfa982c759
            }
            // 11607252
            else if SAIL_RNG_INDEX == 15 {
                0x10e7707175809782
            }
            // 11642411
            else if SAIL_RNG_INDEX == 16 {
                0xd588224d2697c925
            }
            // 11642434
            else if SAIL_RNG_INDEX == 17 {
                0xe5ae26477e6cb5c0
            }
            // 11642457
            else if SAIL_RNG_INDEX == 18 {
                0x368bcd7045d93b8a
            }
            // 11642480
            else if SAIL_RNG_INDEX == 19 {
                0x389a5e6a86a28ebf
            }
            // 11654349
            else if SAIL_RNG_INDEX == 20 {
                0x5e7610557b693076
            }
            // 11654372
            else if SAIL_RNG_INDEX == 21 {
                0xf81fc64b89835a2c
            }
            // 11654395
            else if SAIL_RNG_INDEX == 22 {
                0x585b8f2082b32763
            }
            // 11654418
            else if SAIL_RNG_INDEX == 23 {
                0xa0ff2ea9ae9c303a
            }
            // 11691933
            else if SAIL_RNG_INDEX == 24 {
                0x676bcb2a1d2948c7
            }
            // 11691956
            else if SAIL_RNG_INDEX == 25 {
                0x3186917ec583a634
            }
            // 11691979
            else if SAIL_RNG_INDEX == 26 {
                0x96499244e2f99636
            }
            // 11692002
            else if SAIL_RNG_INDEX == 27 {
                0xd98128edbf856c8b
            }
            // 11703871
            else if SAIL_RNG_INDEX == 28 {
                0xb32f381ed1dd4308
            }
            // 11703894
            else if SAIL_RNG_INDEX == 29 {
                0x598592d8dfd2e979
            }
            // 11703917
            else if SAIL_RNG_INDEX == 30 {
                0x4fe923ed922acbaa
            }
            // 11703940
            else if SAIL_RNG_INDEX == 31 {
                0xd328ebae7d76e01d
            }
            // 11739099
            else if SAIL_RNG_INDEX == 32 {
                0x474b244fb9c6b9be
            }
            // 11739122
            else if SAIL_RNG_INDEX == 33 {
                0xfc78922536ceed7e
            }
            // 11739145
            else if SAIL_RNG_INDEX == 34 {
                0x099cc02109507e6e
            }
            // 11739168
            else if SAIL_RNG_INDEX == 35 {
                0xd162477f78dbe50c
            }
            // 11751037
            else if SAIL_RNG_INDEX == 36 {
                0x4e233d90f70197d4
            }
            // 11751060
            else if SAIL_RNG_INDEX == 37 {
                0x7e1de3f6c6ca3f95
            }
            // 11751083
            else if SAIL_RNG_INDEX == 38 {
                0x8bc992933f2c3e0e
            }
            // 11751106
            else if SAIL_RNG_INDEX == 39 {
                0x6c7af608c97ed68a
            }
            // 11788621
            else if SAIL_RNG_INDEX == 40 {
                0x22de7f9e8964f904
            }
            // 11788644
            else if SAIL_RNG_INDEX == 41 {
                0xc5f3936903478f97
            }
            // 11788667
            else if SAIL_RNG_INDEX == 42 {
                0xddbdfc075fd6a687
            }
            // 11788690
            else if SAIL_RNG_INDEX == 43 {
                0xe7d99ae1cc225df9
            }
            // 11800559
            else if SAIL_RNG_INDEX == 44 {
                0x91a320f9b5652a3d
            }
            // 11800582
            else if SAIL_RNG_INDEX == 45 {
                0x3261647d2249704a
            }
            // 11800605
            else if SAIL_RNG_INDEX == 46 {
                0x2f44a7f12e5bfb33
            }
            // 11800628
            else if SAIL_RNG_INDEX == 47 {
                0x437028357b346589
            }
            // 11838143
            else if SAIL_RNG_INDEX == 48 {
                0x2c08732532c76143
            }
            // 11838166
            else if SAIL_RNG_INDEX == 49 {
                0x2f958a0395a22961
            }
            // 11838189
            else if SAIL_RNG_INDEX == 50 {
                0x4241a06ecf25d426
            }
            // 11838212
            else if SAIL_RNG_INDEX == 51 {
                0xa5351284095e140c
            }
            // 11850081
            else if SAIL_RNG_INDEX == 52 {
                0xc8a6a6cd0caafb2b
            }
            // 11850104
            else if SAIL_RNG_INDEX == 53 {
                0xfa95d107e3565a99
            }
            // 11850127
            else if SAIL_RNG_INDEX == 54 {
                0xa6302a4f98adb0af
            }
            // 11850150
            else if SAIL_RNG_INDEX == 55 {
                0x3721bf7127e07585
            }
            // 11885309
            else if SAIL_RNG_INDEX == 56 {
                0xa31a5932d0ca98fb
            }
            // 11885332
            else if SAIL_RNG_INDEX == 57 {
                0xea22264e09433885
            }
            // 11885355
            else if SAIL_RNG_INDEX == 58 {
                0x558da90df31436fb
            }
            // 11885378
            else if SAIL_RNG_INDEX == 59 {
                0xc031f3f4936266fb
            }
            // 11897247
            else if SAIL_RNG_INDEX == 60 {
                0x16dc60b28dbeb555
            }
            // 11897270
            else if SAIL_RNG_INDEX == 61 {
                0x0788642be82a1216
            }
            // 11897293
            else if SAIL_RNG_INDEX == 62 {
                0xf62d3db2a5dff06d
            }
            // 11897316
            else if SAIL_RNG_INDEX == 63 {
                0x25c50c249615f0d1
            }
            // 11934831
            else if SAIL_RNG_INDEX == 64 {
                0x4b4b359d16bb187b
            }
            // 11934854
            else if SAIL_RNG_INDEX == 65 {
                0x6e9c3c02c772efa3
            }
            // 11934877
            else if SAIL_RNG_INDEX == 66 {
                0xaa298dfc63fa5ecc
            }
            // 11934900
            else if SAIL_RNG_INDEX == 67 {
                0xbdee101d6d559df1
            }
            // 11946769
            else if SAIL_RNG_INDEX == 68 {
                0x1b614d0c2c11d33d
            }
            // 11946792
            else if SAIL_RNG_INDEX == 69 {
                0xaf55cbd9f81250a0
            }
            // 11946815
            else if SAIL_RNG_INDEX == 70 {
                0x9ba377891fd398cf
            }
            // 11946838
            else if SAIL_RNG_INDEX == 71 {
                0x1d7a9580d8d701ec
            }
            // 11981997
            else if SAIL_RNG_INDEX == 72 {
                0x15d2e31fef270bc0
            }
            // 11982020
            else if SAIL_RNG_INDEX == 73 {
                0x53a39387875e4003
            }
            // 11982043
            else if SAIL_RNG_INDEX == 74 {
                0xe05b1ea14a41f804
            }
            // 11982066
            else if SAIL_RNG_INDEX == 75 {
                0xd92def102810b104
            }
            // 11996291
            else if SAIL_RNG_INDEX == 76 {
                0x8cbb1559793049d6
            }
            // 11996314
            else if SAIL_RNG_INDEX == 77 {
                0x8aa9ddebf8be5c08
            }
            // 11996337
            else if SAIL_RNG_INDEX == 78 {
                0x4209e7626aca9938
            }
            // 11996360
            else if SAIL_RNG_INDEX == 79 {
                0x94e14a0010b39b55
            }
            // 12031519
            else if SAIL_RNG_INDEX == 80 {
                0xfc94b3d8beaefeb7
            }
            // 12031542
            else if SAIL_RNG_INDEX == 81 {
                0xb009a0ccabdeb2e9
            }
            // 12031565
            else if SAIL_RNG_INDEX == 82 {
                0x1af156cf4c009c09
            }
            // 12031588
            else if SAIL_RNG_INDEX == 83 {
                0xf96459ce16c51cea
            }
            // 12043457
            else if SAIL_RNG_INDEX == 84 {
                0x31ffcad6670ec639
            }
            // 12043480
            else if SAIL_RNG_INDEX == 85 {
                0xf1445d189729f1f5
            }
            // 12043503
            else if SAIL_RNG_INDEX == 86 {
                0x45bcfc1f0e107184
            }
            // 12043526
            else if SAIL_RNG_INDEX == 87 {
                0xb31097267307f6e1
            }
            // 12081041
            else if SAIL_RNG_INDEX == 88 {
                0x0bbc42fda4975ac0
            }
            // 12081064
            else if SAIL_RNG_INDEX == 89 {
                0x88818cfadeff9149
            }
            // 12081087
            else if SAIL_RNG_INDEX == 90 {
                0xa4bee359f40c2f58
            }
            // 12081110
            else if SAIL_RNG_INDEX == 91 {
                0xb4a972fe357cbad4
            }
            // 12092979
            else if SAIL_RNG_INDEX == 92 {
                0x9d3019510f1843b5
            }
            // 12093002
            else if SAIL_RNG_INDEX == 93 {
                0x7c1126090c6a5ab6
            }
            // 12093025
            else if SAIL_RNG_INDEX == 94 {
                0xc09883ad79f715d3
            }
            // 12093048
            else if SAIL_RNG_INDEX == 95 {
                0x4abefe2a78c03976
            }
            // 12128207
            else if SAIL_RNG_INDEX == 96 {
                0xc9606e4ac29b41c8
            }
            // 12128230
            else if SAIL_RNG_INDEX == 97 {
                0x92cd8ecbb3f05bc6
            }
            // 12128253
            else if SAIL_RNG_INDEX == 98 {
                0xdd1e0a29098083d2
            }
            // 12128276
            else if SAIL_RNG_INDEX == 99 {
                0x5f879b6f5be3d044
            }
            // 12140145
            else if SAIL_RNG_INDEX == 100 {
                0xbc11b00e08636704
            }
            // 12140168
            else if SAIL_RNG_INDEX == 101 {
                0x2a8aa7d3afe7b292
            }
            // 12140191
            else if SAIL_RNG_INDEX == 102 {
                0x2cfc3a8bf2b3bfe4
            }
            // 12140214
            else if SAIL_RNG_INDEX == 103 {
                0xba8bf3284e1cadd2
            }
            // 12177729
            else if SAIL_RNG_INDEX == 104 {
                0xc2371c7e9aa72e61
            }
            // 12177752
            else if SAIL_RNG_INDEX == 105 {
                0xdd2688c5639f971f
            }
            // 12177775
            else if SAIL_RNG_INDEX == 106 {
                0x61cb1e29d8790e57
            }
            // 12177798
            else if SAIL_RNG_INDEX == 107 {
                0x602cdadc59fc8bd4
            }
            // 12189667
            else if SAIL_RNG_INDEX == 108 {
                0xafff63403f90f937
            }
            // 12189690
            else if SAIL_RNG_INDEX == 109 {
                0xa6645a064465c651
            }
            // 12189713
            else if SAIL_RNG_INDEX == 110 {
                0x5ba811fa006b3e9c
            }
            // 12189736
            else if SAIL_RNG_INDEX == 111 {
                0xde5a8d4907cb9bb4
            }
            // 12224895
            else if SAIL_RNG_INDEX == 112 {
                0x1fd91f60f05acdd2
            }
            // 12224918
            else if SAIL_RNG_INDEX == 113 {
                0xe8a188b03e20ac17
            }
            // 12224941
            else if SAIL_RNG_INDEX == 114 {
                0x5d6010a46a7dc7dc
            }
            // 12224964
            else if SAIL_RNG_INDEX == 115 {
                0x25823de46572d3ce
            }
            // 12239189
            else if SAIL_RNG_INDEX == 116 {
                0xe072ed8a2ea04281
            }
            // 12239212
            else if SAIL_RNG_INDEX == 117 {
                0x31ae35331d5c1875
            }
            // 12239235
            else if SAIL_RNG_INDEX == 118 {
                0x8cba0325fc07cbaf
            }
            // 12239258
            else if SAIL_RNG_INDEX == 119 {
                0x94b20bf22b9d2c12
            }
            // 12274417
            else if SAIL_RNG_INDEX == 120 {
                0x251564bdfe388be6
            }
            // 12274440
            else if SAIL_RNG_INDEX == 121 {
                0xf0eded596495d234
            }
            // 12274463
            else if SAIL_RNG_INDEX == 122 {
                0x61c7b410dcc7ee6a
            }
            // 12274486
            else if SAIL_RNG_INDEX == 123 {
                0x77edd28b9d5ad1f6
            }
            // 12286355
            else if SAIL_RNG_INDEX == 124 {
                0x5172d90532d28218
            }
            // 12286378
            else if SAIL_RNG_INDEX == 125 {
                0xb280c21897178e70
            }
            // 12286401
            else if SAIL_RNG_INDEX == 126 {
                0xb1e1deef0cc312e8
            }
            // 12286424
            else if SAIL_RNG_INDEX == 127 {
                0x868882eb41cd065f
            }
            // 12323939
            else if SAIL_RNG_INDEX == 128 {
                0x5c727f3966a8b7f8
            }
            // 12323962
            else if SAIL_RNG_INDEX == 129 {
                0x7acee7646ee8ede7
            }
            // 12323985
            else if SAIL_RNG_INDEX == 130 {
                0x78869a678240afa7
            }
            // 12324008
            else if SAIL_RNG_INDEX == 131 {
                0x57a2a8c690c926da
            }
            // 12335877
            else if SAIL_RNG_INDEX == 132 {
                0x7898f765a06f3d71
            }
            // 12335900
            else if SAIL_RNG_INDEX == 133 {
                0xc0c96ad7429fd7ff
            }
            // 12335923
            else if SAIL_RNG_INDEX == 134 {
                0xd3b2d21f5e4080f3
            }
            // 12335946
            else if SAIL_RNG_INDEX == 135 {
                0x2ae35faa87e00f3a
            }
            // 12371105
            else if SAIL_RNG_INDEX == 136 {
                0x0c50c13cde7dc1ac
            }
            // 12371128
            else if SAIL_RNG_INDEX == 137 {
                0x9cb2e9089aaa43a5
            }
            // 12371151
            else if SAIL_RNG_INDEX == 138 {
                0xadd36724fd05443e
            }
            // 12371174
            else if SAIL_RNG_INDEX == 139 {
                0x4a5f6951d36ad776
            }
            // 12383043
            else if SAIL_RNG_INDEX == 140 {
                0x85cdf2dd89f5a780
            }
            // 12383066
            else if SAIL_RNG_INDEX == 141 {
                0xd5dbb4ade4b5ebd1
            }
            // 12383089
            else if SAIL_RNG_INDEX == 142 {
                0xaf9df4c8134907d5
            }
            // 12383112
            else if SAIL_RNG_INDEX == 143 {
                0x50e9d853557422d0
            }
            // 12420627
            else if SAIL_RNG_INDEX == 144 {
                0xa3c106efb52cef2a
            }
            // 12420650
            else if SAIL_RNG_INDEX == 145 {
                0xf8e44b43bb301e39
            }
            // 12420673
            else if SAIL_RNG_INDEX == 146 {
                0x61fdce0886fbb929
            }
            // 12420696
            else if SAIL_RNG_INDEX == 147 {
                0xc3034fcb912ed0a8
            }
            // 12432565
            else if SAIL_RNG_INDEX == 148 {
                0xf307955fe93ad83d
            }
            // 12432588
            else if SAIL_RNG_INDEX == 149 {
                0x3a8a145c389ccdb3
            }
            // 12432611
            else if SAIL_RNG_INDEX == 150 {
                0x12fd28f29683249f
            }
            // 12432634
            else if SAIL_RNG_INDEX == 151 {
                0x2232187391e66725
            }
            // 12467793
            else if SAIL_RNG_INDEX == 152 {
                0xa43f489535fdab19
            }
            // 12467816
            else if SAIL_RNG_INDEX == 153 {
                0xe1696aced53cfaca
            }
            // 12467839
            else if SAIL_RNG_INDEX == 154 {
                0x4d8d278be4de7800
            }
            // 12467862
            else if SAIL_RNG_INDEX == 155 {
                0x08df23e5e7b6bcc5
            }
            // 12482087
            else if SAIL_RNG_INDEX == 156 {
                0x32ed41ed5358bd13
            }
            // 12482110
            else if SAIL_RNG_INDEX == 157 {
                0x875d58b4748bd249
            }
            // 12482133
            else if SAIL_RNG_INDEX == 158 {
                0x0a8018d98899d5a4
            }
            // 12482156
            else if SAIL_RNG_INDEX == 159 {
                0xc7090be30ac1ae74
            }
            // 12517315
            else if SAIL_RNG_INDEX == 160 {
                0x96f5d9a743600ee1
            }
            // 12517338
            else if SAIL_RNG_INDEX == 161 {
                0xaf88c3e8f18da9ab
            }
            // 12517361
            else if SAIL_RNG_INDEX == 162 {
                0x64a6df5e4d4f3e2a
            }
            // 12517384
            else if SAIL_RNG_INDEX == 163 {
                0xbf942d50fa14b011
            }
            // 12529253
            else if SAIL_RNG_INDEX == 164 {
                0x7f4eb4fd40bf0e74
            }
            // 12529276
            else if SAIL_RNG_INDEX == 165 {
                0x4c7719325fe98fe9
            }
            // 12529299
            else if SAIL_RNG_INDEX == 166 {
                0x676c0f1271c32ab9
            }
            // 12529322
            else if SAIL_RNG_INDEX == 167 {
                0xee84078dd86e193c
            }
            // 12566837
            else if SAIL_RNG_INDEX == 168 {
                0xe2ff4b8791341e0a
            }
            // 12566860
            else if SAIL_RNG_INDEX == 169 {
                0x59df82dca02af463
            }
            // 12566883
            else if SAIL_RNG_INDEX == 170 {
                0x4f535e369a853a04
            }
            // 12566906
            else if SAIL_RNG_INDEX == 171 {
                0x2159ce8e81e390c2
            }
            // 12578775
            else if SAIL_RNG_INDEX == 172 {
                0x0e7409f648f12334
            }
            // 12578798
            else if SAIL_RNG_INDEX == 173 {
                0x363737076cdecd64
            }
            // 12578821
            else if SAIL_RNG_INDEX == 174 {
                0x76230d84f8e30993
            }
            // 12578844
            else if SAIL_RNG_INDEX == 175 {
                0xaa32af6ded869308
            }
            // 12614003
            else if SAIL_RNG_INDEX == 176 {
                0x0a497324e1697ebe
            }
            // 12614026
            else if SAIL_RNG_INDEX == 177 {
                0x475e7d4c6d305e99
            }
            // 12614049
            else if SAIL_RNG_INDEX == 178 {
                0xd8f9c858bc715f9d
            }
            // 12614072
            else if SAIL_RNG_INDEX == 179 {
                0xab61bb8d5fd013f0
            }
            // 12625941
            else if SAIL_RNG_INDEX == 180 {
                0xe2727b6eb44dcda9
            }
            // 12625964
            else if SAIL_RNG_INDEX == 181 {
                0xc942df1da4c19192
            }
            // 12625987
            else if SAIL_RNG_INDEX == 182 {
                0xe36408099d57d522
            }
            // 12626010
            else if SAIL_RNG_INDEX == 183 {
                0x5a3f511c2e88bc89
            }
            // 12663525
            else if SAIL_RNG_INDEX == 184 {
                0xc79ef326506ef440
            }
            // 12663548
            else if SAIL_RNG_INDEX == 185 {
                0x810dd465704362c0
            }
            // 12663571
            else if SAIL_RNG_INDEX == 186 {
                0xe4483e4e146307c7
            }
            // 12663594
            else if SAIL_RNG_INDEX == 187 {
                0x55ce348ab2377a4b
            }
            // 12675463
            else if SAIL_RNG_INDEX == 188 {
                0x60b4c2d9fd1fdef8
            }
            // 12675486
            else if SAIL_RNG_INDEX == 189 {
                0x9e04727e883411ed
            }
            // 12675509
            else if SAIL_RNG_INDEX == 190 {
                0xe8aa4f6c66f7b78b
            }
            // 12675532
            else if SAIL_RNG_INDEX == 191 {
                0x3ca9c78764f787b3
            }
            // 12710691
            else if SAIL_RNG_INDEX == 192 {
                0xb40d8b59715ecad5
            }
            // 12710714
            else if SAIL_RNG_INDEX == 193 {
                0x4aeb06a61183c5b5
            }
            // 12710737
            else if SAIL_RNG_INDEX == 194 {
                0x6fa6d0afe7d417cc
            }
            // 12710760
            else if SAIL_RNG_INDEX == 195 {
                0x21e8e3d410e33955
            }
            // 12724985
            else if SAIL_RNG_INDEX == 196 {
                0x0cb09c439cca1b7e
            }
            // 12725008
            else if SAIL_RNG_INDEX == 197 {
                0x76b73e930447ef94
            }
            // 12725031
            else if SAIL_RNG_INDEX == 198 {
                0xa78f42855efd2ec9
            }
            // 12725054
            else if SAIL_RNG_INDEX == 199 {
                0xf1efbe7164622d8e
            }
            // 12760213
            else if SAIL_RNG_INDEX == 200 {
                0x8fed9ef9f09be297
            }
            // 12760236
            else if SAIL_RNG_INDEX == 201 {
                0x0c5f6489dec90308
            }
            // 12760259
            else if SAIL_RNG_INDEX == 202 {
                0xc66ad53fba5aa922
            }
            // 12760282
            else if SAIL_RNG_INDEX == 203 {
                0xbf365783f97b271e
            }
            // 12772151
            else if SAIL_RNG_INDEX == 204 {
                0x65cf2055be88a620
            }
            // 12772174
            else if SAIL_RNG_INDEX == 205 {
                0x770c2b4897bd2397
            }
            // 12772197
            else if SAIL_RNG_INDEX == 206 {
                0x52e529b097864a70
            }
            // 12772220
            else if SAIL_RNG_INDEX == 207 {
                0xc3bd96025bb6b362
            }
            // 12809735
            else if SAIL_RNG_INDEX == 208 {
                0x7ce8030337b91d62
            }
            // 12809758
            else if SAIL_RNG_INDEX == 209 {
                0x8ac768aeaad2753b
            }
            // 12809781
            else if SAIL_RNG_INDEX == 210 {
                0x6f71cb495eabdfd3
            }
            // 12809804
            else if SAIL_RNG_INDEX == 211 {
                0xc06a8e83c7d064fc
            }
            // 12821673
            else if SAIL_RNG_INDEX == 212 {
                0x437b573c066fb103
            }
            // 12821696
            else if SAIL_RNG_INDEX == 213 {
                0x6ec164b9685a4dc2
            }
            // 12821719
            else if SAIL_RNG_INDEX == 214 {
                0x6a70f3c6bfa95382
            }
            // 12821742
            else if SAIL_RNG_INDEX == 215 {
                0x1739223134520dbc
            }
            // 12856901
            else if SAIL_RNG_INDEX == 216 {
                0x04781d6b8f3750b7
            }
            // 12856924
            else if SAIL_RNG_INDEX == 217 {
                0xf1e750dfeda9662a
            }
            // 12856947
            else if SAIL_RNG_INDEX == 218 {
                0x86c038eea8dbaa08
            }
            // 12856970
            else if SAIL_RNG_INDEX == 219 {
                0x1a5785b31fe8f4e2
            }
            // 12868839
            else if SAIL_RNG_INDEX == 220 {
                0x0da0c6acf6420810
            }
            // 12868862
            else if SAIL_RNG_INDEX == 221 {
                0x6fe0e57d9a9a9a10
            }
            // 12868885
            else if SAIL_RNG_INDEX == 222 {
                0xb9d57101b3865017
            }
            // 12868908
            else if SAIL_RNG_INDEX == 223 {
                0xfc813c7b4179d597
            }
            // 12906423
            else if SAIL_RNG_INDEX == 224 {
                0xc7f184282f4c1074
            }
            // 12906446
            else if SAIL_RNG_INDEX == 225 {
                0xdb48bc60f7393943
            }
            // 12906469
            else if SAIL_RNG_INDEX == 226 {
                0x72940247736aa46a
            }
            // 12906492
            else if SAIL_RNG_INDEX == 227 {
                0xe078de6ad7a73fc8
            }
            // 12918361
            else if SAIL_RNG_INDEX == 228 {
                0xfac816500bb512c4
            }
            // 12918384
            else if SAIL_RNG_INDEX == 229 {
                0x3a7b42343014e5c6
            }
            // 12918407
            else if SAIL_RNG_INDEX == 230 {
                0xdab089eb3486aea7
            }
            // 12918430
            else if SAIL_RNG_INDEX == 231 {
                0xdd51663d2685bfe8
            }
            // 12955945
            else if SAIL_RNG_INDEX == 232 {
                0xda82bb3f34329363
            }
            // 12955968
            else if SAIL_RNG_INDEX == 233 {
                0x884bcdafb05664bd
            }
            // 12955991
            else if SAIL_RNG_INDEX == 234 {
                0xa40968899dc87633
            }
            // 12956014
            else if SAIL_RNG_INDEX == 235 {
                0xfc19197eb73b97bc
            }
            // 12967883
            else if SAIL_RNG_INDEX == 236 {
                0xbd793656ada4a912
            }
            // 12967906
            else if SAIL_RNG_INDEX == 237 {
                0x42ee4291bfdf642a
            }
            // 12967929
            else if SAIL_RNG_INDEX == 238 {
                0xde00192562038197
            }
            // 12967952
            else if SAIL_RNG_INDEX == 239 {
                0x1fd32675a88a2152
            }
            // 13003111
            else if SAIL_RNG_INDEX == 240 {
                0xb5021c45fbb76f7b
            }
            // 13003134
            else if SAIL_RNG_INDEX == 241 {
                0x56623376179eb395
            }
            // 13003157
            else if SAIL_RNG_INDEX == 242 {
                0x6b7d141a138a46bd
            }
            // 13003180
            else if SAIL_RNG_INDEX == 243 {
                0x48912d613c90f7d1
            }
            // 13015049
            else if SAIL_RNG_INDEX == 244 {
                0x9b7573d2d2973ea7
            }
            // 13015072
            else if SAIL_RNG_INDEX == 245 {
                0xd1002581ab5f3c8a
            }
            // 13015095
            else if SAIL_RNG_INDEX == 246 {
                0x98de18e6ec927b8f
            }
            // 13015118
            else if SAIL_RNG_INDEX == 247 {
                0xb1c83471f2b05adc
            }
            // 13052633
            else if SAIL_RNG_INDEX == 248 {
                0x95a1a5a5450d3d21
            }
            // 13052656
            else if SAIL_RNG_INDEX == 249 {
                0x36b39eb24118e1f3
            }
            // 13052679
            else if SAIL_RNG_INDEX == 250 {
                0x4eb90b6b18d6367c
            }
            // 13052702
            else if SAIL_RNG_INDEX == 251 {
                0x025d32da2684867d
            }
            // 13064571
            else if SAIL_RNG_INDEX == 252 {
                0x570ddbec8cbd28e6
            }
            // 13064594
            else if SAIL_RNG_INDEX == 253 {
                0x275fb812d79e6052
            }
            // 13064617
            else if SAIL_RNG_INDEX == 254 {
                0x9f2d9da6b4274185
            }
            // 13064640
            else if SAIL_RNG_INDEX == 255 {
                0x03decab155b5000c
            }
            // 13099799
            else if SAIL_RNG_INDEX == 256 {
                0xbb4d2f730a19f8d4
            }
            // 13099822
            else if SAIL_RNG_INDEX == 257 {
                0x856df9047e6f88de
            }
            // 13099845
            else if SAIL_RNG_INDEX == 258 {
                0xd4f3804de4af1fea
            }
            // 13099868
            else if SAIL_RNG_INDEX == 259 {
                0x8a6ae524900d3a52
            }
            // 13111737
            else if SAIL_RNG_INDEX == 260 {
                0x8cd26c3606b72ec6
            }
            // 13111760
            else if SAIL_RNG_INDEX == 261 {
                0xad776ad55c043c78
            }
            // 13111783
            else if SAIL_RNG_INDEX == 262 {
                0xf1ba4ba6b4601160
            }
            // 13111806
            else if SAIL_RNG_INDEX == 263 {
                0x43ebc7dad66c21b7
            }
            // 24513701
            else if SAIL_RNG_INDEX == 264 {
                0xeb1e4a4045b812f5
            }
            // 24513724
            else if SAIL_RNG_INDEX == 265 {
                0x311a88a1b0fd0222
            }
            // 24513747
            else if SAIL_RNG_INDEX == 266 {
                0x53ceba2220c5d401
            }
            // 24513770
            else if SAIL_RNG_INDEX == 267 {
                0x2bce9d8b72486587
            }
            // 24522973
            else if SAIL_RNG_INDEX == 268 {
                0xad0adf7fde6863cd
            }
            // 24522996
            else if SAIL_RNG_INDEX == 269 {
                0xda7d032e9dda1cd2
            }
            // 24523019
            else if SAIL_RNG_INDEX == 270 {
                0x69bc88bbeff9c765
            }
            // 24523042
            else if SAIL_RNG_INDEX == 271 {
                0x3da62fbf466fb176
            }
            // 24532179
            else if SAIL_RNG_INDEX == 272 {
                0xad3ff571d0bedc53
            }
            // 24532202
            else if SAIL_RNG_INDEX == 273 {
                0x6053a326bc08480e
            }
            // 24532225
            else if SAIL_RNG_INDEX == 274 {
                0xcf41ab01269a3f5e
            }
            // 24532248
            else if SAIL_RNG_INDEX == 275 {
                0x4995ed116d962f54
            }
            // 24549804
            else if SAIL_RNG_INDEX == 276 {
                0x1796d6263fadcbc4
            }
            // 24549827
            else if SAIL_RNG_INDEX == 277 {
                0x1e15b0865007a8b9
            }
            // 24549850
            else if SAIL_RNG_INDEX == 278 {
                0xed97448779899cc3
            }
            // 24549873
            else if SAIL_RNG_INDEX == 279 {
                0x6720bd96353a7021
            }
            // 24563024
            else if SAIL_RNG_INDEX == 280 {
                0xa173116ebdc285d9
            }
            // 24563047
            else if SAIL_RNG_INDEX == 281 {
                0x2ce6a7e92bc2b183
            }
            // 24563070
            else if SAIL_RNG_INDEX == 282 {
                0xf13545ac3d3fdb73
            }
            // 24563093
            else if SAIL_RNG_INDEX == 283 {
                0x77c36ae05057d707
            }
            // 24573291
            else if SAIL_RNG_INDEX == 284 {
                0x3de62c9075bb7ee7
            }
            // 24573314
            else if SAIL_RNG_INDEX == 285 {
                0x704b25186341f94b
            }
            // 24573337
            else if SAIL_RNG_INDEX == 286 {
                0x5d8a49e78c818f99
            }
            // 24573360
            else if SAIL_RNG_INDEX == 287 {
                0xcb092189413516a2
            } else {
                panic!("Unsupported RNG!");
            };
            SAIL_RNG_INDEX += 1;
            return ret;
        }
    }
}
