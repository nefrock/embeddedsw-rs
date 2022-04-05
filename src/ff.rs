extern crate embeddedsw_sys;
use core::ptr::addr_of;
use core::{ffi::c_void, mem::*};
use embeddedsw_sys::{self as esys, FRESULT};

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

#[repr(u8)]
pub enum FileMountOption {
    Delayed = 0,
    Immediately = 1,
}

///
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
    FTooManyOpenFiles =
        FRESULT::FR_TOO_MANY_OPEN_FILES as u32,
    FInvalidParameter =
        FRESULT::FR_INVALID_PARAMETER as u32,
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
            FRESULT::FR_INVALID_NAME => {
                FResult::FInvalidName
            }
            FRESULT::FR_DENIED => FResult::FDenied,
            FRESULT::FR_EXIST => FResult::FExist,
            FRESULT::FR_INVALID_OBJECT => {
                FResult::FInvalidObject
            }
            FRESULT::FR_WRITE_PROTECTED => {
                FResult::FWriteProtected
            }
            FRESULT::FR_INVALID_DRIVE => {
                FResult::FInvalidDrive
            }
            FRESULT::FR_NOT_ENABLED => FResult::FNotEnabled,
            FRESULT::FR_NO_FILESYSTEM => {
                FResult::FNoFilesystem
            }
            FRESULT::FR_MKFS_ABORTED => {
                FResult::FMfksAborted
            }
            FRESULT::FR_TIMEOUT => FResult::FTimeOut,
            FRESULT::FR_LOCKED => FResult::FLocked,
            FRESULT::FR_NOT_ENOUGH_CORE => {
                FResult::FNotEnoughCore
            }
            FRESULT::FR_TOO_MANY_OPEN_FILES => {
                FResult::FTooManyOpenFiles
            }
            FRESULT::FR_INVALID_PARAMETER => {
                FResult::FInvalidParameter
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FatFs {
    inner: esys::FATFS,
}

impl FatFs {
    pub fn mount(
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
                fresult => {
                    Err(FResult::from_fresult(fresult))
                }
            }
        }
    }

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

#[derive(Debug)]
pub struct Fil {
    inner: esys::FIL,
}

impl Fil {
    pub fn open(
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
                fresult => {
                    Err(FResult::from_fresult(fresult))
                }
            }
        }
    }

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
                fresult => {
                    Err(FResult::from_fresult(fresult))
                }
            }
        }
    }

    pub fn write(
        &mut self,
        buff: &[u8],
        n: usize,
    ) -> Result<usize, FResult> {
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
                fresult => {
                    Err(FResult::from_fresult(fresult))
                }
            }
        }
    }

    pub fn close(&mut self) -> Result<(), FResult> {
        unsafe {
            match esys::f_close(&mut self.inner) {
                FRESULT::FR_OK => Ok(()),
                fresult => {
                    Err(FResult::from_fresult(fresult))
                }
            }
        }
    }
}
