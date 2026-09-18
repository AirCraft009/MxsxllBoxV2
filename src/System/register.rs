pub const REGISTERS: usize = 26;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Registers {
    PC,
    MSP,
    MBP,
    MAX,
    MBX,
    MCX,
    MDX,
    MSI,
    MDI,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    CR1,
    CR2,
    CR3,
    CR4,
    CR5,
    CR6,
    CR7,
    CR8,
    NOREG,
}

impl PartialEq for Registers {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Flags {
    pub zf: bool,
    pub cf: bool,
    pub sf: bool,
    pub of: bool,
    pub irf: bool,
}
