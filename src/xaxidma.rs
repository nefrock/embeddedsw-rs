extern crate embeddedsw_sys;
use core::mem::MaybeUninit;

use embeddedsw_sys as esys;

/// Enumurates for DMA driver errors
#[derive(Debug)]
pub enum DmaError {
    InvalidParam,
    NotInit,
    NotSGDMA,
    Unknown,
    Channel,
    Submisson,
    ConfigInit,
}

/// Enumerates for DMA directions
///
/// DMAToDevice: PS  -> DDR
/// DeviceToDMA: DDR -> PS
///
///                 +------+
///              +->|  PS  |---+
///              |  +------+   |
/// DeviceToDMA  |             |  DMAToDevice
///              |  +-------+  |
///              +--|  DDR  |<-+
///                 +-------+     
///                     ^          
///                     |
///                     v
///                 +------+
///                 |  PL  |
///                 +------+
#[derive(Debug)]
pub enum DmaDirection {
    DMAToDevice,
    DeviceToDMA,
}

//-------------------------------------------------------------------------------------------------
// XAxiDmaConfig
//-------------------------------------------------------------------------------------------------

/// The configuration structure for AXI DMA instance.
///
/// # Example
/// ```
/// #
/// #![no_std]
/// #![no_main]
/// #![feature(start)]
/// extern crate alloc;
/// extern crate embeddedsw_rs;
/// use embeddedsw_rs as xemb;
/// use xemb::{ println, xaxidma::{ self, XAxiDma, XAxiDmaXonfig }, raw::* };
/// # [panic_handler]
/// # fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
/// #    println!("{}", info);
/// #    loop {}
/// # }
///
/// #[no_mangle]
/// #[start]
/// fn main() {
///    // XPAR_AXI_DMA_0_DEVICE_ID is defined in parameters.h, so bindgen generates constants.
///    let mut axi_dma_config = XAxiDmaConfig::lookup_config(raw::XPAR_AXI_DMA_0_DEVICE_ID).unwarap();
///    
/// }
/// ```
#[repr(C)]
pub struct XAxiDmaConfig {
    config: *mut esys::XAxiDma_Config,
}

impl XAxiDmaConfig {
    /// Look up the hardware configuration for XAxiDma instance.
    ///
    /// # Errors
    /// If this function cannot find device configuration,
    /// it returns DmaError variants for configuration initalize.
    ///
    pub fn lookup_config(
        id: u32,
    ) -> Result<Self, DmaError> {
        let ptr = unsafe { esys::XAxiDma_LookupConfig(id) };
        if ptr.is_null() {
            Err(DmaError::ConfigInit)
        } else {
            Ok(Self { config: ptr })
        }
    }
}

//-------------------------------------------------------------------------------------------------
// XAxiDma
//-------------------------------------------------------------------------------------------------

/// The device instance structs for XAxiDma.
///
///
/// # Safety
/// An instance must be allocated for each instance in use.
///
/// # Example
/// ```
/// #
/// #
/// #![no_std]
/// #![no_main]
/// #![feature(start)]
/// # extern crate alloc;
/// extern crate embeddedsw_rs;
/// use embeddedsw_rs as xemb;
/// use xemb::{ println, xaxidma::{ self, XAxiDma, XAxiDmaXonfig }, raw::* };
/// # [panic_handler]
/// # fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
/// #    println!("{}", info);
/// #    loop {}
/// # }
/// #
///
/// #[no_mangle]
/// #[start]
/// fn main(_argc: isize, _argv: *const *const u8) -> isize {
///    // XPAR_AXI_DMA_0_DEVICE_ID is defined in parameters.h, so bindgen generates constants.
///    let mut axi_dma_config = XAxiDmaConfig::lookup_config(raw::XPAR_AXI_DMA_0_DEVICE_ID).unwarap();
///    
///    // initalize DMA instance
///    let mut axi_dma = MaybeUninit::<XAxiDma>::uninit();
///    XAxiDma::cfg_initialize(&mut axi_dma, &mut axi_dma_config).unwrap();
///    let mut axi_dma = axi_dma.assume_init();
///    
///    // disable IRQ interrupt
///    axi_dma.axi_irq_interrupt_disable(DMAToDevice);
///    axi_dma.axi_irq_interrupt_disable(DeviceToDMA);
///
///    // flush DCache
///    xaxidma::xil_dcach_flush_range(dma_base_addr as isize, dma_size);
///    
///    // simple DMA transfer
///    axi_dma.axi_simple_transfer(dma_base_addr, dma_size, xaxidma::DMAToDevice).unwrap();
///
///    while axi_dma.busy() {
///       // wait here
///    }
///
///    return 0;
/// }
/// ```
#[repr(C)]
pub struct XAxiDma {
    inner: esys::XAxiDma,
}

impl XAxiDma {
    /// This function inilializes a DMA instance.
    ///
    /// # Safety  
    /// This function must be called before using DMA instances.
    pub fn cfg_initialize(
        xaxidma: &mut MaybeUninit<XAxiDma>,
        config: &mut XAxiDmaConfig,
    ) -> Result<(), DmaError> {
        match unsafe {
            esys::XAxiDma_CfgInitialize(
                xaxidma.as_mut_ptr() as *mut esys::XAxiDma,
                config.config,
            )
        } as u32
        {
            esys::XST_SUCCESS => Ok(()),
            esys::XST_INVALID_PARAM => {
                Err(DmaError::InvalidParam)
            }
            _ => Err(DmaError::Unknown),
        }
    }

    /// This function resets both TX and RX channels of a DMA instance.
    /// If you reset one channel by this function, the whole AXI DMA instance is reseted.
    pub fn reset(&mut self) {
        unsafe {
            esys::XAxiDma_Reset(&mut self.inner as *mut _)
        }
    }

    /// This function checks whether rest is done.
    pub fn reset_is_done(&mut self) -> bool {
        unsafe {
            match esys::XAxiDma_ResetIsDone(
                &mut self.inner as *mut _,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

    /// This function pauses DMA transaction on both chennels.
    ///
    /// # Error
    /// - If this function is called before initalizing XAxiDMA structs,
    ///   it returns DmaError variants for NotSGDMA.
    /// - If this function cannot pause AXI DMA transactions due to unknown error,
    ///   it returns DMAError variants for Unknown.
    pub fn pause(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Pause(
                &mut self.inner as *mut _,
            ) as u32
            {
                esys::XST_SUCCESS => Ok(()),
                esys::XST_NOT_SGDMA => {
                    Err(DmaError::NotSGDMA)
                }
                _ => Err(DmaError::Unknown),
            }
        }
    }

    /// This function resumes DMA transactions on both channels.
    ///
    /// # Error
    /// - If this function is called before initalizing XAxiDMA structs,
    ///   it returns DmaError variants for NotSGDMA.
    /// - If this function cannot pause AXI DMA transactions due to unknown error,
    ///   it returns DMAError variants for Unknown.
    pub fn resume(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Resume(
                &mut self.inner as *mut _,
            ) as u32
            {
                esys::XST_SUCCESS => Ok(()),
                esys::XST_NOT_SGDMA => {
                    Err(DmaError::NotSGDMA)
                }
                esys::XST_DMA_ERROR => {
                    Err(DmaError::Channel)
                }
                _ => Err(DmaError::Unknown),
            }
        }
    }

    /// This function checks whether the DMA instance  is busy.
    pub fn busy(
        &mut self,
        direction: DmaDirection,
    ) -> bool {
        let direction = match direction {
            DmaDirection::DMAToDevice => {
                esys::XAXIDMA_DMA_TO_DEVICE
            }
            DmaDirection::DeviceToDMA => {
                esys::XAXIDMA_DEVICE_TO_DMA
            }
        };

        unsafe {
            match esys::XAxiDma_Busy(
                &mut self.inner as *mut _,
                direction as i32,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

    /// This function does one simle transfer submisson.
    ///
    ///
    /// # Error
    /// - If DMA instance is busy, it cannot submit.
    /// - If DMA instance is in SG mode, it cannot submit.
    pub fn simple_transfer(
        &mut self,
        buff_addr: usize,
        length: u32,
        direction: DmaDirection,
    ) -> Result<(), DmaError> {
        let direction = match direction {
            DmaDirection::DMAToDevice => {
                esys::XAXIDMA_DMA_TO_DEVICE
            }
            DmaDirection::DeviceToDMA => {
                esys::XAXIDMA_DEVICE_TO_DMA
            }
        };
        unsafe {
            match esys::XAxiDma_SimpleTransfer(
                &mut self.inner as *mut _,
                buff_addr,
                length,
                direction as i32,
            ) {
                esys::XST_SUCCESS => Ok(()),
                esys::XST_FAILURE => {
                    Err(DmaError::Submisson)
                }
                esys::XST_INVALID_PARAM => {
                    Err(DmaError::InvalidParam)
                }
                _ => Err(DmaError::Unknown),
            }
        }
    }

    /// This function runs a self-test on the driver/device.
    pub fn self_test(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Selftest(
                &mut self.inner as *mut _,
            ) as u32
            {
                esys::XST_SUCCESS => Ok(()),
                _ => Err(DmaError::Unknown),
            }
        }
    }

    unsafe fn write_reg(
        base_addr: u32,
        offset: u32,
        data: u32,
    ) {
        core::ptr::write_volatile(
            (base_addr + offset) as *mut u32,
            data,
        )
    }

    unsafe fn read_reg(base_addr: u32, offset: u32) -> u32 {
        core::ptr::read_volatile(
            (base_addr + offset) as *const u32,
        )
    }

    /// This function enables IRQ interrupt.
    pub fn irq_interrupt_enable(
        &self,
        direction: DmaDirection,
    ) {
        let direction = match direction {
            DmaDirection::DMAToDevice => {
                esys::XAXIDMA_DMA_TO_DEVICE
            }
            DmaDirection::DeviceToDMA => {
                esys::XAXIDMA_DEVICE_TO_DMA
            }
        };
        unsafe {
            let rx_val = Self::read_reg(
                self.inner.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
            );
            Self::write_reg(
                self.inner.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
                rx_val | esys::XAXIDMA_IRQ_ALL_MASK,
            );
        }
    }

    /// This function disables IRQ interrupt.
    pub fn irq_interrupt_disable(
        &self,
        direction: DmaDirection,
    ) {
        let direction = match direction {
            DmaDirection::DMAToDevice => {
                esys::XAXIDMA_DMA_TO_DEVICE
            }
            DmaDirection::DeviceToDMA => {
                esys::XAXIDMA_DEVICE_TO_DMA
            }
        };
        unsafe {
            let rx_val = Self::read_reg(
                self.inner.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
            );
            Self::write_reg(
                self.inner.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
                rx_val & !esys::XAXIDMA_IRQ_ALL_MASK,
            );
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Public functions
//-------------------------------------------------------------------------------------------------

/// This function flishs DCache.
pub fn xil_dcache_flush_range(addr: isize, length: u32) {
    unsafe { esys::Xil_DCacheFlushRange(addr, length) }
}
