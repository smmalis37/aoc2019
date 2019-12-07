use Mode::*;
use Opcode::*;

pub fn parse_intcode(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn run_intcode(memory: &mut [isize], input: impl IntoIterator<Item = isize>) -> Vec<isize> {
    let mut pc = 0;
    let mut outputs = vec![];
    let mut input = input.into_iter();

    loop {
        let instr = Instruction::new(memory[pc]);
        match instr.opcode {
            Add | Multiply | LessThan | Equals => do_math(memory, &mut pc, instr),
            Input => {
                assert!(instr.modes[0] == Position);
                *get_mut_memory(memory, pc + 1) = input.next().unwrap();
                pc += 2;
            }
            Output => {
                outputs.push(get_parameter(memory, pc + 1, instr.modes[0]));
                pc += 2;
            }
            JumpIfTrue | JumpIfFalse => do_jump(memory, &mut pc, instr),
            Terminate => break,
        }
    }

    outputs
}

fn do_math(memory: &mut [isize], pc: &mut usize, instr: Instruction) {
    assert!(instr.modes[2] == Position);
    let value1 = get_parameter(memory, *pc + 1, instr.modes[0]);
    let value2 = get_parameter(memory, *pc + 2, instr.modes[1]);
    let result = match instr.opcode {
        Add => value1 + value2,
        Multiply => value1 * value2,
        LessThan => (value1 < value2).into(),
        Equals => (value1 == value2).into(),
        _ => unreachable!(),
    };
    *get_mut_memory(memory, *pc + 3) = result;
    *pc += 4;
}

fn do_jump(memory: &mut [isize], pc: &mut usize, instr: Instruction) {
    let cond = get_parameter(memory, *pc + 1, instr.modes[0]);
    let new_pc = get_parameter(memory, *pc + 2, instr.modes[1]);
    if match instr.opcode {
        JumpIfTrue => cond != 0,
        JumpIfFalse => cond == 0,
        _ => unreachable!(),
    } {
        *pc = new_pc as usize;
    } else {
        *pc += 3;
    }
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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
}

impl Opcode {
    fn new(val: isize) -> Self {
        match val {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
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
