use crate::memory::Process;

pub struct PatternScanner<'a> {
    process: &'a Process,
}

impl<'a> PatternScanner<'a> {
    pub fn new(process: &'a Process) -> Self {
        Self { process }
    }

    pub fn scan_pattern(&self, start: usize, size: usize, pattern: &str) -> Option<usize> {
        let (bytes, mask) = Self::parse_pattern(pattern);
        
        // Read memory in chunks
        const CHUNK_SIZE: usize = 0x10000;
        let mut offset = 0;

        while offset < size {
            let read_size = std::cmp::min(CHUNK_SIZE, size - offset);
            
            if let Ok(buffer) = self.process.read_bytes(start + offset, read_size) {
                if let Some(pos) = Self::find_pattern(&buffer, &bytes, &mask) {
                    return Some(start + offset + pos);
                }
            }
            
            offset += CHUNK_SIZE - bytes.len();
        }

        None
    }

    fn parse_pattern(pattern: &str) -> (Vec<u8>, Vec<bool>) {
        let tokens: Vec<&str> = pattern.split_whitespace().collect();
        let mut bytes = Vec::new();
        let mut mask = Vec::new();

        for token in tokens {
            if token == "?" || token == "??" {
                bytes.push(0);
                mask.push(false);
            } else {
                if let Ok(byte) = u8::from_str_radix(token, 16) {
                    bytes.push(byte);
                    mask.push(true);
                }
            }
        }

        (bytes, mask)
    }

    fn find_pattern(buffer: &[u8], pattern: &[u8], mask: &[bool]) -> Option<usize> {
        if pattern.is_empty() || buffer.len() < pattern.len() {
            return None;
        }

        for i in 0..=(buffer.len() - pattern.len()) {
            let mut found = true;

            for j in 0..pattern.len() {
                if mask[j] && buffer[i + j] != pattern[j] {
                    found = false;
                    break;
                }
            }

            if found {
                return Some(i);
            }
        }

        None
    }

    // Scan for string
    pub fn scan_string(&self, start: usize, size: usize, search: &str) -> Option<usize> {
        let bytes = search.as_bytes();
        const CHUNK_SIZE: usize = 0x10000;
        let mut offset = 0;

        while offset < size {
            let read_size = std::cmp::min(CHUNK_SIZE, size - offset);
            
            if let Ok(buffer) = self.process.read_bytes(start + offset, read_size) {
                if let Some(pos) = buffer.windows(bytes.len()).position(|w| w == bytes) {
                    return Some(start + offset + pos);
                }
            }
            
            offset += CHUNK_SIZE - bytes.len();
        }

        None
    }

    // Resolve RIP-relative address (for x64)
    pub fn resolve_rip_relative(&self, address: usize, offset: usize, instruction_size: usize) -> Result<usize, ()> {
        let rel_offset = self.process.read::<i32>(address + offset)?;
        Ok((address + instruction_size).wrapping_add(rel_offset as usize))
    }
}
