# embeddedsw-rs
Bindings to Xilinx's zynq driver for bearmetal applications.

You can use this crate to develop baremetal application with memory safety.


## Installation
```rust
[dependecies]
embeddedsw-rs = { git = "https://github.com/nefrock/embeddedsw-rs", branch = "master" }
```


## Examples
The code of th following exampls can be found at [samples repository]().

1. Print a "Hello Rust World" to console via UART.
```rust
#![no_std]
#![no_main]
#![feature(start)]

extern crate embeddedsw_rs;
use embeddedsw_rs as xemb;
use xemb::println;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello Rust World!!");
    return 0;
}

```

2. Read contents contained on the SD card using xilffs library.
```rust
#![no_std]
#![no_main]
#![feature(start)]

extern crate embeddedsw_rs;
use core::mem::MaybeUninit;
use embeddedsw_rs as xemb;
use xemb::{
    ff::{FileAccessMode::*, FileMountOption::*, *},
    println,
};

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("SD Card Test");

    // Mount Logical Drive
    let path = "0:/\0";      // must be null terminated string
    let opt = Immediately;
    let mut fatfs = MaybeUninit::<FatFs>::uninit();
    FatFs::mount(&mut fatfs, path, opt).unwrap();
    let mut fatfs = unsafe { fatfs.assume_init() };

    // Open the test.dat file
    let fname = "test.dat\0"; // must be null terminated string
    let mode = Read;
    let mut fil = MaybeUninit::<Fil>::uninit();
    Fil::open(&mut fil, fname, mode).unwrap();
    let mut fil = unsafe { fil.assume_init() };

    // Read contents in the test.dat
    let mut buff = [2; 124];
    let n = 10;
    let read_bytes = fil.read(&mut buff, n).unwrap();
    for i in 0..read_bytes {
        println("", buff[i]);
    }

    // Close the test.dat and unmount logica drive
    fil.close().unwrap();
    fatfs.unmount(path);

    println!("Scucessfully Read SD Card Test");
    return 0;
}
```


3. Memory safe heap allocator using Xilinx's malloc and Rust's type checker.
```rust
#![no_std]
#![no_main]
#![feature(start)]

extern crate alloc;
extern crate embeddedsw_rs;
use alloc::vec::Vec;
use embeddedsw_rs as xemb;
use xemb::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Allocator Test");
    let mut v = Vec::new();

    for i in 0..100 {
        if (i + 1) % 10 == 0 {
            println!("pushed [{}] elements", i + 1);
        }
        v.push(i as usize);
    }

    for (i, e) in v.iter().enumerate() {
        if *e != i {
            println!("[Error] expected {}, but got {}", i, e);
        }
        if (i + 1) % 10 == 0 {
            println!("poped [{}] elements", i + 1);
        }
    }

    println!("Sucessfully Allocator Test");
    return 0;
}

```

## Support Xilinx Dirver library
- xalloc.rs  
    Global allocator using Xilinx's malloc
- xil_printf.rs
    Uart sender and useful macros.
- xaxidma.rs  
    AXI DMA Driver. Only simple dma transfe mode
- xscugic.rs  
    Generic interrupt controller.
- ff.rs  
    Xilinx's FatFs library.

If you want to more detail information about these drivers, please see doc comments and [samples repository]().


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.