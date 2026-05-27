use std::mem;
use std::ptr;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::winnt::{PROCESS_VM_READ, PROCESS_VM_WRITE, PROCESS_VM_OPERATION, PROCESS_QUERY_INFORMATION};
use winapi::shared::minwindef::{FALSE, TRUE};

pub struct Process {
    pub pid: u32,
    pub handle: *mut winapi::ctypes::c_void,
}

impl Process {
    pub fn from_name(name: &str) -> Option<Self> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot.is_null() {
                return None;
            }

            let mut entry: PROCESSENTRY32 = mem::zeroed();
            entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut entry) == TRUE {
                loop {
                    let process_name = String::from_utf8_lossy(&entry.szExeFile)
                        .trim_end_matches('\0')
                        .to_lowercase();

                    if process_name.contains(&name.to_lowercase()) {
                        let pid = entry.th32ProcessID;
                        CloseHandle(snapshot);

                        let handle = OpenProcess(
                            PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_QUERY_INFORMATION,
                            FALSE,
                            pid,
                        );

                        if !handle.is_null() {
                            return Some(Process { pid, handle });
                        }
                    }

                    if Process32Next(snapshot, &mut entry) != TRUE {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
            None
        }
    }

    pub fn get_module_base(&self, module_name: &str) -> Option<usize> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(
                TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32,
                self.pid,
            );
            if snapshot.is_null() {
                return None;
            }

            let mut entry: MODULEENTRY32 = mem::zeroed();
            entry.dwSize = mem::size_of::<MODULEENTRY32>() as u32;

            if Module32First(snapshot, &mut entry) == TRUE {
                loop {
                    let mod_name = String::from_utf8_lossy(&entry.szModule)
                        .trim_end_matches('\0')
                        .to_lowercase();

                    if mod_name.contains(&module_name.to_lowercase()) {
                        CloseHandle(snapshot);
                        return Some(entry.modBaseAddr as usize);
                    }

                    if Module32Next(snapshot, &mut entry) != TRUE {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
            None
        }
    }

    pub fn read<T: Copy>(&self, address: usize) -> Result<T, ()> {
        unsafe {
            let mut buffer: T = mem::zeroed();
            let size = mem::size_of::<T>();
            let mut bytes_read = 0;

            let result = ReadProcessMemory(
                self.handle,
                address as *const _,
                &mut buffer as *mut T as *mut _,
                size,
                &mut bytes_read,
            );

            if result != FALSE && bytes_read == size {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }

    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, ()> {
        unsafe {
            let mut buffer = vec![0u8; size];
            let mut bytes_read = 0;

            let result = ReadProcessMemory(
                self.handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                size,
                &mut bytes_read,
            );

            if result != FALSE && bytes_read == size {
                Ok(buffer)
            } else {
                Err(())
            }
        }
    }

    pub fn write<T: Copy>(&self, address: usize, value: T) -> Result<(), ()> {
        unsafe {
            let size = mem::size_of::<T>();
            let mut bytes_written = 0;

            let result = WriteProcessMemory(
                self.handle,
                address as *mut _,
                &value as *const T as *const _,
                size,
                &mut bytes_written,
            );

            if result != FALSE && bytes_written == size {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn write_bytes(&self, address: usize, bytes: &[u8]) -> Result<(), ()> {
        unsafe {
            let mut bytes_written = 0;

            let result = WriteProcessMemory(
                self.handle,
                address as *mut _,
                bytes.as_ptr() as *const _,
                bytes.len(),
                &mut bytes_written,
            );

            if result != FALSE && bytes_written == bytes.len() {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    // Multi-level pointer chain resolution
    pub fn read_pointer_chain(&self, base: usize, offsets: &[usize]) -> Result<usize, ()> {
        let mut address = base;
        
        for (i, &offset) in offsets.iter().enumerate() {
            if i < offsets.len() - 1 {
                address = self.read::<usize>(address)?;
            }
            address = address.wrapping_add(offset);
        }
        
        Ok(address)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                CloseHandle(self.handle);
            }
        }
    }
}
