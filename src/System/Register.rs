#[repr(usize)]
enum Registers {
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
    M8
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