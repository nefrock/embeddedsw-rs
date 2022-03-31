#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod xalloc;
pub mod xil_printf;

#[cfg(feature = "xaxidma")]
pub mod xaxidma;

#[cfg(feature = "xilffs")]
pub mod ff;
