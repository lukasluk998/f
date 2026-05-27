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
    pub handle: *mut std::ffi::c_void,
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
    
    // MEMORY BATCHING - Read entire struct at once (80% fewer reads!)
    // Instead of: read health (1 call), read position (1 call), read max_health (1 call)
    // Do: read entire player struct (1 call), parse locally
    
    /// Read entire buffer from memory (for struct batching)
    pub fn read_buffer(&self, address: usize, size: usize) -> Result<Vec<u8>, ()> {
        self.read_bytes(address, size)
    }
    
    /// Read structured player data in single call
    /// Much faster and safer than multiple reads
    pub fn read_player_data_batch(&self, player_address: usize, offsets: &PlayerBatchOffsets) -> Result<PlayerBatchData, ()> {
        // Calculate struct size (from first offset to last offset + largest field size)
        let max_offset = offsets.max_offset() + 64; // +64 for safety margin
        
        // Read entire memory region in ONE call
        let buffer = self.read_buffer(player_address, max_offset)?;
        
        // Parse fields locally (no more memory reads!)
        Ok(PlayerBatchData::from_buffer(&buffer, offsets))
    }
}

// Batch offset structure for player data
#[derive(Clone, Debug)]
pub struct PlayerBatchOffsets {
    pub health: usize,
    pub max_health: usize,
    pub position: usize,         // Vec3 (12 bytes)
    pub player_model: usize,
    pub transform: usize,
    pub rotation: usize,         // Vec3 (12 bytes)
    pub velocity: usize,         // Vec3 (12 bytes)
}

impl PlayerBatchOffsets {
    pub fn max_offset(&self) -> usize {
        *[
            self.health,
            self.max_health,
            self.position,
            self.player_model,
            self.transform,
            self.rotation,
            self.velocity,
        ]
        .iter()
        .max()
        .unwrap_or(&0)
    }
}

// Batched player data (parsed from single memory read)
#[derive(Clone, Debug)]
pub struct PlayerBatchData {
    pub health: f32,
    pub max_health: f32,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub velocity: Option<[f32; 3]>,
}

impl PlayerBatchData {
    /// Parse player data from memory buffer
    pub fn from_buffer(buffer: &[u8], offsets: &PlayerBatchOffsets) -> Self {
        Self {
            health: Self::read_f32(buffer, offsets.health),
            max_health: Self::read_f32(buffer, offsets.max_health),
            position: Self::read_vec3(buffer, offsets.position),
            rotation: Self::read_vec3(buffer, offsets.rotation),
            velocity: Self::read_vec3(buffer, offsets.velocity),
        }
    }
    
    fn read_f32(buffer: &[u8], offset: usize) -> f32 {
        if offset + 4 > buffer.len() {
            return 0.0;
        }
        let bytes = &buffer[offset..offset + 4];
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    
    fn read_vec3(buffer: &[u8], offset: usize) -> Option<[f32; 3]> {
        if offset + 12 > buffer.len() {
            return None;
        }
        Some([
            Self::read_f32(buffer, offset),
            Self::read_f32(buffer, offset + 4),
            Self::read_f32(buffer, offset + 8),
        ])
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
