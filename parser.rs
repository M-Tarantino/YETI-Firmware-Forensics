pub struct HeaderParser;

impl HeaderParser {
    pub fn identify_elf(data: &[u8]) -> Option<String> {
        if data.starts_with(b"\x7fELF") && data.len() > 18 {
            let arch = match data[18] {
                40 => "ARM",
                62 => "x86_64",
                3 => "x86",
                8 => "MIPS",
                _ => "Unknown",
            };
            return Some(format!("ELF Binary ({})", arch));
        }
        None
    }
}