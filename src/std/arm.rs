pub mod cpu;
pub mod intc;
pub mod serial;
pub mod timer;

pub use {
    cpu::{registers::*, ArmCpu, EL},
    intc::Gicv2,
    serial::PL011,
    timer::GTimer,
};
