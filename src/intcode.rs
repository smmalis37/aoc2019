use crossbeam::channel::*;
use Mode::*;
use Opcode::*;

pub(crate) type IntCodeCell = i64;

#[derive(Clone)]
pub struct IntCode {
    memory: Memory,
    pc: IntCodeCell,
    relative_base: IntCodeCell,
}

impl std::str::FromStr for IntCode {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            memory: Memory::new(s.split(',').map(|l| l.parse()).collect::<Result<_, _>>()?),
            pc: 0,
            relative_base: 0,
        })
    }
}

impl IntCode {
    pub(crate) fn run_no_io(mut self, inputs: &[(IntCodeCell, IntCodeCell)]) -> Vec<IntCodeCell> {
        for &(input_location, input_value) in inputs {
            self.memory[input_location] = input_value;
        }
        let (_, input_recv) = bounded(0);
        let (output_send, _) = bounded(0);

        self.run(input_recv, output_send);
        self.memory.starting_memory
    }

    pub(crate) fn run_single_threaded<'a>(
        mut self,
        input: impl IntoIterator<Item = &'a IntCodeCell>,
    ) -> Vec<IntCodeCell> {
        let (input_send, input_recv) = unbounded();
        input.into_iter().for_each(|&x| input_send.send(x).unwrap());

        let (output_send, output_recv) = unbounded();

        self.run(input_recv, output_send);

        output_recv.into_iter().collect()
    }

    pub(crate) fn run_multi_threaded(
        mut self,
        input: Receiver<IntCodeCell>,
        output: Sender<IntCodeCell>,
    ) {
        self.run(input, output)
    }

    fn run(&mut self, input: Receiver<IntCodeCell>, output: Sender<IntCodeCell>) {
        loop {
            let instr = Instruction::new(self.memory[self.pc]);
            match instr.opcode {
                Add | Multiply | LessThan | Equals => self.do_math(instr),
                JumpIfTrue | JumpIfFalse => self.do_jump(instr),

                Input => {
                    *self.get_mut_memory(1, instr) = input.recv().unwrap();
                    self.pc += 2;
                }

                Output => {
                    output.send(self.get_parameter(1, instr)).unwrap();
                    self.pc += 2;
                }

                AdjustRelativeBase => {
                    self.relative_base += self.get_parameter(1, instr);
                    self.pc += 2;
                }

                Terminate => break,
            }
        }
    }

    fn do_math(&mut self, instr: Instruction) {
        let value1 = self.get_parameter(1, instr);
        let value2 = self.get_parameter(2, instr);

        let result = match instr.opcode {
            Add => value1 + value2,
            Multiply => value1 * value2,
            LessThan => (value1 < value2).into(),
            Equals => (value1 == value2).into(),
            _ => unreachable!(),
        };

        *self.get_mut_memory(3, instr) = result;
        self.pc += 4;
    }

    fn do_jump(&mut self, instr: Instruction) {
        let cond = self.get_parameter(1, instr);
        let new_pc = self.get_parameter(2, instr);

        if match instr.opcode {
            JumpIfTrue => cond != 0,
            JumpIfFalse => cond == 0,
            _ => unreachable!(),
        } {
            self.pc = new_pc;
        } else {
            self.pc += 3;
        }
    }

    fn get_parameter(&self, offset: usize, instr: Instruction) -> IntCodeCell {
        let index = self.pc + offset as IntCodeCell;

        match instr.modes[offset - 1] {
            Position => self.memory[self.memory[index]],
            Immediate => self.memory[index],
            Relative => self.memory[self.memory[index] + self.relative_base],
        }
    }

    fn get_mut_memory(&mut self, offset: usize, instr: Instruction) -> &mut IntCodeCell {
        let index = self.pc + offset as IntCodeCell;

        match instr.modes[offset - 1] {
            Position => {
                let first = self.memory[index];
                &mut self.memory[first]
            }
            Immediate => unreachable!(),
            Relative => {
                let first = self.memory[index] + self.relative_base;
                &mut self.memory[first]
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    modes: [Mode; 3],
}

impl Instruction {
    fn new(val: IntCodeCell) -> Self {
        Self {
            opcode: Opcode::new(val % 100),
            modes: [
                Mode::new(val / 100 % 10),
                Mode::new(val / 1000 % 10),
                Mode::new(val / 10000 % 10),
            ],
        }
    }
}

#[derive(Copy, Clone)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Terminate,
}

impl Opcode {
    fn new(val: IntCodeCell) -> Self {
        match val {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => AdjustRelativeBase,
            99 => Terminate,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn new(val: IntCodeCell) -> Self {
        match val {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Memory {
    starting_memory: Vec<IntCodeCell>,
    extra_memory: Vec<IntCodeCell>,
}

impl Memory {
    fn new(starting_memory: Vec<IntCodeCell>) -> Self {
        Self {
            starting_memory,
            extra_memory: Vec::new(),
        }
    }
}

impl std::ops::Index<IntCodeCell> for Memory {
    type Output = IntCodeCell;

    fn index(&self, index: IntCodeCell) -> &Self::Output {
        let index = index as usize;

        if index < self.starting_memory.len() {
            &self.starting_memory[index]
        } else if index - self.starting_memory.len() < self.extra_memory.len() {
            &self.extra_memory[index - self.starting_memory.len()]
        } else {
            &0
        }
    }
}

impl std::ops::IndexMut<IntCodeCell> for Memory {
    fn index_mut(&mut self, index: IntCodeCell) -> &mut Self::Output {
        let index = index as usize;

        if index < self.starting_memory.len() {
            &mut self.starting_memory[index]
        } else {
            let index = index - self.starting_memory.len();

            if index >= self.extra_memory.len() {
                self.extra_memory.resize(index + 1, 0);
            }

            &mut self.extra_memory[index]
        }
    }
}
