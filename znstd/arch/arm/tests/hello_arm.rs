use zinq::system::{Processor, System};

use zinq_std_arm as znstd_arm;
use znstd_arm::Arm;

use zinq_std_emu as znstd_emu;
use znstd_emu::StepEmu;

#[test]
fn arith_imm_branch() {
    let mut vm = System::new(Arm::v8(), 48);

    /*
     * Map program into memory
     */
    let test_case = [
        0x01, 0x04, 0x00, 0x91, // 0:   ADD X1, X0, #1
        0x21, 0x04, 0x00, 0x91, // 4:   ADD X1, X1, #1
        0x02, 0x00, 0x00, 0x14, // 8:   B #8
        0x21, 0x04, 0x00, 0x91, // 12:  ADD X1, X1, #1
        0x00, 0x04, 0x00, 0x91, // 16:  ADD X0, X0, #1
    ];

    vm.mem_mut()
        .get_mut(0..20)
        .unwrap()
        .copy_from_slice(&test_case);
    vm.proc_mut().set_ip(0);

    /*
     * Single-step emulation
     */
    let mut emu = StepEmu::new();
    let vm = emu.run(vm);

    assert_eq!(vm.proc().ip(), 20);
    assert_eq!(vm.proc().x(0).unwrap(), 1);
    assert_eq!(vm.proc().x(1).unwrap(), 2);
}
