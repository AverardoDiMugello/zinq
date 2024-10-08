use crate::std::arm::cpu::registers::PC;
use crate::std::arm::cpu::ArmCtx;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl<'cpu, 'ctx, 'a: 'cpu + 'ctx> ArmCtx<'cpu, 'ctx, 'a> {
    pub fn branch_to(
        &mut self,
        target: usize,
        _branch_type: BranchType,
        _branch_conditional: bool,
    ) {
        // TODO: BRBE and SPE
        // println!("Branching to 0x{target:x}.");
        self.write::<PC>(target as u64);
    }

    pub fn branch_to_addr(&mut self, target: usize, _branch_type: BranchType) {
        // println!("Branching to 0x{target:x}.");
        self.write::<PC>(target as u64);
    }

    pub fn branch_not_taken(&mut self, _branch_type: BranchType, _branch_conditional: bool) {
        // TODO: SPE
    }
}
