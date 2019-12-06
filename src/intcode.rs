pub fn parse_intcode(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

pub fn run_intcode(memory: &mut [usize]) -> &[usize] {
    let mut index = 0;

    loop {
        match memory[index] {
            1 => {
                memory[memory[index + 3]] = memory[memory[index + 1]] + memory[memory[index + 2]];
            }
            2 => {
                memory[memory[index + 3]] = memory[memory[index + 1]] * memory[memory[index + 2]];
            }
            99 => break,
            _ => unreachable!(),
        }
        index += 4;
    }

    memory
}
