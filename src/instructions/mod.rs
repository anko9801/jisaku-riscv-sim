use crate::processor::State;
use crate::processor::XprName::ra;

pub trait Instruction {
    // fn rd(&self) -> XprName;
    // fn rs1(&self) -> XprName;
    // fn rs2(&self) -> XprName;
    // fn i_imm(&self) -> u64;
    // fn asm(&self) -> String;
    fn effect(&self, state: &mut State);
}

pub struct LUI(pub u32);
impl Instruction for LUI {
    fn effect(&self, state: &mut State) {}
}

pub struct JALR(pub u32);
impl Instruction for JALR {
    fn effect(&self, state: &mut State) {}
}

pub struct C_JR(pub u16);
impl Instruction for C_JR {
    fn effect(&self, state: &mut State) {
        println!("ret");
        state.pc = state.regs.get(ra);
    }
}
