use crate::system::register::Registers;

#[repr(u8)]
pub enum Opcode {
    NONE,
    NOOP,
    ADD,
    SUB,
    MUL,
    ADDI,
    SUBI,
    MULI,
    PUSH,
    POP,
    PUSHI,
    PUSHA,
    POPA,
    OR,
    AND,
    XOR,
    NOT,
    // non-destructive and (only sets flags)
    TEST,
    CMP,
    ORI,
    ANDI,
    XORI,
    NOTI,
    TESTI,
    CMPI,
    LJUMP,
    JUMP,
    JUMPA,
    JZ,
    JC,
    JO,
    JNZ,
    JNC,
    JNO,
    JG,
    JL,
    LSHIFT,
    LSHIFTI,
    RSHIFT,
    RSHIFTI,
    HALT
}

pub enum ITypes{
    ILLEGAL,
    Op,
    OpReg,
    OpImm,
    OpRegReg,
    OpRegImm,
}

pub struct Instruction {
    i_type: ITypes,
    opcode: u8,
    rx: u8,
    ry: u8,
    immi: u64,
}

impl Instruction {
    pub fn new(opcode: u8, rx: u8, ry: u8, immi: u64, i_type: ITypes) -> Self {
        Instruction{
            i_type,
            opcode,
            rx,
            ry,
            immi
        }
    }
}