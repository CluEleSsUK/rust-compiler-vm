use crate::stack::{Stack, VectorStack};

pub type AddressSize = i32;

pub enum Instruction {
    NOOP,
    PUSH(AddressSize),
    POP,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Object {
    INT(i32),
    STRING(*mut str),
    NULL,
}

pub struct VirtualMachine<'a> {
    stack: Box<dyn Stack<'a, Object>>,
    accumulator: Option<Box<Object>>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new() -> VirtualMachine<'a> {
        return VirtualMachine {
            stack: Box::from(VectorStack::new()),
            accumulator: None,
        };
    }

    pub fn run(&mut self, mut instructions: Vec<Instruction>) -> Box<Object> {
        let instruction = instructions.first();

        let output = match instruction {
            None => self.accumulator.clone(),
            Some(it) => {
                self.execute(it);
                instructions.remove(0);
                Some(self.run(instructions))
            }
        };

        return output.unwrap_or(Box::from(Object::NULL));
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::NOOP => {}
            Instruction::PUSH(operand) => {
                self.stack.push(Box::new(Object::INT(operand.clone())));
            }

            Instruction::POP => {
                self.accumulator = self.stack.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop_work() {
        let mut vm = VirtualMachine::new();
        let mut instructions = vec![
            Instruction::PUSH(5),
            Instruction::PUSH(10),
            Instruction::POP
        ];

        assert_eq!(*vm.run(instructions), Object::INT(10));
    }

    #[test]
    fn pop_nothing_returns_null() {
        let mut vm = VirtualMachine::new();
        let mut instructions = vec![Instruction::POP];

        assert_eq!(*vm.run(instructions), Object::NULL);
    }

    #[test]
    fn noop_does_nothing() {
        let mut vm = VirtualMachine::new();
        let mut instructions = vec![
            Instruction::NOOP,
            Instruction::PUSH(5),
            Instruction::NOOP,
            Instruction::NOOP,
            Instruction::POP,
            Instruction::NOOP,
        ];
        assert_eq!(*vm.run(instructions), Object::INT(5))
    }
}
