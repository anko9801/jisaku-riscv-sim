use crate::{
    error::{SimError, SimResult},
    instructions::{
        rv32i::{ADDI, AUIPC, JAL, JALR, LB, LUI},
        rvc::{
            C_ADD, C_ADDI, C_ADDI4SPN, C_ADDW, C_AND, C_EBREAK, C_FLD, C_FLDSP, C_FLWSP, C_FSD,
            C_FSDSP, C_FSWSP, C_JAL, C_JALR, C_JR, C_LD, C_LDSP, C_LI, C_LQSP, C_LW, C_LWSP, C_MV,
            C_NOP, C_OR, C_SD, C_SDSP, C_SLLI, C_SLLI64, C_SQSP, C_SUB, C_SUBW, C_SW, C_SWSP,
            C_XOR,
        },
        Instruction,
    },
    processor::{
        State,
        XLEN::{self, RV128, RV32, RV64},
    },
    utils::x,
};

#[derive(Debug)]
pub enum InstructionRaw {
    B16(u16),
    B32(u32),
}

impl InstructionRaw {
    pub fn get_inst(code: &[u8], pos: usize) -> Self {
        let length = Self::inst_length(u16::from_le_bytes(code[pos..pos + 2].try_into().unwrap()));
        assert!(length <= 4);
        match length {
            2 => InstructionRaw::B16(u16::from_le_bytes(
                code[pos..pos + length].try_into().unwrap(),
            )),
            4 => InstructionRaw::B32(u32::from_le_bytes(
                code[pos..pos + length].try_into().unwrap(),
            )),
            _ => panic!("test"),
        }
    }

    fn inst_length(base: u16) -> usize {
        if x(base, 0, 2) != 0b11 {
            2
        } else if x(base, 2, 3) != 0b111 {
            4
        } else if x(base, 5, 1) == 0 {
            6
        } else if x(base, 6, 1) == 0 {
            8
        } else if x(base, 12, 3) != 0b111 {
            10 + x(base, 12, 3) as usize * 2
        } else {
            26
        }
    }
}

impl State {
    pub fn decode_inst(&self, inst: InstructionRaw) -> SimResult<Box<dyn Instruction>> {
        match inst {
            InstructionRaw::B16(code) => self.decode_inst16(code),
            InstructionRaw::B32(code) => self.decode_inst32(code),
        }
    }

    fn decode_inst16(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let opcode = x(inst, 0, 2);
        match opcode {
            0b00 => self.decode_inst_c0(inst),
            0b01 => self.decode_inst_c1(inst),
            0b10 => self.decode_inst_c2(inst),
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst32(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        let opcode = x(inst, 2, 5);
        match opcode {
            0x00 => self.decode_inst_op_00000(inst),
            0x01 => self.decode_inst_op_00001(inst),
            0x02 => self.decode_inst_op_00010(inst),
            0x03 => self.decode_inst_op_00011(inst),
            0x04 => self.decode_inst_op_00100(inst),
            0x05 => Ok(Box::new(AUIPC::new(inst))),
            0x06 => self.decode_inst_op_00110(inst),
            0x08 => self.decode_inst_op_01000(inst),
            0x0c => self.decode_inst_op_01100(inst),
            0x0d => Ok(Box::new(LUI::new(inst))),
            0x18 => self.decode_inst_op_11000(inst),
            0x19 => Ok(Box::new(JALR::new(inst))),
            0x1b => Ok(Box::new(JAL::new(inst))),
            0x1c => self.decode_inst_op_11100(inst),
            _ => {
                panic!("");
            }
        }
    }

    fn decode_inst_c0(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let funct3 = x(inst, 13, 3);
        match funct3 {
            0b000 => {
                if inst == 0 {
                    Err(SimError::ParseError("Illegal instruction".to_string()))
                } else {
                    Ok(Box::new(C_ADDI4SPN::new(inst)))
                }
            }
            0b001 => Ok(Box::new(C_FLD::new(inst))),
            0b010 => Ok(Box::new(C_LW::new(inst))),
            0b011 => Ok(Box::new(C_LD::new(inst))),
            0b100 => Err(SimError::ParseError("Reserved".to_string())),
            0b101 => Ok(Box::new(C_FSD::new(inst))),
            0b110 => Ok(Box::new(C_SW::new(inst))),
            0b111 => Ok(Box::new(C_SD::new(inst))),
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst_c1(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let funct3 = x(inst, 13, 3);
        match funct3 {
            0b000 => {
                if inst == 1 {
                    Ok(Box::new(C_NOP::new(inst)))
                } else {
                    Ok(Box::new(C_ADDI::new(inst)))
                }
            }
            0b001 => match self.xlen {
                XLEN::RV32 => Ok(Box::new(C_JAL::new(inst))),
                _ => Ok(Box::new(C_JAL::new(inst))),
            },
            0b010 => Ok(Box::new(C_LI::new(inst))),
            0b100 => {
                let flag1 = x(inst, 5, 2);
                let flag2 = x(inst, 10, 2);
                let flag3 = x(inst, 12, 1);
                match (flag1, flag2, flag3) {
                    (0b00, 0b11, 0) => Ok(Box::new(C_SUB::new(inst))),
                    (0b01, 0b11, 0) => Ok(Box::new(C_XOR::new(inst))),
                    (0b10, 0b11, 0) => Ok(Box::new(C_OR::new(inst))),
                    (0b11, 0b11, 0) => Ok(Box::new(C_AND::new(inst))),
                    (0b00, 0b11, 1) => Ok(Box::new(C_SUBW::new(inst))),
                    (0b01, 0b11, 1) => Ok(Box::new(C_ADDW::new(inst))),
                    (0b10, 0b11, 1) => Err(SimError::ParseError("Reserved".to_string())),
                    (0b11, 0b11, 1) => Err(SimError::ParseError("Reserved".to_string())),
                    _ => panic!("unexpected branch"),
                }
            }
            _ => panic!("unexpected branch"),
        }
    }

    fn decode_inst_c2(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let flag1 = x(inst, 2, 5);
        let rd = x(inst, 7, 5);
        let flag2 = x(inst, 12, 1);
        let funct3 = x(inst, 13, 3);
        match funct3 {
            0b000 => match (flag1, rd, flag2) {
                (0, _, 0) => Ok(Box::new(C_SLLI64::new(inst))),
                (_, _, _) => Ok(Box::new(C_SLLI::new(inst))),
            },
            0b001 => match self.xlen {
                RV32 | RV64 => Ok(Box::new(C_FLDSP::new(inst))),
                RV128 => Ok(Box::new(C_LQSP::new(inst))),
            },
            0b010 => Ok(Box::new(C_LWSP::new(inst))),
            0b011 => match self.xlen {
                RV32 => Ok(Box::new(C_FLWSP::new(inst))),
                RV64 | RV128 => Ok(Box::new(C_LDSP::new(inst))),
            },
            0b100 => match (flag1, rd, flag2) {
                (0, _, 0) => Ok(Box::new(C_JR::new(inst))),
                (0, _, _) => Ok(Box::new(C_MV::new(inst))),
                (1, 0, 0) => Ok(Box::new(C_EBREAK::new(inst))),
                (1, _, 0) => Ok(Box::new(C_JALR::new(inst))),
                (1, _, _) => Ok(Box::new(C_ADD::new(inst))),
                _ => panic!("unexpected branch"),
            },
            0b101 => match self.xlen {
                RV32 | RV64 => Ok(Box::new(C_FSDSP::new(inst))),
                RV128 => Ok(Box::new(C_SQSP::new(inst))),
            },
            0b110 => Ok(Box::new(C_SWSP::new(inst))),
            0b111 => match self.xlen {
                RV32 => Ok(Box::new(C_FSWSP::new(inst))),
                RV64 | RV128 => Ok(Box::new(C_SDSP::new(inst))),
            },
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst_op_00000(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        let funct = x(inst, 12, 3);
        match funct {
            0b000 => Ok(Box::new(LB::new(inst))),
            _ => {
                panic!()
            }
        }
    }
    fn decode_inst_op_00001(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_00010(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_00011(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_00100(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        let funct = x(inst, 12, 3);
        match funct {
            0b000 => Ok(Box::new(ADDI::new(inst))),
            _ => {
                panic!()
            }
        }
    }
    fn decode_inst_op_00110(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_01000(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_01100(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_11000(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
    fn decode_inst_op_11100(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        panic!()
    }
}
