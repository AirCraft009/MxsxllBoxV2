pub const REGISTERS: usize = 24;

#[repr(usize)]
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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Flags{
    //zero
    pub zf: bool,
    //carry
    pub cf: bool,
    //sign
    pub sf: bool,
    //overflow
    pub of: bool,
    //CONTROL FLAGS:
    //interrupt
    pub irf: bool,
}