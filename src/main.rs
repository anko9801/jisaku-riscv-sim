mod decode;
mod error;
pub mod instructions;
mod processor;

use processor::{State, XprName::*};

use crate::decode::InstructionRaw;

fn main() {
    let insts = vec![0x80, 0x82];
    let mut state = State::new();
    state.regs.set(a0, 10);
    println!("{}", state.regs.get(a0));

    let inst = InstructionRaw::get_inst(&insts, state.pc as usize);
    let test = state.decode_inst(inst);
    match test {
        Ok(a) => a.effect(&mut state),
        Err(e) => println!("{:?}", e),
    }

    println!("Hello, world!");
}
