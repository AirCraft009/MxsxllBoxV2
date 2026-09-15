use crate::system::register::Flags;
use crate::system::memory::Memory;

pub struct CPU {
    pub flags: Flags,
    pub registers: [u64; 16],
    pub memory: Memory,
}

impl CPU {

    pub fn default() -> CPU {
        CPU {
            registers: [0u64; 16],
            memory: Memory::default(),
            flags: Flags::default(),
        }
    }

    pub fn new(memory: Memory) -> CPU {
        CPU {
            registers: [0u64; 16],
            memory,
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

    pub fn execute(&mut self) {

    }
}