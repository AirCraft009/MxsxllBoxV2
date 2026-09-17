use crate::system::cpu::CPU;
use crate::system::instructions::Instruction;
use crate::system::register::Registers::PC;

const OP_LEN: u64 = 1;
const OP_REG_LEN: u64 = 2;
const OP_REG_REG_LEN: u64 = 3;
const OP_IMM_LEN: u64 = 9;
const OP_REG_IMM_LEN: u64 = 10;

impl CPU {
    fn noop(&mut self, instruction: &Instruction) {
        *self.modify_register(PC) += OP_LEN;
    }

    fn halt(&mut self, instruction: &Instruction) {
        self.running = false;
    }

    fn add(&mut self, instruction: &Instruction) {
        *
    }
}

impl CPU {
    pub fn execute_handler(&self, instruction: &Instruction){
        match instruction.opcode {
            NOOP=> {

            }
        }
    }
}