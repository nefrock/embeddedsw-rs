extern crate embeddedsw_sys;
use core::ptr::addr_of;
use core::{ffi::c_void, mem::*};
use embeddedsw_sys::{self as esys, FRESULT};

//-------------------------------------------------------------------------------------------------
// Enumerates for file access modes
//-------------------------------------------------------------------------------------------------

/// Enumrates for file access modes
/// Please see [Fatfs library](http://elm-chan.org/fsw/ff/doc/open.html) to get a more information.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum FileAccessMode {
    Read = esys::FA_READ,
    Write = esys::FA_WRITE,
    OpenExisting = esys::FA_OPEN_EXISTING,
    CreateNew = esys::FA_CREATE_NEW,
    CreateAlways = esys::FA_CREATE_ALWAYS,
    OpenAlways = esys::FA_OPEN_ALWAYS,
    OpenApend = esys::FA_OPEN_APPEND,
}

//-------------------------------------------------------------------------------------------------
// Enumerates for file mount options
//-------------------------------------------------------------------------------------------------

/// Enumerates for file mount options
/// Please see [Fatfs library](http://elm-chan.org/fsw/ff/doc/open.html) to get more information.
#[repr(u8)]
pub enum FileMountOption {
    Delayed = 0,
    Immediately = 1,
}

//-------------------------------------------------------------------------------------------------
// Enumerates for file
//-------------------------------------------------------------------------------------------------

/// Enumerates for Return values.
/// Please see [Fatfs library](http://elm-chan.org/fsw/ff/doc/rc.html#de) to get more information.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum FResult {
    FOk = FRESULT::FR_OK as u32,
    FDiskErr = FRESULT::FR_DISK_ERR as u32,
    FIntErr = FRESULT::FR_INT_ERR as u32,
    FNotReady = FRESULT::FR_NOT_READY as u32,
    FNoFile = FRESULT::FR_NO_FILE as u32,
    FNoPath = FRESULT::FR_NO_PATH as u32,
    FInvalidName = FRESULT::FR_INVALID_NAME as u32,
    FDenied = FRESULT::FR_DENIED as u32,
    FExist = FRESULT::FR_EXIST as u32,
    FInvalidObject = FRESULT::FR_INVALID_OBJECT as u32,
    FWriteProtected = FRESULT::FR_WRITE_PROTECTED as u32,
    FInvalidDrive = FRESULT::FR_INVALID_DRIVE as u32,
    FNotEnabled = FRESULT::FR_NOT_ENABLED as u32,
    FNoFilesystem = FRESULT::FR_NO_FILESYSTEM as u32,
    FMfksAborted = FRESULT::FR_MKFS_ABORTED as u32,
    FTimeOut = FRESULT::FR_TIMEOUT as u32,
    FLocked = FRESULT::FR_LOCKED as u32,
    FNotEnoughCore = FRESULT::FR_NOT_ENOUGH_CORE as u32,
    FTooManyOpenFiles = FRESULT::FR_TOO_MANY_OPEN_FILES as u32,
    FInvalidParameter = FRESULT::FR_INVALID_PARAMETER as u32,
}

impl FResult {
    pub fn from_fresult(fresult: esys::FRESULT) -> FResult {
        match fresult {
            FRESULT::FR_OK => FResult::FOk,
            FRESULT::FR_DISK_ERR => FResult::FDiskErr,
            FRESULT::FR_INT_ERR => FResult::FIntErr,
            FRESULT::FR_NOT_READY => FResult::FNotReady,
            FRESULT::FR_NO_FILE => FResult::FNoFile,
            FRESULT::FR_NO_PATH => FResult::FNoPath,
            FRESULT::FR_INVALID_NAME => FResult::FInvalidName,
            FRESULT::FR_DENIED => FResult::FDenied,
            FRESULT::FR_EXIST => FResult::FExist,
            FRESULT::FR_INVALID_OBJECT => FResult::FInvalidObject,
            FRESULT::FR_WRITE_PROTECTED => FResult::FWriteProtected,
            FRESULT::FR_INVALID_DRIVE => FResult::FInvalidDrive,
            FRESULT::FR_NOT_ENABLED => FResult::FNotEnabled,
            FRESULT::FR_NO_FILESYSTEM => FResult::FNoFilesystem,
            FRESULT::FR_MKFS_ABORTED => FResult::FMfksAborted,
            FRESULT::FR_TIMEOUT => FResult::FTimeOut,
            FRESULT::FR_LOCKED => FResult::FLocked,
            FRESULT::FR_NOT_ENOUGH_CORE => FResult::FNotEnoughCore,
            FRESULT::FR_TOO_MANY_OPEN_FILES => FResult::FTooManyOpenFiles,
            FRESULT::FR_INVALID_PARAMETER => FResult::FInvalidParameter,
        }
    }
}

//-------------------------------------------------------------------------------------------------
// FatFs struct
//-------------------------------------------------------------------------------------------------

/// The FatFs struct
///
/// # Example
/// ```
/// #![no_std]
/// #![no_main]
/// #![feature(start)]
///
/// extern crate embeddedsw_rs;
/// use core::mem::MaybeUninit;
/// use embeddedsw_rs as xemb;
/// use xemb::{
///     ff::{FileAccessMode::*, FileMountOption::*, *},
///     print, println,
/// };
///
/// # #[panic_handler]
/// # fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
/// #    loop {}
/// # }
///
/// #[no_mangle]
/// #[start]
/// fn main(_argc: isize, _argv: *const *const u8) -> isize {
///
///     // Mount Logical Drive
///     let path = "0:/\0";
///     let opt = Immediately;
///     let mut fatfs = MaybeUninit::<FatFs>::uninit();
///     if let Err(e) = FatFs::mount(&mut fatfs, path, opt) {
///         println!("[Error] {:?}", e);
///         return 0;
///     }
///     let mut fatfs = unsafe { fatfs.assume_init() };
///
///     // Open the test.dat file
///     let fname = "some.file\0";
///     let mode = Read;
///     let mut fil = MaybeUninit::<Fil>::uninit();
///     if let Err(e) =  Fil::open(&mut fil, fname, mode) {
///        println!("[Error] {:?}", e);
///        return 0;
///     }
///     let mut fil = unsafe { fil.assume_init() };
///
///     // Read contents in the some.file
///     let mut buff = [2; 124];
///     let n = 10;
///     if let Err(e) = fil.read(&mut buff, n) {
///         println!("[Error] {:?}", e);
///         return 0;
///     } else {
///         // print the 10 bytes that are read by read() function
///         for i in 0..n {
///             println!("{}", buff[i] as char);
///         }
///     }
///
///     // Close the test.dat and unmount the logical volume
///     if let Err(fresult) = fil.close() {
///         println!("close file: {:?}", fresult);
///         return 0;
///     }
///
///     fatfs.unmount(path);
///
///     println!("Scucessfully Read SD Card Test");
///     return 0;
/// }
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct FatFs {
    inner: esys::FATFS,
}

impl FatFs {
    /// This functions mount a logical volume
    ///
    /// # Errors
    /// If this function cannot mount a logical volume,
    /// it returns one of the following FResult variants.
    /// - FOk
    /// - FInvalidDrive
    /// - FDiskErr
    /// - FNotEnabled
    /// - FNoFileSystem
    ///
    /// please see [Fatfs](http://elm-chan.org/fsw/ff/doc/rc.html#de) library to get more detail of this bindings.
    ///
    /// # Safety
    /// str of path must be null-terminated.
    /// If non null-terminated str is passed on to the argument of path,
    /// this function hangs.
    pub unsafe fn mount(
        fatfs: &mut MaybeUninit<FatFs>,
        path: &'static str,
        opt: FileMountOption,
    ) -> Result<(), FResult> {
        unsafe {
            match esys::f_mount(
                fatfs.as_mut_ptr() as *mut esys::FATFS,
                path.as_ptr() as *const esys::TCHAR,
                opt as u8,
            ) {
                FRESULT::FR_OK => Ok(()),
                fresult => Err(FResult::from_fresult(fresult)),
            }
        }
    }

    /// This function unmount a logical volume
    pub fn unmount(&mut self, path: &'static str) {
        unsafe {
            esys::f_mount(
                core::ptr::null_mut::<esys::FATFS>(),
                path.as_ptr() as *const _,
                0,
            );
        }
    }
}

///
#[derive(Debug)]
pub struct Fil {
    inner: esys::FIL,
}

impl Fil {
    /// This function opens a file
    ///
    /// If this function cannot open a logical volume,
    /// it returns one of the following FResult variants.
    /// - FOk
    /// - FIntErr
    /// - FNotReady
    /// - FNoPath
    /// - FInvalidName
    /// - FDenied
    /// - FInvalidObject
    /// - FWriteProtected
    /// - FInvalidDrive
    /// - FNotEnabled
    /// - FNoFileSystem
    /// - FTimeout
    /// - FLocked
    /// - FNotEnoughCore
    /// - FTooManyOpenFiles
    ///
    /// please see [Fatfs](http://elm-chan.org/fsw/ff/doc/rc.html#de) library to get more detail of this bindings.
    ///
    ///
    /// # Safety
    /// str of path must be null-terminated.
    /// If no null-terminated str is passed on to the argument of path,
    /// this function hangs.
    pub unsafe fn open(
        fil: &mut MaybeUninit<Fil>,
        path: &str,
        mode: FileAccessMode,
    ) -> Result<(), FResult> {
        unsafe {
            match esys::f_open(
                fil.as_mut_ptr() as *mut esys::FIL,
                path.as_ptr() as *const esys::TCHAR,
                mode as u8,
            ) {
                FRESULT::FR_OK => Ok(()),
                fresult => Err(FResult::from_fresult(fresult)),
            }
        }
    }

    /// This function reads a contents of a file.
    ///
    /// If this function cannot read a logical volume,
    /// it returns one of the following FResult variants.
    /// - FOk
    /// - FDiskErr
    /// - FIntErr
    /// - FDenied
    /// - FInvalidObject
    /// - FTimeout
    ///
    /// please see [Fatfs](http://elm-chan.org/fsw/ff/doc/rc.html#de) library to get more detail of this bindings.
    ///
    pub fn read(
        &mut self,
        buff: &mut [u8],
        n: usize,
    ) -> Result<usize, FResult> {
        let br: u32 = 0;
        let fil = &mut self.inner;
        unsafe {
            match esys::f_read(
                fil as *mut esys::FIL,
                buff.as_mut_ptr() as *mut c_void,
                n as u32,
                addr_of!(br) as *mut u32,
            ) {
                FRESULT::FR_OK => Ok(br as usize),
                fresult => Err(FResult::from_fresult(fresult)),
            }
        }
    }

    /// This function writes data to a file.
    ///
    /// If this function cannot write a logical volume,
    /// it returns one of the following FResult variants.
    /// - FOk
    /// - FDiskErr
    /// - FIntErr
    /// - FDenied
    /// - FInvalidObject
    /// - FTimeout
    ///
    /// please see [Fatfs](http://elm-chan.org/fsw/ff/doc/rc.html#de) library to get more detail of this bindings.
    ///
    pub fn write(&mut self, buff: &[u8], n: usize) -> Result<usize, FResult> {
        let bw: u32 = 0;
        let fil = &mut self.inner;
        unsafe {
            match esys::f_write(
                fil as *mut esys::FIL,
                buff.as_ptr() as *const _,
                n as u32,
                addr_of!(bw) as *mut u32,
            ) {
                FRESULT::FR_OK => Ok(bw as usize),
                fresult => Err(FResult::from_fresult(fresult)),
            }
        }
    }

    /// This function closes a opend file.
    ///
    /// If this function cannot close a logical volume,
    /// it returns one of the following FResult variants.
    /// - FOk
    /// - FDiskErr
    /// - FIntErr
    /// - FInvalidObject
    /// - FTimeout
    ///
    /// please see [Fatfs](http://elm-chan.org/fsw/ff/doc/rc.html#de) library to get more detail of this bindings.
    ///
    pub fn close(&mut self) -> Result<(), FResult> {
        unsafe {
            match esys::f_close(&mut self.inner) {
                FRESULT::FR_OK => Ok(()),
                fresult => Err(FResult::from_fresult(fresult)),
            }
        }
    }
}
