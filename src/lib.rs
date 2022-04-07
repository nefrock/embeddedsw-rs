#![no_std]
#![feature(alloc_error_handler)]

pub extern crate embeddedsw_sys;
pub use embeddedsw_sys as raw;

pub mod xalloc;
pub mod xaxidma;
pub mod xil_printf;
pub mod xscugic;

#[cfg(feature = "xilffs")]
pub mod ff;
