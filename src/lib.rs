#![feature(bigint_helper_methods)]
#![feature(generic_const_exprs)]
#![feature(ascii_char)]

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
