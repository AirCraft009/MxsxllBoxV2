use crate::system::Register::Flags;
use crate::system::specifications::Memory;

pub struct CPU {
    pub registers: Box<[u64; 16]>,
    pub memory: Memory,
    pub flags: Flags,
}

impl CPU {

    pub fn default() -> CPU {
        CPU {
            registers: Box::new([0u64; 16]),
            memory: Memory::default(),
            flags: Flags::default(),
        }
    }
    #[inline(always)]
    pub fn set_register(&mut self, register: usize, value: u64) {
        self.registers[register] = value;
    }

    #[inline(always)]
    pub fn get_register(&mut self, register: usize) -> u64 {
        self.registers[register]
    }
}