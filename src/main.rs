use crate::system::cpu::CPU;
use crate::system::instructions::Opcode::ADDI;
use crate::system::memory::Memory;
use crate::system::register::Registers::M1;

mod system;

fn main() {
    let mut mem = Memory::default();
    mem.load_rom(
        &[
            ADDI as u8, M1 as u8, 0x0, 0x0, 0x0, 10
        ]
    );
    let mut cpu = CPU::new(mem);
    cpu.run()
}
