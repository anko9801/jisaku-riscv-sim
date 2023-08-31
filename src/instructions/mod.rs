use crate::processor::XprName::ra;
use crate::processor::{State, XprName};

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
        state.regs.get(ra);
        state.pc += 2;
    }
}

pub struct C_ADDI(pub u16);
impl Instruction for C_ADDI {
    fn effect(&self, state: &mut State) {
        let v = self.0;
        let imm = x(v, 2, 5) - (x(v, 12, 1) << 5);
        let rd = x(v, 7, 5) as usize;
        println!(
            "addi {:?}, {:?}, {}",
            XprName::n(rd as i64).unwrap(),
            XprName::n(rd as i64).unwrap(),
            imm
        );
        let rd_value = state.regs.get(rd);
        state.regs.set(rd, rd_value + imm);
        state.pc += 2;
    }
}

pub struct C_NOP(pub u16);
impl Instruction for C_NOP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub fn x(value: u16, from: usize, size: usize) -> i64 {
    ((value >> from) & ((1 << size) - 1)) as i64
}
