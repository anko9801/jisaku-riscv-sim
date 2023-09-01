mod decode;
mod error;
pub mod instructions;
mod mmu;
mod processor;
mod utils;

use processor::State;

use crate::decode::InstructionRaw;

fn main() {
    let mut state = State::new();
    state.read_elf();
    loop {
        state.print_regs();
        print!("{:#x}:\t", state.pc);
        let inst = state.get_inst();
        match inst {
            Ok(inst) => inst.execute(&mut state),
            Err(e) => panic!("{}", e),
        }
    }
}

/*
テスト用
   let insts = vec![
       0x41, 0x11, // addi sp,sp,-16
       0x06, 0xe4, // sd ra,8(sp)
       0xef, 0x00, 0x60, 0x12, // jal ra,101da
       0x13, 0x85, 0x81, 0x76, // addi a0,gp,1896 # 14760
       0x93, 0x05, 0x40, 0x06, // li a1,100
       0xef, 0x00, 0x60, 0x0e, // jal ra,101a6
       0xa2, 0x60, // ld ra,8(sp)
       0x01, 0x45, // li a0,0
       0x41, 0x01, // addi sp,sp,16
       0x82, 0x80, // ret
   ];
*/
