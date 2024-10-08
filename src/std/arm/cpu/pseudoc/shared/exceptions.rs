use crate::std::arm::cpu::ArmCtx;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    Uncategorized,
    WFxTrap,
    CP15RTTrap,
    CP15RRTTrap,
    CP14RTTrap,
    CP14DTTrap,
    CP14RRTTrap,
    AdvSIMDFPAccessTrap,
    FPIDTrap,
    LDST64BTrap,
    PACTrap,
    IllegalState,
    SupervisorCall,
    HypervisorCall,
    MonitorCall,
    SystemRegisterTrap,
    ERetTrap,
    InstructionAbort,
    PCAlignment,
    DataAbort,
    NV2DataAbort,
    PACFail,
    SPAlignment,
    FPTrappedException,
    SError,
    Breakpoint,
    SoftwareStep,
    Watchpoint,
    NV2Watchpoint,
    SoftwareBreakpoint,
    VectorCatch,
    IRQ,
    SVEAccessTrap,
    SMEAccessTrap,
    TSTARTAccessTrap,
    GPC,
    BranchTarget,
    MemCpyMemSet,
    SystemRegister128Trap,
    FIQ,
}

#[derive(Clone)]
pub struct ExceptionRecord {
    pub exceptype: Exception, // Exception class
    pub syndrome: u64,        // Syndrome record
    // syndrome2: u64,           // Syndrome record
    // paddress: FullAddress,    // Physical fault address
    // vaddress: u64,            // Virtual fault address
    // ipavalid: bool,           // Validity of Intermediate Physical fault address
    // pavalid: bool,            // Validity of Physical fault address
    // ns: bool,                 // Intermediate Physical fault address space
    // ipaddress: u64,           // Intermediate Physical fault address
    pub trappedsyscallinst: bool, // Trapped SVC or SMC instruction
}

impl ExceptionRecord {
    pub fn exception_syndrome(exceptype: Exception) -> Self {
        Self {
            exceptype,
            syndrome: 0,
            // syndrome2: 0,
            // vaddress:  0,
            // ipavalid:  false,
            // NS:        false,
            // ipaddress: 0,
            // paddress.paspace = PASpace UNKNOWN;
            // paddress.address = bits(56) UNKNOWN;
            trappedsyscallinst: false,
        }
    }
}

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn condition_syndrome(&mut self) -> u64 {
        if self.using_aarch32() {
            todo!("cond = AArch32.CurrentCond();");
        } else {
            return 0b11110;
        }
    }
}
