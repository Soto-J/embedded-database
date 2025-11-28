use super::ESP32S3_CONFIG;

pub trait DeviceProfile {
    const FLASH: usize;
    const SRAM: usize;
    const PSRAM: usize;
}

pub struct ESP32S3;

impl DeviceProfile for ESP32S3 {
    const FLASH: usize = ESP32S3_CONFIG::FLASH_MEMORY_SIZE;
    const SRAM: usize = ESP32S3_CONFIG::SRAM_MEMORY_SIZE;
    const PSRAM: usize = ESP32S3_CONFIG::PSRAM_MEMORY_SIZE;
}
