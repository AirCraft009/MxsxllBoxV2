use crate::system::cpu::CPU;
use crate::system::Instructions::{ITypes, Opcode};
use crate::system::Instructions::ITypes::{Op, OpImm, OpReg, OpRegImm, OpRegReg, ILLEGAL};
use crate::system::Instructions::Opcode::HALT;

pub fn decode_instruction(cpu: &CPU, addr: u64){
    // 1b op
    // 5bit(1byte) reg 1
    // 8bytes immediate
    // = 10 bytes
    let op = cpu.memory.read_byte(addr);
    let len = instruction_length_jtable(op);
    let instruction = cpu.memory.read_bytes(addr,(len-1) as usize);
}

pub fn instruction_length_jtable(op: u8) -> u64{
    match get_instruction_type(op) {
        ILLEGAL => {0}
        Op => {1}
        OpReg => {2}
        OpImm => {9}
        OpRegReg => {3}
        OpRegImm => {10}
    }
}
pub fn get_instruction_type(op: u8) -> ITypes{
    match op {
        x if x == Opcode::ADD as u8
            || x == Opcode::SUB as u8
            || x == Opcode::MUL as u8
            || x == Opcode::OR as u8
            || x == Opcode::AND as u8
            || x == Opcode::XOR as u8
            || x == Opcode::TEST as u8
            || x == Opcode::CMP as u8
            || x == Opcode::LSHIFT as u8
            || x == Opcode::RSHIFT as u8 => OpRegReg,

        x if x == Opcode::PUSH as u8
            || x == Opcode::POP as u8
            || x == Opcode::NOT as u8 => OpReg,

        x if x == Opcode::PUSHA as u8
            || x == Opcode::POPA as u8
            || x == Opcode::HALT as u8 => Op,

        x if x == Opcode::PUSHI as u8 => OpImm,
        // Everything with an immediate
        x if x == Opcode::ADDI as u8
            || x == Opcode::SUBI as u8
            || x == Opcode::MULI as u8
            || x == Opcode::ORI as u8
            || x == Opcode::ANDI as u8
            || x == Opcode::XORI as u8
            || x == Opcode::NOTI as u8
            || x == Opcode::TESTI as u8
            || x == Opcode::CMPI as u8
            || x == Opcode::JUMPA as u8
            || x == Opcode::JZ as u8
            || x == Opcode::JC as u8
            || x == Opcode::JO as u8
            || x == Opcode::JNZ as u8
            || x == Opcode::JNC as u8
            || x == Opcode::JNO as u8
            || x == Opcode::JG as u8
            || x == Opcode::JL as u8
            || x == Opcode::LSHIFTI as u8
            || x == Opcode::RSHIFTI as u8 => OpRegImm,

        _ => ILLEGAL,
    }
}