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
    let imm = x(inst, 20, 12);
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
    fn effect(&self, state: &mut State) {}
}

pub struct AUIPC(u32);
impl AUIPC {
    pub fn new(inst: u32) -> Self {
        AUIPC(inst)
    }
}
impl Instruction for AUIPC {
    fn effect(&self, state: &mut State) {}
}

pub struct JAL(u32);
impl JAL {
    pub fn new(inst: u32) -> Self {
        JAL(inst)
    }
}
impl Instruction for JAL {
    fn effect(&self, state: &mut State) {}
}

pub struct JALR(u32);
impl JALR {
    pub fn new(inst: u32) -> Self {
        JALR(inst)
    }
}
impl Instruction for JALR {
    fn effect(&self, state: &mut State) {}
}

pub struct LB(u32);
impl LB {
    pub fn new(inst: u32) -> Self {
        LB(inst)
    }
}
impl Instruction for LB {
    fn effect(&self, state: &mut State) {}
}

pub struct ADDI(u32);
impl ADDI {
    pub fn new(inst: u32) -> Self {
        ADDI(inst)
    }
}
impl Instruction for ADDI {
    fn effect(&self, state: &mut State) {}
}
