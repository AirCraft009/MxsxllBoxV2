use crate::system::register::{Flags, Registers, REGISTERS};
use crate::system::memory::Memory;


pub struct CPU {
    pub flags: Flags,
    pub registers: [u64; REGISTERS],
    pub memory: Memory,
    pub running: bool,
}

impl CPU {

    pub fn default() -> CPU {
        CPU {
            registers: [0u64; REGISTERS],
            memory: Memory::default(),
            flags: Flags::default(),
            running: false,
        }
    }

    pub fn new(memory: Memory) -> CPU {
        CPU {
            registers: [0u64; REGISTERS],
            memory,
            flags: Flags::default(),
            running: false,
        }
    }
    #[inline(always)]
    pub fn set_register(&mut self, register: Registers, value: u64) {
        self.registers[register as usize] = value;
    }

    #[inline(always)]
    pub fn get_register(&mut self, register: Registers) -> u64 {
        self.registers[register as usize]
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.step();
        }
    }

    fn step(&mut self) {
        //fetch
        //decode
        //execute
    }
    
    fn fetch(&self){
        
    }
    
    fn decode(){
        
    }

    fn execute(&mut self) {

    }
}