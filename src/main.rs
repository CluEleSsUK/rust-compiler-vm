mod stack;
mod vm;

use vm::{VirtualMachine, Instruction};

fn main() {
    let mut vm = VirtualMachine::new();
    let instructions = vec!(Instruction::NOOP);
    let output = vm.run(instructions);
}
