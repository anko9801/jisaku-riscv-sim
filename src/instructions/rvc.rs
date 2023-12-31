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
    XprName::from_num(rd + 0x8)
}
#[inline]
fn rvc_cl_type(inst: u16) -> (XprName, XprName) {
    let rd = x(inst, 2, 3);
    let rs1 = x(inst, 7, 3);
    (XprName::from_num(rd + 0x8), XprName::from_num(rs1 + 0x8))
}
#[inline]
fn rvc_cs_type(inst: u16) -> (XprName, XprName) {
    let rs2 = x(inst, 2, 3);
    let rs1 = x(inst, 7, 3);
    (XprName::from_num(rs2 + 0x8), XprName::from_num(rs1 + 0x8))
}
#[inline]
fn rvc_cb_type(inst: u16) -> XprName {
    let rs1 = x(inst, 7, 3);
    XprName::from_num(rs1 + 0x8)
}
#[inline]
fn rvc_cj_type(inst: u16) -> i64 {
    let imm = (x(inst, 2, 1) << 5)
        + (x(inst, 3, 3) << 1)
        + (x(inst, 6, 1) << 7)
        + (x(inst, 7, 1) << 6)
        + (x(inst, 8, 1) << 10)
        + (x(inst, 9, 2) << 8)
        + (x(inst, 11, 1) << 4)
        + (x(inst, 12, 1) << 11);
    imm
}

pub struct C_ADDI4SPN(u16);
impl C_ADDI4SPN {
    pub fn new(inst: u16) -> Self {
        C_ADDI4SPN(inst)
    }
}
impl Instruction for C_ADDI4SPN {
    fn execute(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_FLD {
    rd: XprName,
    rs1: XprName,
    uimm: u64,
}
impl C_FLD {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cl_type(inst);
        let uimm: u64 =
            ((x(inst, 5, 1) << 6) + (x(inst, 6, 1) << 2) + (x(inst, 10, 3) << 3)) as u64;
        C_FLD { rd, rs1, uimm }
    }
}
impl Instruction for C_FLD {
    fn execute(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_LQ {
    rd: XprName,
    rs1: XprName,
    offset: u64,
}
impl C_LQ {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cl_type(inst);
        let offset: u64 =
            ((x(inst, 5, 2) << 6) + (x(inst, 10, 1) << 8) + (x(inst, 11, 2) << 4)) as u64;
        C_LQ { rd, rs1, offset }
    }
}
impl Instruction for C_LQ {
    fn execute(&self, state: &mut State) {
        // TODO: 128 bit
        println!("lq {:?}, {}({:?})", self.rd, self.offset, self.rs1);
        let addr = state.get_reg(self.rs1) + self.offset as i64;
        let mem = state.access_u64(addr);
        state.set_reg(self.rd, mem as i64);
        state.pc += 2;
    }
}

pub struct C_LW {
    rd: XprName,
    rs1: XprName,
    offset: u64,
}
impl C_LW {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cl_type(inst);
        let offset: u64 =
            ((x(inst, 5, 1) << 6) + (x(inst, 6, 1) << 2) + (x(inst, 10, 3) << 3)) as u64;
        C_LW { rd, rs1, offset }
    }
}
impl Instruction for C_LW {
    fn execute(&self, state: &mut State) {
        println!("lw {:?}, {}({:?})", self.rd, self.offset, self.rs1);
        let addr = state.get_reg(self.rs1) + self.offset as i64;
        let mem = state.access_u32(addr);
        state.set_reg(self.rd, mem as i64);
        state.pc += 2;
    }
}

pub struct C_LD {
    rd: XprName,
    rs1: XprName,
    offset: u64,
}
impl C_LD {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cs_type(inst);
        let offset = ((x(inst, 5, 2) << 6) + (x(inst, 10, 3) << 3)) as u64;
        C_LD { rd, rs1, offset }
    }
}
impl Instruction for C_LD {
    fn execute(&self, state: &mut State) {
        println!("ld {:?}, {}({:?})", self.rd, self.offset, self.rs1);
        let addr = state.get_reg(self.rs1) + self.offset as i64;
        let mem = state.access_u32(addr);
        state.set_reg(self.rd, mem as i64);
        state.pc += 2;
    }
}

pub struct C_FSD {
    rd: XprName,
    rs1: XprName,
    offset: u64,
}
impl C_FSD {
    pub fn new(inst: u16) -> Self {
        let (rd, rs1) = rvc_cl_type(inst);
        let offset: u64 =
            ((x(inst, 5, 1) << 6) + (x(inst, 6, 1) << 2) + (x(inst, 10, 3) << 3)) as u64;
        C_FSD { rd, rs1, offset }
    }
}
impl Instruction for C_FSD {
    fn execute(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SQ {
    rs1: XprName,
    rs2: XprName,
    offset: u64,
}
impl C_SQ {
    pub fn new(inst: u16) -> Self {
        let (rs2, rs1) = rvc_cs_type(inst);
        let offset: u64 =
            ((x(inst, 5, 2) << 6) + (x(inst, 10, 1) << 8) + (x(inst, 11, 2) << 4)) as u64;
        C_SQ { rs2, rs1, offset }
    }
}
impl Instruction for C_SQ {
    fn execute(&self, state: &mut State) {
        // TODO: 128 bit
        state.pc += 2;
    }
}

pub struct C_SW {
    rs1: XprName,
    rs2: XprName,
    offset: u64,
}
impl C_SW {
    pub fn new(inst: u16) -> Self {
        let (rs2, rs1) = rvc_cs_type(inst);
        let offset: u64 =
            ((x(inst, 5, 2) << 6) + (x(inst, 10, 1) << 8) + (x(inst, 11, 2) << 4)) as u64;
        C_SW { rs2, rs1, offset }
    }
}
impl Instruction for C_SW {
    fn execute(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_SD {
    rs1: XprName,
    rs2: XprName,
    offset: u64,
}
impl C_SD {
    pub fn new(inst: u16) -> Self {
        let (rs2, rs1) = rvc_cs_type(inst);
        let offset = ((x(inst, 5, 2) << 6) + (x(inst, 10, 3) << 3)) as u64;
        C_SD { rs2, rs1, offset }
    }
}
impl Instruction for C_SD {
    fn execute(&self, state: &mut State) {
        println!("sd {:?}, {}({:?})", self.rs2, self.offset, self.rs1);
        let addr = state.get_reg(self.rs1) + self.offset as i64;
        let rs2 = state.get_reg(self.rs2);
        state.store_u64(addr, rs2 as u64);
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
        let offset = rvc_cj_type(inst);
        C_JAL { offset }
    }
}
impl Instruction for C_JAL {
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
        println!("li {:?}, {}", self.rd, self.imm);
        state.set_reg(self.rd, self.imm);
        state.pc += 2;
    }
}

pub struct C_SRLI {
    rd: XprName,
    shamt: i64,
}
impl C_SRLI {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_cb_type(inst);
        let shamt = x(inst, 2, 5) + (x(inst, 12, 1) << 5);
        C_SRLI { rd, shamt }
    }
}
impl Instruction for C_SRLI {
    fn execute(&self, state: &mut State) {
        println!("srli {:?}, {}", self.rd, self.shamt);
        // TODO: Logical ?
        let rd = state.get_reg(self.rd);
        state.set_reg(self.rd, rd >> self.shamt);
        state.pc += 2;
    }
}

pub struct C_SRLI64 {
    rd: XprName,
    shamt: i64,
}
impl C_SRLI64 {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_cb_type(inst);
        C_SRLI64 { rd, shamt: 64 }
    }
}
impl Instruction for C_SRLI64 {
    fn execute(&self, state: &mut State) {
        println!("srli64 {:?}, {}", self.rd, self.shamt);
        let rd = state.get_reg(self.rd);
        state.set_reg(self.rd, rd >> self.shamt);
        state.pc += 2;
    }
}

pub struct C_SRAI {
    rd: XprName,
    shamt: i64,
}
impl C_SRAI {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_cb_type(inst);
        let shamt = x(inst, 2, 5) + (x(inst, 12, 1) << 5);
        C_SRAI { rd, shamt }
    }
}
impl Instruction for C_SRAI {
    fn execute(&self, state: &mut State) {
        println!("srai {:?}, {}", self.rd, self.shamt);
        // TODO: Arithmetic ?
        let rd = state.get_reg(self.rd);
        state.set_reg(self.rd, rd >> self.shamt);
        state.pc += 2;
    }
}

pub struct C_SRAI64 {
    rd: XprName,
    shamt: i64,
}
impl C_SRAI64 {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_cb_type(inst);
        C_SRAI64 { rd, shamt: 64 }
    }
}
impl Instruction for C_SRAI64 {
    fn execute(&self, state: &mut State) {
        println!("srai64 {:?}, {}", self.rd, self.shamt);
        let rd = state.get_reg(self.rd);
        state.set_reg(self.rd, rd >> self.shamt);
        state.pc += 2;
    }
}

pub struct C_ANDI {
    rd: XprName,
    imm: i64,
}
impl C_ANDI {
    pub fn new(inst: u16) -> Self {
        let rd = rvc_cb_type(inst);
        let imm = x(inst, 2, 5) + (x(inst, 12, 1) << 5);
        C_ANDI { rd, imm }
    }
}
impl Instruction for C_ANDI {
    fn execute(&self, state: &mut State) {
        println!("andi {:?}, {}", self.rd, self.imm);
        let rd = state.get_reg(self.rd);
        state.set_reg(self.rd, rd & self.imm);
        state.pc += 2;
    }
}

pub struct C_SUB {
    rd: XprName,
    rs2: XprName,
}
impl C_SUB {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_SUB { rd, rs2 }
    }
}
impl Instruction for C_SUB {
    fn execute(&self, state: &mut State) {
        println!("sub {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) - state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_XOR {
    rd: XprName,
    rs2: XprName,
}
impl C_XOR {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_XOR { rd, rs2 }
    }
}
impl Instruction for C_XOR {
    fn execute(&self, state: &mut State) {
        println!("xor {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) ^ state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_OR {
    rd: XprName,
    rs2: XprName,
}
impl C_OR {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_OR { rd, rs2 }
    }
}
impl Instruction for C_OR {
    fn execute(&self, state: &mut State) {
        println!("or {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) | state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_AND {
    rd: XprName,
    rs2: XprName,
}
impl C_AND {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_AND { rd, rs2 }
    }
}
impl Instruction for C_AND {
    fn execute(&self, state: &mut State) {
        println!("and {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) & state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_SUBW {
    rd: XprName,
    rs2: XprName,
}
impl C_SUBW {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_SUBW { rd, rs2 }
    }
}
impl Instruction for C_SUBW {
    fn execute(&self, state: &mut State) {
        println!("subw {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) - state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_ADDW {
    rd: XprName,
    rs2: XprName,
}
impl C_ADDW {
    pub fn new(inst: u16) -> Self {
        let (rs2, rd) = rvc_cs_type(inst);
        C_ADDW { rd, rs2 }
    }
}
impl Instruction for C_ADDW {
    fn execute(&self, state: &mut State) {
        println!("addw {:?}, {:?}, {:?}", self.rd, self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rd) + state.get_reg(self.rs2));
        state.pc += 2;
    }
}

pub struct C_J {
    offset: i64,
}
impl C_J {
    pub fn new(inst: u16) -> Self {
        let offset = rvc_cj_type(inst);
        C_J { offset }
    }
}
impl Instruction for C_J {
    fn execute(&self, state: &mut State) {
        println!("j {}", self.offset);
        state.pc += 2;
    }
}

pub struct C_BEQZ {
    rs1: XprName,
    offset: i64,
}
impl C_BEQZ {
    pub fn new(inst: u16) -> Self {
        let rs1 = rvc_cb_type(inst);
        let offset = (x(inst, 3, 1) << 5)
            + (x(inst, 4, 2) << 1)
            + (x(inst, 6, 2) << 6)
            + (x(inst, 10, 2) << 3)
            - (x(inst, 12, 1) << 8);
        C_BEQZ { rs1, offset }
    }
}
impl Instruction for C_BEQZ {
    fn execute(&self, state: &mut State) {
        println!("beqz {:?}, {}", self.rs1, self.offset);
        let rs1 = state.get_reg(self.rs1);
        if rs1 == 0 {
            state.pc += self.offset;
        } else {
            state.pc += 2;
        }
    }
}

pub struct C_BNEZ {
    rs1: XprName,
    offset: i64,
}
impl C_BNEZ {
    pub fn new(inst: u16) -> Self {
        let rs1 = rvc_cb_type(inst);
        let offset = (x(inst, 3, 1) << 5)
            + (x(inst, 4, 2) << 1)
            + (x(inst, 6, 2) << 6)
            + (x(inst, 10, 2) << 3)
            - (x(inst, 12, 1) << 8);
        C_BNEZ { rs1, offset }
    }
}
impl Instruction for C_BNEZ {
    fn execute(&self, state: &mut State) {
        println!("bnez {:?}, {}", self.rs1, self.offset);
        let rs1 = state.get_reg(self.rs1);
        if rs1 != 0 {
            state.pc += self.offset;
        } else {
            state.pc += 2;
        }
    }
}

pub struct C_SLLI(u16);
impl C_SLLI {
    pub fn new(inst: u16) -> Self {
        C_SLLI(inst)
    }
}
impl Instruction for C_SLLI {
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
        println!("ret");
        state.get_reg(XprName::ra);
        state.pc += 2;
    }
}

pub struct C_MV {
    rd: XprName,
    rs2: XprName,
}
impl C_MV {
    pub fn new(inst: u16) -> Self {
        let (rd, rs2) = rvc_cr_type(inst);
        C_MV { rd, rs2 }
    }
}
impl Instruction for C_MV {
    fn execute(&self, state: &mut State) {
        println!("mv {:?}, {:?}", self.rd, self.rs2);
        state.set_reg(self.rd, state.get_reg(self.rs2));
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
        state.pc += 2;
    }
}

pub struct C_ADD {
    rs1: XprName,
    rs2: XprName,
}
impl C_ADD {
    pub fn new(inst: u16) -> Self {
        let (rs1, rs2) = rvc_cr_type(inst);
        C_ADD { rs1, rs2 }
    }
}
impl Instruction for C_ADD {
    fn execute(&self, state: &mut State) {
        println!("add {:?}, {:?}, {:?}", self.rs1, self.rs1, self.rs2);
        let rs1 = state.get_reg(self.rs1);
        let rs2 = state.get_reg(self.rs2);
        state.set_reg(self.rs1, rs1 + rs2);
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
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
    fn execute(&self, state: &mut State) {
        println!("sd {:?}, {}(sp)", self.rs2, self.offset);
        let sp_val = state.regs.get(sp);
        state.regs.set(self.rs2, sp_val + self.offset);
        state.pc += 2;
    }
}
