// Interface to communicate with kernel driver
use std::ptr;
use std::mem;
use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winnt::*;
use winapi::shared::minwindef::*;

const IOCTL_READ_MEMORY: u32 = 0x222004;
const IOCTL_WRITE_MEMORY: u32 = 0x222008;

#[repr(C)]
struct MemoryRequest {
    process_id: u32,
    address: usize,
    buffer: *mut u8,
    size: usize,
}

pub struct DriverInterface {
    handle: *mut winapi::ctypes::c_void,
    process_id: u32,
}

impl DriverInterface {
    pub fn new(process_id: u32) -> Option<Self> {
        unsafe {
            let device_name = b"\\\\.\\RustDriver\0";
            let handle = CreateFileA(
                device_name.as_ptr() as *const i8,
                GENERIC_READ | GENERIC_WRITE,
                0,
                ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                ptr::null_mut(),
            );
            
            if handle == INVALID_HANDLE_VALUE {
                return None;
            }
            
            Some(DriverInterface {
                handle,
                process_id,
            })
        }
    }
    
    pub fn read<T: Copy>(&self, address: usize) -> Result<T, ()> {
        unsafe {
            let mut buffer: T = mem::zeroed();
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: &mut buffer as *mut T as *mut u8,
                size: mem::size_of::<T>(),
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_READ_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }
    
    pub fn write<T: Copy>(&self, address: usize, value: T) -> Result<(), ()> {
        unsafe {
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: &value as *const T as *mut u8,
                size: mem::size_of::<T>(),
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_WRITE_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                ptr::null_mut(),
                0,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
    
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![0u8; size];
        unsafe {
            let mut request = MemoryRequest {
                process_id: self.process_id,
                address,
                buffer: buffer.as_mut_ptr(),
                size,
            };
            
            let mut bytes_returned = 0;
            let result = DeviceIoControl(
                self.handle,
                IOCTL_READ_MEMORY,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut request as *mut _ as *mut _,
                mem::size_of::<MemoryRequest>() as u32,
                &mut bytes_returned,
                ptr::null_mut(),
            );
            
            if result != 0 {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }
}

impl Drop for DriverInterface {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
