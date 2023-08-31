pub mod rv32i;
pub mod rvc;

use crate::processor::State;

pub trait Instruction {
    // fn asm(&self) -> String;
    fn effect(&self, state: &mut State);
}
