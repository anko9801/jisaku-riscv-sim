use super::Instruction;
use crate::{
    processor::{State, XprName},
    utils::x,
};

pub fn rv32i_r_type(inst: u32) -> (XprName, XprName, XprName) {
    let rd = x(inst, 7, 5);
    let rs1 = x(inst, 15, 5);
    let rs2 = x(inst, 20, 5);
    (
        XprName::from_num(rd),
        XprName::from_num(rs1),
        XprName::from_num(rs2),
    )
}
pub fn rv32i_i_type(inst: u32) -> (XprName, XprName, i64) {
    let rd = x(inst, 7, 5);
    let rs1 = x(inst, 15, 5);
    let imm = x(inst, 20, 11) - (x(inst, 31, 1) << 11);
    (XprName::from_num(rd), XprName::from_num(rs1), imm)
}
pub fn rv32i_s_type(inst: u32) -> (XprName, XprName, i64) {
    let imm = x(inst, 7, 5) + (x(inst, 25, 7) << 5);
    let rs1 = x(inst, 15, 5);
    let rs2 = x(inst, 20, 5);
    (XprName::from_num(rs1), XprName::from_num(rs2), imm)
}
pub fn rv32i_b_type(inst: u32) -> (XprName, XprName, i64) {
    let imm =
        (x(inst, 7, 1) << 11) + x(inst, 8, 4) + (x(inst, 25, 6) << 5) + (x(inst, 31, 1) << 12);
    let rs1 = x(inst, 15, 5);
    let rs2 = x(inst, 20, 5);
    (XprName::from_num(rs1), XprName::from_num(rs2), imm)
}
pub fn rv32i_u_type(inst: u32) -> (XprName, i64) {
    let rd = x(inst, 7, 5);
    let imm = x(inst, 12, 20);
    (XprName::from_num(rd), imm)
}
pub fn rv32i_j_type(inst: u32) -> (XprName, i64) {
    let rd = x(inst, 7, 5);
    let imm = (x(inst, 12, 8) << 12)
        + (x(inst, 20, 1) << 11)
        + (x(inst, 21, 10) << 1)
        + (x(inst, 31, 1) << 20);
    (XprName::from_num(rd), imm)
}

pub struct LUI(u32);
impl LUI {
    pub fn new(inst: u32) -> Self {
        LUI(inst)
    }
}
impl Instruction for LUI {
    fn execute(&self, state: &mut State) {
        state.pc += 4;
    }
}

pub struct AUIPC {
    rd: XprName,
    imm: i64,
}
impl AUIPC {
    pub fn new(inst: u32) -> Self {
        let (rd, imm) = rv32i_u_type(inst);
        AUIPC { rd, imm }
    }
}
impl Instruction for AUIPC {
    fn execute(&self, state: &mut State) {
        println!("auipc {:?}, {:#x}", self.rd, self.imm);
        state.set_reg(self.rd, state.pc + (self.imm << 12));
        state.pc += 4;
    }
}

pub struct JAL {
    rd: XprName,
    imm: i64,
}
impl JAL {
    pub fn new(inst: u32) -> Self {
        let (rd, imm) = rv32i_j_type(inst);
        JAL { rd, imm }
    }
}
impl Instruction for JAL {
    fn execute(&self, state: &mut State) {
        println!("jal {:?} {}", self.rd, self.imm);
        state.pc += 4;
    }
}

pub struct JALR {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl JALR {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        JALR { rd, rs1, imm }
    }
}
impl Instruction for JALR {
    fn execute(&self, state: &mut State) {
        state.pc += 4;
    }
}

pub struct LB(u32);
impl LB {
    pub fn new(inst: u32) -> Self {
        LB(inst)
    }
}
impl Instruction for LB {
    fn execute(&self, state: &mut State) {
        state.pc += 4;
    }
}

pub struct ADDI {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl ADDI {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        ADDI { rd, rs1, imm }
    }
}
impl Instruction for ADDI {
    fn execute(&self, state: &mut State) {
        println!("addi {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        state.set_reg(self.rd, state.get_reg(self.rs1) + self.imm);
        state.pc += 4;
    }
}
