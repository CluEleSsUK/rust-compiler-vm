use stack::*;

mod stack;

fn main() {}

type AddressSize = i32;

enum Instruction {
    NOOP,
    PUSH(AddressSize),
    POP,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Object {
    INT(i32),
    STRING(*mut str),
}

struct VirtualMachine<'a> {
    stack: Box<dyn Stack<'a, Object>>,
    accumulator: Option<Box<Object>>,
}

impl<'a> VirtualMachine<'a> {
    fn new() -> VirtualMachine<'a> {
        return VirtualMachine {
            stack: Box::from(VectorStack::new()),
            accumulator: None,
        };
    }

    fn run(&mut self, mut instructions: Vec<Instruction>) -> Option<Box<Object>> {
        let instruction = instructions.first();

        return match instruction {
            None => self.accumulator.clone(),
            Some(it) => {
                self.execute(it);
                instructions.remove(0);
                self.run(instructions)
            }
        };
    }

    fn execute(&mut self, instruction: &Instruction) {
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

        let output = vm.run(instructions);

        match output {
            Some(expr) => assert_eq!(*expr, Object::INT(10)),
            None => panic!("test failed"),
        }
    }

    #[test]
    fn pop_nothing_returns_empty() {
        let mut vm = VirtualMachine::new();
        let mut instructions = vec![Instruction::POP];

        match vm.run(instructions) {
            Some(x) => panic!("Empty pop should return None!"),
            None => {}
        }
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

        match vm.run(instructions) {
            Some(expr) => assert_eq!(*expr, Object::INT(5)),
            None => panic!("test failed"),
        }
    }
}
