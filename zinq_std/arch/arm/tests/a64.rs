use zinq::{
    system::{Processor, System},
    Emulator,
};

use zinq_std_arm as znstd_arm;
use znstd_arm::{Arm, Version::Armv9p4a};

use zinq_std_emu as znstd_emu;
use znstd_emu::StepEmu;

#[test]
fn uncond_branch() {
    let test_case = [
        0x01, 0x04, 0x00, 0x91, // 0:   ADD X1, X0, #1
        0x21, 0x04, 0x00, 0x91, // 4:   ADD X1, X1, #1
        0x02, 0x00, 0x00, 0x14, // 8:   B #8
        0x21, 0x04, 0x00, 0x91, // 12:  ADD X1, X1, #1
        0x00, 0x04, 0x00, 0x91, // 16:  ADD X0, X0, #1
    ];

    let mut vm = System::new(Arm::new(Armv9p4a), 48);
    vm.write_mem(0, &test_case).unwrap();
    vm.proc_mut().set_ip(0);

    let mut emulator = StepEmu::new();
    emulator.max_insns(4);
    emulator.run(&mut vm);

    assert_eq!(vm.proc().ip(), 20);
    assert_eq!(vm.proc().x(0).unwrap(), 1);
    assert_eq!(vm.proc().x(1).unwrap(), 2);
}

#[test]
fn cond_branch() {
    let test_case = [
        0x02, 0x04, 0x00, 0x91, // 0:   ADD X2, X0, #1
        0x23, 0x08, 0x00, 0x91, // 4:   ADD X3, X1, #2
        0x7F, 0x04, 0x00, 0xF1, // 8:   CMP X3, #1
        0x4C, 0x00, 0x00, 0x54, // 12:  B.GT 0x8
        0x44, 0x00, 0x80, 0xD2, // 16:  MOV X4, #2
        0x64, 0x00, 0x80, 0xD2, // 20:  MOV X4, #3
    ];

    let mut vm = System::new(Arm::new(Armv9p4a), 48);
    vm.write_mem(0, &test_case).unwrap();
    vm.proc_mut().set_ip(0);

    let mut emulator = StepEmu::new();
    emulator.max_insns(5);
    emulator.run(&mut vm);

    assert_eq!(vm.proc().ip(), 24); // TODO: because the insn increments the pc... idk maybe this is fine
    assert_eq!(vm.proc().x(4).unwrap(), 3);

    // The second half of this is a good test case! Come back when zinq_macros is written
    // vm.proc_mut().set_x(0, 1).unwrap();
    // vm.proc_mut().set_ip(0);

    // let mut emulator = StepEmu::new();
    // let vm = emulator.run(vm, Some(|icount| icount >= 5));

    // assert_eq!(vm.proc().ip(), 16);
    // assert_eq!(vm.proc().x(4).unwrap(), 2);
}

#[test]
fn cond_branch_32() {
    let test_case = [
        0x02, 0x04, 0x00, 0x11, // ADD W2, W0, #1
        0x23, 0x08, 0x00, 0x11, // ADD W3, W1, #2
        0x7F, 0x04, 0x00, 0x71, // CMP W3, #1
        0xEC, 0xFF, 0xFF, 0x54, // B.GT 0x8
        0x44, 0x00, 0x80, 0x52, // MOV W4, #2
        0x64, 0x00, 0x80, 0x52, // MOV W4, #3
    ];

    let mut vm = System::new(Arm::new(Armv9p4a), 48);
    vm.write_mem(0, &test_case).unwrap();
    vm.proc_mut().set_ip(0);
    vm.proc_mut().set_x(0, ((1 << 32) - 1) << 31).unwrap();
    vm.proc_mut().set_x(1, ((1 << 32) - 1) << 31).unwrap();

    let mut emulator = StepEmu::new();
    emulator.max_insns(5);
    emulator.run(&mut vm);

    assert_eq!(vm.proc().ip(), 20);
    assert_eq!(vm.proc().x(2).unwrap(), (((1 << 32) - 1) << 31) + 1);
    assert_eq!(vm.proc().x(3).unwrap(), (((1 << 32) - 1) << 31) + 2);

    // The second half of this is a good test case! Come back when zinq_macros is written
    // vm.proc_mut().set_x(0, 1).unwrap();
    // vm.proc_mut().set_ip(0);

    // let mut emulator = StepEmu::new();
    // let vm = emulator.run(vm, Some(|icount| icount >= 5));

    // assert_eq!(vm.proc().ip(), 16);
    // assert_eq!(vm.proc().x(4).unwrap(), 2);
}
