use Mode::*;
use Opcode::*;

pub fn parse_intcode(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn run_intcode(memory: &mut [isize]) -> &[isize] {
    let mut index = 0;

    loop {
        let instr = Instruction::new(memory[index]);
        match instr.opcode {
            Add | Multiply => do_math(memory, &mut index, instr),
            Input => unreachable!(),
            Output => unreachable!(),
            Terminate => break,
        }
    }

    memory
}

fn do_math(memory: &mut [isize], index: &mut usize, instr: Instruction) {
    assert!(instr.modes[2] == Position);
    let value1 = get_parameter(memory, *index + 1, instr.modes[0]);
    let value2 = get_parameter(memory, *index + 2, instr.modes[1]);
    let result = match instr.opcode {
        Add => value1 + value2,
        Multiply => value1 * value2,
        _ => unreachable!(),
    };
    *get_mut_memory(memory, *index + 3) = result;
    *index += 4;
}

fn get_parameter(memory: &[isize], index: usize, mode: Mode) -> isize {
    match mode {
        Mode::Position => memory[memory[index] as usize],
        Mode::Immediate => memory[index],
    }
}

fn get_mut_memory(memory: &mut [isize], index: usize) -> &mut isize {
    &mut memory[memory[index] as usize]
}

#[derive(Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    modes: [Mode; 3],
}

impl Instruction {
    fn new(val: isize) -> Self {
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
    Terminate,
}

impl Opcode {
    fn new(val: isize) -> Self {
        match val {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            99 => Terminate,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn new(val: isize) -> Self {
        match val {
            0 => Position,
            1 => Immediate,
            _ => unreachable!(),
        }
    }
}
