use crate::{
    error::{SimError, SimResult},
    instructions::{
        rv32i::{ADDI, AUIPC, JAL, JALR, LB, LUI},
        rvc::{C_ADDI, C_JR, C_NOP},
        Instruction,
    },
    processor::State,
};
// xlen -> opcode -> mask
// Vec<u8> -> [pc -> xlen -> u16, u32 Instruction_raw -> match -> Instruction -> effect]

pub enum InstructionRaw {
    B16(u16),
    B32(u32),
}

impl InstructionRaw {
    pub fn get_inst(code: &[u8], pos: usize) -> Self {
        let length = Self::inst_length(u16::from_be_bytes(code[pos..pos + 2].try_into().unwrap()));
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
        if base & 0b11 != 0b11 {
            2
        } else if base & 0b11111 != 0b11111 {
            4
        } else if base & 0b111111 == 0b011111 {
            6
        } else if base & 0b1111111 == 0b0111111 {
            8
        } else if (base >> 12) & 0b111 != 0b111 {
            10 + (((base >> 12) & 0b111) as usize) * 2
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
        let opcode = inst & 0b11;
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
        let opcode = (inst >> 2) & 0b11111;
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
        let func = (inst >> 13) & 0b111;
        match func {
            0b000 => {
                if inst == 0 {
                    Err(SimError::ParseError("Illegal instruction".to_string()))
                } else {
                    todo!()
                    // Ok(Box::new(C_ADDI4SPN(inst)))
                }
            }
            // 0b001 => Ok(Box::new(C_FLD(inst))),
            // 0b010 => Ok(Box::new(C_LW(inst))),
            // 0b011 => Ok(Box::new(C_LD(inst))),
            // 0b100 => Err(SimError::ParseError("Reserved".to_string())),
            // 0b101 => Ok(Box::new(C_FSD(inst))),
            // 0b110 => Ok(Box::new(C_SW(inst))),
            // 0b111 => Ok(Box::new(C_SD(inst))),
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst_c1(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let func = (inst >> 13) & 0b111;
        match func {
            0b000 => {
                if inst == 1 {
                    Ok(Box::new(C_NOP::new(inst)))
                } else {
                    Ok(Box::new(C_ADDI::new(inst)))
                }
            }
            // 0b001 => Ok(Box::new(C_ADDIW(inst))),
            // 0b010 => Ok(Box::new(C_LI(inst))),
            // 0b011 => Ok(Box::new(C_LUI(inst))),
            // 0b100 => Ok(Box::new(C_LUI(inst))),
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst_c2(&self, inst: u16) -> SimResult<Box<dyn Instruction>> {
        let func = (inst >> 13) & 0b111;
        match func {
            0b100 => {
                if (inst >> 12) & 1 == 0 && (inst >> 2) & 0x1f == 0 {
                    Ok(Box::new(C_JR::new(inst)))
                } else {
                    todo!();
                }
            }
            // 0b000 => {
            //     if inst == 1 {
            //         Ok(Box::new(C_NOP(inst)))
            //     } else {
            //         Ok(Box::new(C_ADDI(inst)))
            //     }
            // }
            // 0b001 => Ok(Box::new(C_ADDIW(inst))),
            // 0b010 => Ok(Box::new(C_LI(inst))),
            // 0b011 => Ok(Box::new(C_LUI(inst))),
            // 0b100 => Ok(Box::new(C_LUI(inst))),
            _ => {
                panic!("unexpected branch");
            }
        }
    }

    fn decode_inst_op_00000(&self, inst: u32) -> SimResult<Box<dyn Instruction>> {
        let funct = (inst >> 12) & 0b111;
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
        let funct = (inst >> 12) & 0b111;
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
