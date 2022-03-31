extern crate embeddedsw_sys;
use embeddedsw_sys as esys;

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

#[derive(Debug)]
pub enum DmaDirection {
    DMAToDevice,
    DeviceToDMA,
}

#[repr(C)]
pub struct XAxiDmaConfig {
    config: *mut esys::XAxiDma_Config,
}

impl XAxiDmaConfig {
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

#[repr(C)]
pub struct XAxiDma {
    pub axi_dma: esys::XAxiDma,
}

impl XAxiDma {
    pub unsafe fn cfg_initialize(
        &mut self,
        config: &mut XAxiDmaConfig,
    ) -> Result<(), DmaError> {
        match esys::XAxiDma_CfgInitialize(
            &mut self.axi_dma as *mut _,
            config.config,
        ) as u32
        {
            esys::XST_SUCCESS => Ok(()),
            esys::XST_INVALID_PARAM => {
                Err(DmaError::InvalidParam)
            }
            _ => Err(DmaError::Unknown),
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            esys::XAxiDma_Reset(&mut self.axi_dma as *mut _)
        }
    }

    pub fn reset_is_done(&mut self) -> bool {
        unsafe {
            match esys::XAxiDma_ResetIsDone(
                &mut self.axi_dma as *mut _,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn pause(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Pause(
                &mut self.axi_dma as *mut _,
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

    pub fn resume(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Resume(
                &mut self.axi_dma as *mut _,
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
                &mut self.axi_dma as *mut _,
                direction as i32,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

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
                &mut self.axi_dma as *mut _,
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

    pub fn self_test(&mut self) -> Result<(), DmaError> {
        unsafe {
            match esys::XAxiDma_Selftest(
                &mut self.axi_dma as *mut _,
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
                self.axi_dma.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
            );
            Self::write_reg(
                self.axi_dma.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
                rx_val | esys::XAXIDMA_IRQ_ALL_MASK,
            );
        }
    }

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
                self.axi_dma.RegBase as u32
                    + (esys::XAXIDMA_RX_OFFSET * direction),
                esys::XAXIDMA_CR_OFFSET,
            );
            Self::write_reg(
                self.axi_dma.RegBase as u32
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

pub fn xil_dcache_flush_range(addr: isize, length: u32) {
    unsafe { esys::Xil_DCacheFlushRange(addr, length) }
}
