pub struct CPU<'a> {
    registers: Vec<u64>,
    pc: &'a u64,
    msp: &'a u64,
    
}