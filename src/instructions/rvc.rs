use super::Instruction;
use crate::{
    processor::{State, XprName, XprName::sp},
    utils::x,
};

#[inline]
fn rvc_cr_type(inst: u16) -> (XprName, XprName) {
    let rs1 = x(inst, 7, 5);
    let rs2 = x(inst, 2, 5);
    (XprName::from_num(rs1), XprName::from_num(rs2))
}
#[inline]
fn rvc_ci_type(inst: u16) -> XprName {
    let rs1 = x(inst, 7, 5);
    XprName::from_num(rs1)
}
#[inline]
fn rvc_css_type(inst: u16) -> (XprName, i64) {
    let rs2 = x(inst, 2, 5);
    let imm = x(inst, 7, 6);
    (XprName::from_num(rs2), imm)
}
#[inline]
fn rvc_ciw_type(inst: u16) -> XprName {
    let rd = x(inst, 2, 3);
    XprName::from_num(rd + 0x10)
}
#[inline]
fn rvc_cl_type(inst: u16) -> (XprName, XprName) {
    let rd = x(inst, 2, 3);
    let rs1 = x(inst, 7, 3);
    (XprName::from_num(rd + 0x10), XprName::from_num(rs1 + 0x10))
}
#[inline]
fn rvc_cs_type(inst: u16) -> (XprName, XprName) {
    let rs2 = x(inst, 2, 3);
    let rs1 = x(inst, 7, 3);
    (XprName::from_num(rs2 + 0x10), XprName::from_num(rs1 + 0x10))
}
#[inline]
fn rvc_cb_type(inst: u16) -> XprName {
    let rs1 = x(inst, 7, 3);
    XprName::from_num(rs1 + 0x10)
}

pub struct C_ADDI4SPN(u16);
impl C_ADDI4SPN {
    pub fn new(inst: u16) -> Self {
        C_ADDI4SPN(inst)
    }
}
impl Instruction for C_ADDI4SPN {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FLD(u16);
impl C_FLD {
    pub fn new(inst: u16) -> Self {
        C_FLD(inst)
    }
}
impl Instruction for C_FLD {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LW(u16);
impl C_LW {
    pub fn new(inst: u16) -> Self {
        C_LW(inst)
    }
}
impl Instruction for C_LW {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LD {
    rd: XprName,
    rs1: XprName,
    imm: u64,
}
impl C_LD {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cs_type(inst);
        let imm = ((x(inst, 5, 2) << 6) + (x(inst, 10, 2) << 3)) as u64;
        C_LD { rd, rs1, imm }
    }
}
impl Instruction for C_LD {
    fn effect(&self, state: &mut State) {
        println!("ld");
        state.pc += 2;
    }
}

pub struct C_FSD(u16);
impl C_FSD {
    pub fn new(inst: u16) -> Self {
        C_FSD(inst)
    }
}
impl Instruction for C_FSD {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SW(u16);
impl C_SW {
    pub fn new(inst: u16) -> Self {
        C_SW(inst)
    }
}
impl Instruction for C_SW {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SD(u16);
impl C_SD {
    pub fn new(inst: u16) -> Self {
        C_SD(inst)
    }
}
impl Instruction for C_SD {
    fn effect(&self, state: &mut State) {
        println!("sd ");
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

pub struct C_JAL {
    offset: i64,
}
impl C_JAL {
    pub fn new(inst: u16) -> Self {
        let imm = (x(inst, 2, 1) << 5)
            + (x(inst, 3, 3) << 1)
            + (x(inst, 6, 1) << 7)
            + (x(inst, 7, 1) << 6)
            + (x(inst, 8, 1) << 10)
            + (x(inst, 9, 2) << 8)
            + (x(inst, 11, 1) << 4)
            + (x(inst, 12, 1) << 11);
        C_JAL { offset: imm }
    }
}
impl Instruction for C_JAL {
    fn effect(&self, state: &mut State) {
        println!("jal {}", self.offset);
        state.pc += 2;
    }
}

pub struct C_LI {
    rd: XprName,
    imm: i64,
}
impl C_LI {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_ci_type(inst);
        let imm = x(inst, 2, 5) + (x(inst, 12, 1) << 5);
        C_LI { rd, imm }
    }
}
impl Instruction for C_LI {
    fn effect(&self, state: &mut State) {
        println!("li {:?}, {}", self.rd, self.imm);
        state.pc += 2;
    }
}

pub struct C_SLLI(u16);
impl C_SLLI {
    pub fn new(inst: u16) -> Self {
        C_SLLI(inst)
    }
}
impl Instruction for C_SLLI {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SLLI64(u16);
impl C_SLLI64 {
    pub fn new(inst: u16) -> Self {
        C_SLLI64(inst)
    }
}
impl Instruction for C_SLLI64 {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FLDSP(u16);
impl C_FLDSP {
    pub fn new(inst: u16) -> Self {
        C_FLDSP(inst)
    }
}
impl Instruction for C_FLDSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LQSP(u16);
impl C_LQSP {
    pub fn new(inst: u16) -> Self {
        C_LQSP(inst)
    }
}
impl Instruction for C_LQSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LWSP(u16);
impl C_LWSP {
    pub fn new(inst: u16) -> Self {
        C_LWSP(inst)
    }
}
impl Instruction for C_LWSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FLWSP(u16);
impl C_FLWSP {
    pub fn new(inst: u16) -> Self {
        C_FLWSP(inst)
    }
}
impl Instruction for C_FLWSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LDSP {
    rd: XprName,
    offset: i64,
}
impl C_LDSP {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_ci_type(inst);
        let offset = (x(inst, 2, 3) << 6) + (x(inst, 5, 2) << 3) + (x(inst, 12, 1) << 5);
        C_LDSP { rd, offset }
    }
}
impl Instruction for C_LDSP {
    fn effect(&self, state: &mut State) {
        println!("ld {:?}, {}(sp)", self.rd, self.offset);
        state.pc += 2;
    }
}

pub struct C_JR(u16);
impl C_JR {
    pub fn new(inst: u16) -> Self {
        C_JR(inst)
    }
}
impl Instruction for C_JR {
    fn effect(&self, state: &mut State) {
        println!("ret");
        state.get_reg(XprName::ra);
        state.pc += 2;
    }
}

pub struct C_MV(u16);
impl C_MV {
    pub fn new(inst: u16) -> Self {
        C_MV(inst)
    }
}
impl Instruction for C_MV {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_EBREAK(u16);
impl C_EBREAK {
    pub fn new(inst: u16) -> Self {
        C_EBREAK(inst)
    }
}
impl Instruction for C_EBREAK {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_JALR(u16);
impl C_JALR {
    pub fn new(inst: u16) -> Self {
        C_JALR(inst)
    }
}
impl Instruction for C_JALR {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_ADD(u16);
impl C_ADD {
    pub fn new(inst: u16) -> Self {
        C_ADD(inst)
    }
}
impl Instruction for C_ADD {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FSDSP(u16);
impl C_FSDSP {
    pub fn new(inst: u16) -> Self {
        C_FSDSP(inst)
    }
}
impl Instruction for C_FSDSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SQSP(u16);
impl C_SQSP {
    pub fn new(inst: u16) -> Self {
        C_SQSP(inst)
    }
}
impl Instruction for C_SQSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SWSP(u16);
impl C_SWSP {
    pub fn new(inst: u16) -> Self {
        C_SWSP(inst)
    }
}
impl Instruction for C_SWSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FSWSP(u16);
impl C_FSWSP {
    pub fn new(inst: u16) -> Self {
        C_FSWSP(inst)
    }
}
impl Instruction for C_FSWSP {
    fn effect(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SDSP {
    rs2: XprName,
    offset: i64,
}
impl C_SDSP {
    pub fn new(inst: u16) -> Self {
        let (rs2, imm) = rvc_css_type(inst);
        let offset = (x(imm as u64, 0, 3) << 6) + (x(imm as u64, 3, 3) << 3);
        C_SDSP { rs2, offset }
    }
}
impl Instruction for C_SDSP {
    fn effect(&self, state: &mut State) {
        println!("sd {:?}, {}(sp)", self.rs2, self.offset);
        let sp_val = state.regs.get(sp);
        state.regs.set(self.rs2, sp_val + self.offset);
        state.pc += 2;
    }
}
