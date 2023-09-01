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
        state.set_reg(self.rd, state.pc + 4);
        state.pc += self.imm;
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

pub struct BEQ {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BEQ {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BEQ { rs1, rs2, offset }
    }
}
impl Instruction for BEQ {
    fn execute(&self, state: &mut State) {
        println!("beq {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if state.get_reg(self.rs1) == state.get_reg(self.rs2) {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
    }
}

pub struct BNE {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BNE {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BNE { rs1, rs2, offset }
    }
}
impl Instruction for BNE {
    fn execute(&self, state: &mut State) {
        println!("bne {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if state.get_reg(self.rs1) != state.get_reg(self.rs2) {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
    }
}

pub struct BLT {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BLT {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BLT { rs1, rs2, offset }
    }
}
impl Instruction for BLT {
    fn execute(&self, state: &mut State) {
        println!("blt {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if state.get_reg(self.rs1) < state.get_reg(self.rs2) {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
    }
}

pub struct BGE {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BGE {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BGE { rs1, rs2, offset }
    }
}
impl Instruction for BGE {
    fn execute(&self, state: &mut State) {
        println!("bge {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if state.get_reg(self.rs1) >= state.get_reg(self.rs2) {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
    }
}

pub struct BLTU {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BLTU {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BLTU { rs1, rs2, offset }
    }
}
impl Instruction for BLTU {
    fn execute(&self, state: &mut State) {
        println!("bltu {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if (state.get_reg(self.rs1) as u64) < state.get_reg(self.rs2) as u64 {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
    }
}

pub struct BGEU {
    rs1: XprName,
    rs2: XprName,
    offset: i64,
}
impl BGEU {
    pub fn new(inst: u32) -> Self {
        let (rs1, rs2, offset) = rv32i_b_type(inst);
        BGEU { rs1, rs2, offset }
    }
}
impl Instruction for BGEU {
    fn execute(&self, state: &mut State) {
        println!("bgeu {:?}, {:?}, {}", self.rs1, self.rs2, self.offset);
        if state.get_reg(self.rs1) as u64 >= state.get_reg(self.rs2) as u64 {
            state.pc += self.offset;
        } else {
            state.pc += 4;
        }
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

pub struct SLTI {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl SLTI {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        SLTI { rd, rs1, imm }
    }
}
impl Instruction for SLTI {
    fn execute(&self, state: &mut State) {
        println!("slti {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        let rs1 = state.get_reg(self.rs1);
        let value = if rs1 < self.imm { 1 } else { 0 };
        state.set_reg(self.rd, value);
        state.pc += 4;
    }
}

pub struct SLTIU {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl SLTIU {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        SLTIU { rd, rs1, imm }
    }
}
impl Instruction for SLTIU {
    fn execute(&self, state: &mut State) {
        println!("sltiu {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        let rs1 = state.get_reg(self.rs1);
        let value = if (rs1 as u64) < self.imm as u64 { 1 } else { 0 };
        state.set_reg(self.rd, value);
        state.pc += 4;
    }
}

pub struct XORI {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl XORI {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        XORI { rd, rs1, imm }
    }
}
impl Instruction for XORI {
    fn execute(&self, state: &mut State) {
        println!("xori {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        state.set_reg(self.rd, state.get_reg(self.rs1) ^ self.imm);
        state.pc += 4;
    }
}

pub struct ORI {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl ORI {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        ORI { rd, rs1, imm }
    }
}
impl Instruction for ORI {
    fn execute(&self, state: &mut State) {
        println!("ori {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        state.set_reg(self.rd, state.get_reg(self.rs1) | self.imm);
        state.pc += 4;
    }
}

pub struct ANDI {
    rd: XprName,
    rs1: XprName,
    imm: i64,
}
impl ANDI {
    pub fn new(inst: u32) -> Self {
        let (rd, rs1, imm) = rv32i_i_type(inst);
        ANDI { rd, rs1, imm }
    }
}
impl Instruction for ANDI {
    fn execute(&self, state: &mut State) {
        println!("andi {:?}, {:?}, {}", self.rd, self.rs1, self.imm);
        state.set_reg(self.rd, state.get_reg(self.rs1) & self.imm);
        state.pc += 4;
    }
}
