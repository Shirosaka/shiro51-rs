pub(crate) const MEMORY_XRAM_SIZE: usize = 0x1000;

#[derive(Debug)]
#[allow(unused)]
pub struct EMI {
    xram: [u8; MEMORY_XRAM_SIZE],
}

impl Default for EMI {
    fn default() -> Self {
        Self {
            xram: [0u8; MEMORY_XRAM_SIZE],
        }
    }
}
