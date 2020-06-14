use std::io::Read;

/// Barrows heavily from https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/cartridge_driver.rs
/// Some small changes, but hey, this is a learning exercise.
pub struct Rom {
    /// memory, size defined as 0xFFF - 0x200
    pub memory: [u8; 3584],
    pub size: usize,
} 

impl Rom {

        /// Loads rom data based on a filename (in this case from main args)
        pub fn new(filename: &str) -> Rom {

            let mut file = std::fs::File::open(filename).expect("File not found!");
            //defined as 4096 - 512 bytes.
            let mut buffer = [0u8; 3584];
    
            let rom_size = if let Ok(rom_size) = file.read(&mut buffer) {
                rom_size
            } else {
                0
            };
    
            Rom {
                memory: buffer,
                size: rom_size,
            }
    
        }
}