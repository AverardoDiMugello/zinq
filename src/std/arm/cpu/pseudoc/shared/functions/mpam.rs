#[derive(Clone, Copy)]
pub enum PARTIDSpaceType {
    Secure,
    Root,
    Realm,
    NonSecure,
}

#[derive(Clone, Copy)]
pub struct MPAMInfo {
    pub mpam_sp: PARTIDSpaceType,
    pub partid: PARTIDType,
    pub pmg: PMGType,
}

pub type PARTIDType = u64;
pub type PMGType = u64;
