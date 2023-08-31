pub mod rv32i;
pub mod rvc;

use crate::processor::State;

pub trait Instruction {
    // fn rd(&self) -> XprName;
    // fn rs1(&self) -> XprName;
    // fn rs2(&self) -> XprName;
    // fn i_imm(&self) -> u64;
    // fn asm(&self) -> String;
    fn effect(&self, state: &mut State);
}

pub fn x(value: u16, from: usize, size: usize) -> i64 {
    ((value >> from) & ((1 << size) - 1)) as i64
}
