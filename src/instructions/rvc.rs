use crate::processor::{State, XprName};

use super::{x, Instruction};

pub struct C_JR(u16);
impl C_JR {
    pub fn new(inst: u16) -> Self {
        C_JR(inst)
    }
}
impl Instruction for C_JR {
    fn effect(&self, state: &mut State) {
        println!("ret");
        state.regs.get(XprName::ra);
        state.pc += 2;
    }
}

pub struct C_ADDI {
    imm: i64,
    rd: XprName,
}
impl C_ADDI {
    pub fn new(inst: u16) -> Self {
        let imm = x(inst, 2, 5) - (x(inst, 12, 1) << 5);
        let rd = x(inst, 7, 5) as usize;
        C_ADDI {
            imm,
            rd: XprName::n(rd as i64).unwrap(),
        }
    }
}
impl Instruction for C_ADDI {
    fn effect(&self, state: &mut State) {
        println!("addi {:?}, {:?}, {}", self.rd, self.rd, self.imm);
        let rd_value = state.regs.get(self.rd);
        state.regs.set(self.rd, rd_value + self.imm);
        state.pc += 2;
    }
}

pub struct C_NOP(u16);
impl C_NOP {
    pub fn new(inst: u16) -> Self {
        C_NOP(inst)
    }
}
impl Instruction for C_NOP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}
