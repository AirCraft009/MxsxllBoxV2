use crate::system::cpu::CPU;
use crate::system::instructions::Instruction;
use crate::system::instructions::Opcode;
use crate::system::register::Registers;

impl CPU {
    #[inline]
    fn advance_pc(&mut self, instruction: &Instruction) {
        *self.modify_register(Registers::PC) += instruction.size as u64;
    }

    fn set_alu_flags_add(&mut self, a: u64, b: u64, result: u64) {
        self.flags.zf = result == 0;
        self.flags.sf = (result as i64) < 0;
        self.flags.cf = result < a;
        let a_s = a as i64;
        let b_s = b as i64;
        let r_s = result as i64;
        self.flags.of = (a_s >= 0) == (b_s >= 0) && (r_s < 0) != (a_s < 0);
    }

    fn set_alu_flags_sub(&mut self, a: u64, b: u64, result: u64) {
        self.flags.zf = result == 0;
        self.flags.sf = (result as i64) < 0;
        self.flags.cf = a < b;
        let a_s = a as i64;
        let b_s = b as i64;
        let r_s = result as i64;
        self.flags.of = (a_s >= 0) != (b_s >= 0) && (r_s < 0) == (a_s < 0);
    }

    fn set_alu_flags_logic(&mut self, result: u64) {
        self.flags.zf = result == 0;
        self.flags.sf = (result as i64) < 0;
        self.flags.cf = false;
        self.flags.of = false;
    }

    fn set_mul_flags(&mut self, a: u64, b: u64, result: u64) {
        self.flags.zf = result == 0;
        self.flags.sf = (result as i64) < 0;
        let overflow = (a as u128).wrapping_mul(b as u128) > u64::MAX as u128;
        self.flags.cf = overflow;
        self.flags.of = overflow;
    }
}

impl CPU {
    // ------------------------------------------------------------------ control

    fn noop(&mut self, instruction: &Instruction) {
        self.advance_pc(instruction);
    }

    fn halt(&mut self, _instruction: &Instruction) {
        self.running = false;
    }

    // ------------------------------------------------------------------ ALU reg,reg

    fn add(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a.wrapping_add(b);
        self.set_reg(rx, result);
        self.set_alu_flags_add(a, b, result);
        self.advance_pc(instruction);
    }

    fn sub(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a.wrapping_sub(b);
        self.set_reg(rx, result);
        self.set_alu_flags_sub(a, b, result);
        self.advance_pc(instruction);
    }

    fn mul(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a.wrapping_mul(b);
        self.set_reg(rx, result);
        self.set_mul_flags(a, b, result);
        self.advance_pc(instruction);
    }

    fn or(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a | b;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn and(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a & b;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn xor(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a ^ b;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn test(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a & b;
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn cmp(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let a = self.get_reg(rx);
        let b = self.get_reg(ry);
        let result = a.wrapping_sub(b);
        self.set_alu_flags_sub(a, b, result);
        self.advance_pc(instruction);
    }

    fn lshift(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let mut value = self.get_reg(rx);
        let count = self.get_reg(ry) & 63;
        if count > 0 {
            self.flags.cf = ((value >> (64 - count)) & 1) != 0;
            value <<= count;
            self.flags.of = self.flags.cf;
        } else {
            self.flags.cf = false;
            self.flags.of = false;
        }
        self.set_reg(rx, value);
        self.flags.zf = value == 0;
        self.flags.sf = (value as i64) < 0;
        self.advance_pc(instruction);
    }

    fn rshift(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let ry = instruction.ry;
        let mut value = self.get_reg(rx);
        let count = self.get_reg(ry) & 63;
        if count > 0 {
            self.flags.cf = ((value >> (count - 1)) & 1) != 0;
            value >>= count;
        } else {
            self.flags.cf = false;
        }
        self.flags.of = false;
        self.set_reg(rx, value);
        self.flags.zf = value == 0;
        self.flags.sf = (value as i64) < 0;
        self.advance_pc(instruction);
    }

    // ------------------------------------------------------------------ ALU reg,imm

    fn addi(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        println!("ADDING: {} to {}", instruction.immi, rx);
        let a = self.get_reg(rx);
        let b = instruction.immi;
        let result = a.wrapping_add(b);
        self.set_reg(rx, result);
        self.set_alu_flags_add(a, b, result);
        self.advance_pc(instruction);
    }

    fn subi(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let b = instruction.immi;
        let result = a.wrapping_sub(b);
        self.set_reg(rx, result);
        self.set_alu_flags_sub(a, b, result);
        self.advance_pc(instruction);
    }

    fn muli(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let b = instruction.immi;
        let result = a.wrapping_mul(b);
        self.set_reg(rx, result);
        self.set_mul_flags(a, b, result);
        self.advance_pc(instruction);
    }

    fn ori(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let result = a | instruction.immi;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn andi(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let result = a & instruction.immi;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn xori(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let result = a ^ instruction.immi;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn noti(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let result = !instruction.immi;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn testi(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let result = a & instruction.immi;
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    fn cmpi(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let a = self.get_reg(rx);
        let b = instruction.immi;
        let result = a.wrapping_sub(b);
        self.set_alu_flags_sub(a, b, result);
        self.advance_pc(instruction);
    }

    fn lshiftt(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let mut value = self.get_reg(rx);
        let count = instruction.immi & 63;
        if count > 0 {
            self.flags.cf = ((value >> (64 - count)) & 1) != 0;
            value <<= count;
            self.flags.of = self.flags.cf;
        } else {
            self.flags.cf = false;
            self.flags.of = false;
        }
        self.set_reg(rx, value);
        self.flags.zf = value == 0;
        self.flags.sf = (value as i64) < 0;
        self.advance_pc(instruction);
    }

    fn rshiftt(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let mut value = self.get_reg(rx);
        let count = instruction.immi & 63;
        if count > 0 {
            self.flags.cf = ((value >> (count - 1)) & 1) != 0;
            value >>= count;
        } else {
            self.flags.cf = false;
        }
        self.flags.of = false;
        self.set_reg(rx, value);
        self.flags.zf = value == 0;
        self.flags.sf = (value as i64) < 0;
        self.advance_pc(instruction);
    }

    // ------------------------------------------------------------------ stack / not

    fn push(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let val = self.get_reg(rx);
        let msp = self.get_register(Registers::MSP);
        let new_msp = msp.wrapping_sub(8);
        self.memory.write_long(new_msp, val);
        self.set_register(Registers::MSP, new_msp);
        self.advance_pc(instruction);
    }

    fn pop(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let msp = self.get_register(Registers::MSP);
        let val = self.memory.read_long(msp);
        let new_msp = msp.wrapping_add(8);
        self.set_register(Registers::MSP, new_msp);
        self.set_reg(rx, val);
        self.advance_pc(instruction);
    }

    fn pushi(&mut self, instruction: &Instruction) {
        let msp = self.get_register(Registers::MSP);
        let new_msp = msp.wrapping_sub(8);
        self.memory.write_long(new_msp, instruction.immi);
        self.set_register(Registers::MSP, new_msp);
        self.advance_pc(instruction);
    }

    fn pusha(&mut self, _instruction: &Instruction) {
        for i in 0..25 {
            let reg = unsafe { std::mem::transmute::<u8, Registers>(i) };
            let val = self.get_register(reg);
            let msp = self.get_register(Registers::MSP);
            let new_msp = msp.wrapping_sub(8);
            self.memory.write_long(new_msp, val);
            self.set_register(Registers::MSP, new_msp);
        }
        self.advance_pc(_instruction);
    }

    fn popa(&mut self, _instruction: &Instruction) {
        for i in (0..25).rev() {
            let reg = unsafe { std::mem::transmute::<u8, Registers>(i) };
            let msp = self.get_register(Registers::MSP);
            let val = self.memory.read_long(msp);
            let new_msp = msp.wrapping_add(8);
            self.set_register(Registers::MSP, new_msp);
            self.set_register(reg, val);
        }
        self.advance_pc(_instruction);
    }

    fn not(&mut self, instruction: &Instruction) {
        let rx = instruction.rx;
        let value = self.get_reg(rx);
        let result = !value;
        self.set_reg(rx, result);
        self.set_alu_flags_logic(result);
        self.advance_pc(instruction);
    }

    // ------------------------------------------------------------------ jumps

    fn jump(&mut self, instruction: &Instruction) {
        // relative: immi is i64 offset
        let pc = self.get_register(Registers::PC) as i64;
        let offset = instruction.immi as i64;
        let new_pc = (pc + offset) as u64;
        self.set_register(Registers::PC, new_pc);
    }

    fn jumpa(&mut self, instruction: &Instruction) {
        // absolute: immi is u64 target address
        self.set_register(Registers::PC, instruction.immi);
    }

    fn ljump(&mut self, instruction: &Instruction) {
        // absolute: immi is u64 target address (rx ignored)
        self.set_register(Registers::PC, instruction.immi);
    }

    fn jz(&mut self, instruction: &Instruction) {
        if self.flags.zf {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jnz(&mut self, instruction: &Instruction) {
        if !self.flags.zf {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jc(&mut self, instruction: &Instruction) {
        if self.flags.cf {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jnc(&mut self, instruction: &Instruction) {
        if !self.flags.cf {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jo(&mut self, instruction: &Instruction) {
        if self.flags.of {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jno(&mut self, instruction: &Instruction) {
        if !self.flags.of {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jg(&mut self, instruction: &Instruction) {
        // signed greater: ZF=0 and SF==OF
        if !self.flags.zf && (self.flags.sf == self.flags.of) {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    fn jl(&mut self, instruction: &Instruction) {
        // signed less: SF != OF
        if self.flags.sf != self.flags.of {
            let pc = self.get_register(Registers::PC) as i64;
            let offset = instruction.immi as i64;
            self.set_register(Registers::PC, (pc + offset) as u64);
        } else {
            self.advance_pc(instruction);
        }
    }

    // ------------------------------------------------------------------ dispatch

    pub fn execute_handler(&mut self, instruction: &Instruction) {
        match instruction.opcode {
            x if x == Opcode::NOOP as u8 => self.noop(instruction),
            x if x == Opcode::HALT as u8 => self.halt(instruction),
            x if x == Opcode::ADD as u8 => self.add(instruction),
            x if x == Opcode::SUB as u8 => self.sub(instruction),
            x if x == Opcode::MUL as u8 => self.mul(instruction),
            x if x == Opcode::OR as u8 => self.or(instruction),
            x if x == Opcode::AND as u8 => self.and(instruction),
            x if x == Opcode::XOR as u8 => self.xor(instruction),
            x if x == Opcode::TEST as u8 => self.test(instruction),
            x if x == Opcode::CMP as u8 => self.cmp(instruction),
            x if x == Opcode::LSHIFT as u8 => self.lshift(instruction),
            x if x == Opcode::RSHIFT as u8 => self.rshift(instruction),
            x if x == Opcode::ADDI as u8 => self.addi(instruction),
            x if x == Opcode::SUBI as u8 => self.subi(instruction),
            x if x == Opcode::MULI as u8 => self.muli(instruction),
            x if x == Opcode::ORI as u8 => self.ori(instruction),
            x if x == Opcode::ANDI as u8 => self.andi(instruction),
            x if x == Opcode::XORI as u8 => self.xori(instruction),
            x if x == Opcode::NOTI as u8 => self.noti(instruction),
            x if x == Opcode::TESTI as u8 => self.testi(instruction),
            x if x == Opcode::CMPI as u8 => self.cmpi(instruction),
            x if x == Opcode::LSHIFTI as u8 => self.lshiftt(instruction),
            x if x == Opcode::RSHIFTI as u8 => self.rshiftt(instruction),
            x if x == Opcode::PUSH as u8 => self.push(instruction),
            x if x == Opcode::POP as u8 => self.pop(instruction),
            x if x == Opcode::PUSHI as u8 => self.pushi(instruction),
            x if x == Opcode::PUSHA as u8 => self.pusha(instruction),
            x if x == Opcode::POPA as u8 => self.popa(instruction),
            x if x == Opcode::NOT as u8 => self.not(instruction),
            x if x == Opcode::JUMP as u8 => self.jump(instruction),
            x if x == Opcode::JUMPA as u8 => self.jumpa(instruction),
            x if x == Opcode::LJUMP as u8 => self.ljump(instruction),
            x if x == Opcode::JZ as u8 => self.jz(instruction),
            x if x == Opcode::JNZ as u8 => self.jnz(instruction),
            x if x == Opcode::JC as u8 => self.jc(instruction),
            x if x == Opcode::JNC as u8 => self.jnc(instruction),
            x if x == Opcode::JO as u8 => self.jo(instruction),
            x if x == Opcode::JNO as u8 => self.jno(instruction),
            x if x == Opcode::JG as u8 => self.jg(instruction),
            x if x == Opcode::JL as u8 => self.jl(instruction),
            _ => self.advance_pc(instruction),
        }
    }
}
