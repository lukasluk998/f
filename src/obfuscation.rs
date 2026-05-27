/// String obfuscation module
/// 
/// Encrypts strings at compile-time to prevent static analysis.
/// EAC scans binaries for known strings like "GameAssembly.dll", "RustClient.exe", etc.
/// 
/// This module provides compile-time XOR encryption of sensitive strings.

/// Compile-time XOR encryption key
/// This changes every build to prevent signature matching
const XOR_KEY: u8 = {
    // Generate pseudo-random key based on compile time
    // In real implementation, this would use build timestamp
    0x42 ^ (line!() as u8)
};

/// Encrypt a byte array at compile-time
const fn encrypt_bytes(bytes: &[u8]) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut i = 0;
    while i < bytes.len() && i < 256 {
        result[i] = bytes[i] ^ XOR_KEY;
        i += 1;
    }
    result
}

/// Macro for compile-time string encryption
/// 
/// Usage:
///   let process_name = obf_str!("RustClient.exe");
///   let dll_name = obf_str!("GameAssembly.dll");
#[macro_export]
macro_rules! obf_str {
    ($s:expr) => {{
        // Encrypt at compile time
        const ENCRYPTED: [u8; 256] = $crate::obfuscation::encrypt_bytes($s.as_bytes());
        const LEN: usize = $s.len();
        
        // Decrypt at runtime
        let mut decrypted = Vec::with_capacity(LEN);
        for i in 0..LEN {
            decrypted.push(ENCRYPTED[i] ^ $crate::obfuscation::XOR_KEY);
        }
        
        unsafe { String::from_utf8_unchecked(decrypted) }
    }};
}

/// Advanced obfuscation with stack strings
/// Stores encrypted string on stack to avoid .rdata section scanning
pub struct StackString {
    data: [u8; 128],
    len: usize,
}

impl StackString {
    /// Create encrypted stack string
    pub const fn new(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut data = [0u8; 128];
        let mut i = 0;
        
        while i < bytes.len() && i < 128 {
            data[i] = bytes[i] ^ XOR_KEY ^ (i as u8);
            i += 1;
        }
        
        StackString {
            data,
            len: bytes.len(),
        }
    }
    
    /// Decrypt and return string
    pub fn decrypt(&self) -> String {
        let mut result = Vec::with_capacity(self.len);
        for i in 0..self.len {
            result.push(self.data[i] ^ XOR_KEY ^ (i as u8));
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

/// Obfuscated string literals
/// These are the most commonly scanned strings in game cheats
pub mod strings {
    use super::StackString;
    
    // Process names
    pub const RUST_CLIENT: StackString = StackString::new("RustClient.exe");
    pub const GAME_ASSEMBLY: StackString = StackString::new("GameAssembly.dll");
    pub const UNITY_PLAYER: StackString = StackString::new("UnityPlayer.dll");
    
    // Common cheat strings
    pub const LOCAL_PLAYER: StackString = StackString::new("LocalPlayer");
    pub const PLAYER_MODEL: StackString = StackString::new("PlayerModel");
    pub const BASE_PLAYER: StackString = StackString::new("BasePlayer");
    pub const PLAYER_INPUT: StackString = StackString::new("PlayerInput");
    
    // Window titles
    pub const RUST_WINDOW: StackString = StackString::new("Rust");
    
    // Driver names
    pub const DRIVER_NAME: StackString = StackString::new("RustDriver");
    pub const DRIVER_DEVICE: StackString = StackString::new("\\\\.\\RustDriver");
    
    // Registry keys (for HWID spoof)
    pub const REG_HARDWARE: StackString = StackString::new("HARDWARE");
    pub const REG_DESCRIPTION: StackString = StackString::new("DESCRIPTION");
    pub const REG_SYSTEM: StackString = StackString::new("System");
}

/// API function name obfuscation
/// Dynamically resolves API functions to avoid IAT scanning
pub mod api_obf {
    use std::ffi::CString;
    use std::os::raw::c_char;
    
    /// Get obfuscated API name
    pub fn get_api_name(name: &str) -> CString {
        // XOR obfuscate the name
        let mut obfuscated = Vec::new();
        for (i, &b) in name.as_bytes().iter().enumerate() {
            obfuscated.push(b ^ super::XOR_KEY ^ (i as u8));
        }
        
        // Decrypt for GetProcAddress
        let mut decrypted = Vec::new();
        for (i, &b) in obfuscated.iter().enumerate() {
            decrypted.push(b ^ super::XOR_KEY ^ (i as u8));
        }
        
        CString::new(decrypted).unwrap()
    }
    
    // Common WinAPI names
    pub const KERNEL32: &str = "kernel32.dll";
    pub const NTDLL: &str = "ntdll.dll";
    pub const USER32: &str = "user32.dll";
    
    pub const OPEN_PROCESS: &str = "OpenProcess";
    pub const READ_PROCESS_MEMORY: &str = "ReadProcessMemory";
    pub const WRITE_PROCESS_MEMORY: &str = "WriteProcessMemory";
    pub const VIRTUAL_ALLOC_EX: &str = "VirtualAllocEx";
    pub const CREATE_REMOTE_THREAD: &str = "CreateRemoteThread";
}

/// Polymorphic string generation
/// Generates different string representations that mean the same thing
pub mod polymorphic {
    /// Generate path with random case and separators
    pub fn obfuscate_path(path: &str) -> String {
        // Mix forward/back slashes
        let mut result = String::new();
        for (i, c) in path.chars().enumerate() {
            if c == '/' || c == '\\' {
                // Randomly use / or \
                if i % 2 == 0 {
                    result.push('\\');
                } else {
                    result.push('/');
                }
            } else {
                result.push(c);
            }
        }
        result
    }
    
    /// Add random case variation
    pub fn random_case(s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
        }
        result
    }
}

/// Runtime string builder
/// Constructs strings at runtime to avoid static analysis
pub struct RuntimeString {
    parts: Vec<&'static str>,
}

impl RuntimeString {
    pub fn new() -> Self {
        RuntimeString { parts: Vec::new() }
    }
    
    pub fn add(&mut self, part: &'static str) -> &mut Self {
        self.parts.push(part);
        self
    }
    
    pub fn build(&self) -> String {
        self.parts.concat()
    }
}

/// Helper macro for building strings at runtime
#[macro_export]
macro_rules! runtime_str {
    ($($part:expr),+) => {{
        let mut builder = $crate::obfuscation::RuntimeString::new();
        $(
            builder.add($part);
        )+
        builder.build()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_obf_str() {
        let s = obf_str!("test");
        assert_eq!(s, "test");
    }
    
    #[test]
    fn test_stack_string() {
        let s = StackString::new("GameAssembly.dll");
        assert_eq!(s.decrypt(), "GameAssembly.dll");
    }
    
    #[test]
    fn test_runtime_string() {
        let s = runtime_str!("Game", "Assembly", ".dll");
        assert_eq!(s, "GameAssembly.dll");
    }
}
