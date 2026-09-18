use crate::system::register::{Flags, Registers, REGISTERS};
use crate::system::memory::Memory;
use crate::system::register::Registers::PC;
use crate::system::instructions::Instruction;
use crate::system::instructions::Opcode::{HALT, NONE};

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
    pub fn get_register(&self, register: Registers) -> u64 {
        self.registers[register as usize]
    }

    #[inline(always)]
    pub fn set_reg(&mut self, register: u8, value: u64) {
        self.registers[register as usize] = value;
    }

    #[inline(always)]
    pub fn get_reg(&self, register: u8) -> u64 {
        self.registers[register as usize]
    }

    #[inline(always)]
    pub fn modify_reg(&mut self, register: u8) -> &mut u64 {
        &mut self.registers[register as usize]
    }


    #[inline(always)]
    pub fn modify_register(&mut self, register: Registers) -> &mut u64 {
        &mut self.registers[register as usize]
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.step();
        }
    }

    fn step(&mut self) {
        let pc = self.get_register(PC);
        let ins = self.fetch_decode_instruction(pc);
        self.execute_handler(&ins);
    }

    fn execute(&mut self) {
        // legacy hook; real dispatch lives in execute_handler (handlers.rs).
    }
}
