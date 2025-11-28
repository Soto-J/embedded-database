pub mod mcu;

#[allow(non_camel_case_types)]
pub struct ESP32S3_CONFIG;

impl ESP32S3_CONFIG {
    // 16 MB
    pub const FLASH_MEMORY_SIZE: usize = 16 * 1024 * 1024;
    // 8 MB
    pub const PSRAM_MEMORY_SIZE: usize = 8 * 1024 * 1024;
    // 512 KB
    pub const SRAM_MEMORY_SIZE: usize = 512 * 1024;
}

pub const MAX_RECORD_SIZE: usize = 2048; // 2 KB
pub const MAX_DATABASE_SIZE: usize = 128_000; // 128 KB total
pub const MAX_ALLOC: usize = 16_384; // 16 KB scratch
