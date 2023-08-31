mod decode;
mod error;
pub mod instructions;
mod processor;
mod utils;

use processor::State;

use crate::decode::InstructionRaw;

fn main() {
    let path = std::path::PathBuf::from("sample-objects/symver.x86_64.so");
    let insts = vec![
        0x41, 0x11, // addi sp,sp,-16
        0x06, 0xe4, // sd ra,8(sp)
        0xef, 0x00, 0x60, 0x12, // jal ra,101da <build_payload>
        0x13, 0x85, 0x81, 0x76, // addi a0,gp,1896 # 14760 <payload>
        0x93, 0x05, 0x40, 0x06, // li a1,100
        0xef, 0x00, 0x60, 0x0e, // jal ra,101a6 <copy_and_print>
        0xa2, 0x60, // ld ra,8(sp)
        0x01, 0x45, // li a0,0
        0x41, 0x01, // addi sp,sp,16
        0x82, 0x80, // ret
    ];
    let mut state = State::new();
    while state.pc < insts.len() as i64 {
        let inst = InstructionRaw::get_inst(&insts, state.pc as usize);
        let test = state.decode_inst(inst);
        match test {
            Ok(a) => a.effect(&mut state),
            Err(e) => println!("{:?}", e),
        }
    }
}
