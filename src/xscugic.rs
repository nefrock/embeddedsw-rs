extern crate alloc;
extern crate embeddedsw_sys;
use core::ffi;
use core::{arch::asm, mem::MaybeUninit, ptr};
use embeddedsw_sys as esys;

//-------------------------------------------------------------------------------------------------
// XScuGicConfig
//-------------------------------------------------------------------------------------------------

/// The configuration structure for XScuGic.
#[repr(C)]
pub struct XScuGicConfig {
    config: *mut esys::XScuGic_Config,
}

impl XScuGicConfig {
    /// Look up the hardware configuration for XScuGic instance.
    ///
    /// # Errors
    /// If this function cannot find device configuration,
    /// it returns ().
    ///
    pub fn lookup_config(id: u16) -> Result<Self, ()> {
        let config =
            unsafe { esys::XScuGic_LookupConfig(id) };

        if config.is_null() {
            return Err(());
        } else {
            Ok(Self { config })
        }
    }

    /// This function takes a cpu base address.
    /// It is used for effective_address.
    pub fn get_cpu_base_addr(&self) -> u32 {
        unsafe { (*self.config).CpuBaseAddress }
    }
}

//-------------------------------------------------------------------------------------------------
// XScuGic
//-------------------------------------------------------------------------------------------------

/// The XScuGic instance structs
///
/// # Error
///
/// # Example
/// ```
/// extern crate embeddedsw_rs;
/// use core::mem::MaybeUninit;
/// use cstr_core::CStr;
/// use embeddedsw_rs as xemb;
/// use embeddedsw_rs::{
///    raw,
///    xscugic::{self, XScuGic, XScuGicConfig},
/// };
/// use xemb::{print, println};
///
/// const INTC_DEVICE_ID: u16 = 0;
/// const INTC_DEVICE_INT_ID: u16 = 0x0e;
/// const XSCUGIC_CPU_MASK: u16 = raw::XSCUGIC_SPI_CPU0_MASK;
///
/// static mut INTERRUPT_PROCESSED: bool = false;
///
///#[panic_handler]
///fn panic(Info: &core::panic::PanicInfo<'_>) -> ! {
///    loop {}
///}
///
///#[no_mangle]
///#[start]
///fn main(_argc: isize, _argv: *const *const u8) -> isize {
///
///    let xconfig =  XScuGicConfig::lookup_config(INTC_DEVICE_ID).unwrap();
///
///    let mut xscu_gic = MaybeUninit::<XScuGic>::uninit();
///    // cfg initialize
///    unsafe {
///        XScuGic::cfg_initialize(
///            &mut xscu_gic,
///            &xconfig,
///            xconfig.get_cpu_base_addr(),
///        ).unwrap();
///    }
///    let mut xscu_gic = unsafe { xscu_gic.assume_init() };
///
///    // setupt interrupt system
///    xscu_gic.exception_register_handler();
///    unsafe { xscugic::xil_exception_enable() };
///
///    // connect handler
///    xscu_gic.connect(INTC_DEVICE_INT_ID as u32, Some(device_handler)).unwarap();
///
///    // unable interrupt
///    xscu_gic.enable(INTC_DEVICE_INT_ID as u32);
///
///    // software interrupt
///    xscu_gic.software_intr(
///        INTC_DEVICE_INT_ID as u32,
///        XSCUGIC_CPU_MASK,
///    ).unwrap();
///
///    loop {
///        if unsafe { INTERRUPT_PROCESSED } {
///            break;
///        }
///    }
///
///    return 0;
///}
///
/// extern "C" fn device_handler() {
///     println!("[Info] called device handler");
///     unsafe {
///         INTERRUPT_PROCESSED = true;
///     }
/// }
/// ```
#[repr(C)]
pub struct XScuGic {
    inner: esys::XScuGic,
}

impl XScuGic {
    /// This function initializes a XScuGic structs.
    ///
    /// # Safety
    /// effective_addr is the device base address in the virtual memory address space.
    /// The caller is responsible for keeping the address mapping
    /// from EffectiveAddr to the device physical base address unchanged once this function is invoked.
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

    /// This function makes the connections between the ID of interrupt source
    /// and the corresponding handler.
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

    /// This function disconnects the registerd handler corresponding to the interrupt ID.
    pub fn disconnect(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Disconnect(&mut self.inner, id)
        };
    }

    /// This function enables the interrupt source provided as the id.
    pub fn enable(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Enable(&mut self.inner, id)
        };
    }

    /// This function diables the interrupt source provided as the id.
    pub fn disable(&mut self, id: u32) {
        unsafe {
            esys::XScuGic_Enable(&mut self.inner, id)
        };
    }

    /// This function registers the handler.
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

    /// This function remove the registerd handler.
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
    unsafe fn xil_in32(addr: u32) -> u32 {
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
    unsafe fn read_reg(base_addr: u32, offset: u32) -> u32 {
        Self::xil_in32(base_addr + offset)
    }

    /// This function diable FIQ interrupt.
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
// Xilinx Exception helper functions
// They are defined in xil_exceoption.h like macros
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
