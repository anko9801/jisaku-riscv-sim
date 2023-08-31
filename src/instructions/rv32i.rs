use crate::processor::State;

use super::Instruction;

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
