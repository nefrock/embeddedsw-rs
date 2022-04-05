extern crate alloc;
extern crate embeddedsw_sys;
use core::ffi;
use core::{arch::asm, mem::MaybeUninit, ptr};
use embeddedsw_sys as esys;

//-------------------------------------------------------------------------------------------------
// Generic Interrupt Controller
//--------------------------------------------------------------------------------------------------

/// Generic Interrupt Controller
#[repr(C)]
pub struct XScuGic {
    inner: esys::XScuGic,
}

impl XScuGic {
    pub unsafe fn cfg_initialize(
        xscugic: &mut MaybeUninit<XScuGic>,
        config: &XScuGicConfig,
        effective_addr: u32,
    ) -> Result<(), i32> {
        let status = esys::XScuGic_CfgInitialize(
            xscugic.as_mut_ptr() as *mut esys::XScuGic,
            config.config,
            effective_addr,
        );

        if status != 0 {
            return Err(status);
        } else {
            Ok(())
        }
    }

    pub fn connect(
        &mut self,
        id: u32,
        handler: Option<extern "C" fn() -> ()>,
    ) -> Result<(), i32> {
        let handler = handler.map(|f| unsafe {
            let ptr = f as *const _;
            core::mem::transmute::<
                *const (),
                unsafe extern "C" fn(
                    *mut ffi::c_void,
                ) -> (),
            >(ptr)
        });
        let status = unsafe {
            esys::XScuGic_Connect(
                &mut self.inner,
                id,
                handler,
                ptr::null::<()>() as *mut _,
            )
        };
        if status != 0 {
            Err(status)
        } else {
            Ok(())
        }
    }

    pub fn disconnect(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Disconnect(&mut self.inner, id)
        };
    }

    pub fn enable(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Enable(&mut self.inner, id)
        };
    }

    pub fn disable(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Enable(&mut self.inner, id)
        };
    }

    pub fn exception_register_handler(&mut self) {
        unsafe {
            let xscu_interrupt_handler = Some(
                esys::XScuGic_InterruptHandler as *const _,
            )
            .map(|f| {
                core::mem::transmute::<
                    *const (),
                    unsafe extern "C" fn(
                        *mut ffi::c_void,
                    )
                        -> (),
                >(f)
            });

            esys::Xil_ExceptionRegisterHandler(
                esys::XIL_EXCEPTION_ID_INT,
                xscu_interrupt_handler,
                &mut self.inner as *mut esys::XScuGic
                    as *mut _,
            )
        }
    }

    pub fn exception_remove_handler(&self) {
        unsafe {
            esys::Xil_ExceptionRemoveHandler(
                esys::XIL_EXCEPTION_ID_INT,
            )
        };
    }

    #[inline(always)]
    unsafe fn xil_out32(addr: u32, value: u32) {
        core::ptr::write_volatile(addr as *mut u32, value)
    }

    #[inline(always)]
    pub unsafe fn xil_in32(addr: u32) -> u32 {
        core::ptr::read_volatile(addr as *mut u32)
    }

    #[inline(always)]
    unsafe fn write_reg(
        base_addr: u32,
        offset: u32,
        data: u32,
    ) {
        Self::xil_out32(base_addr + offset, data)
    }

    #[inline(always)]
    pub unsafe fn read_reg(
        base_addr: u32,
        offset: u32,
    ) -> u32 {
        Self::xil_in32(base_addr + offset)
    }

    #[inline(always)]
    pub fn disable_fiq(&self) {
        unsafe {
            let reg = Self::read_reg(
                esys::XPAR_PSU_RCPU_GIC_BASEADDR,
                0,
            );
            Self::write_reg(
                esys::XPAR_PSU_RCPU_GIC_BASEADDR,
                0,
                reg & !8,
            );
        }
    }
}

//-------------------------------------------------------------------------------------------------
// GIC Config
//-------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct XScuGicConfig {
    config: *mut esys::XScuGic_Config,
}

impl XScuGicConfig {
    pub fn lookup_config(id: u16) -> Result<Self, ()> {
        let config =
            unsafe { esys::XScuGic_LookupConfig(id) };

        if config.is_null() {
            return Err(());
        } else {
            Ok(Self { config })
        }
    }

    pub fn get_cpu_base_addr(&self) -> u32 {
        unsafe { (*self.config).CpuBaseAddress }
    }
}

//-------------------------------------------------------------------------------------------------
// Xilinx Exception helper functions
// They are defined in xil_exceoption.h such as macros
//-------------------------------------------------------------------------------------------------

#[inline(always)]
unsafe fn mfcpsr() -> u32 {
    let mut rval;
    asm!("mrs {}, cpsr", out(reg) rval, options(preserves_flags));
    rval
}

#[inline(always)]
unsafe fn mtcpsr(v: u32) {
    asm!("msr cpsr, {}", in(reg) v)
}

#[inline(always)]
unsafe fn xil_exception_enable_mask(mask: u32) {
    mtcpsr(mfcpsr() & !(mask & esys::XIL_EXCEPTION_ALL))
}

#[inline(always)]
pub unsafe fn xil_exception_enable() {
    xil_exception_enable_mask(esys::XIL_EXCEPTION_IRQ)
}
