use std::default::Default;
use std::str::FromStr;

pub fn solve(input: &str) -> String {
    let input = input.trim();
    let p1 = part1(input);
    let p2 = part2(input);
    format!("{p1}, {p2}")
}

fn part1(input: &str) -> isize {
    let instructions = parse_instructions(input).unwrap();
    let mut state = CpuState::default();
    state.ticks = 1;
    let mut sum = 0;

    for tick in [20, 60, 100, 140, 180, 220] {
        state.fast_forward(&instructions, tick);
        sum += state.x as isize * tick as isize;
    }

    sum
}

fn part2(input: &str) -> String {
    let instructions = parse_instructions(input).unwrap();
    let mut cpu = CpuState::default();
    let mut crt = CrtState::default();

    let mut display = String::new();

    for tick in 0.. {
        cpu.fast_forward(&instructions, tick);
        if tick % 40 == 0 {
            display.push('\n');
        }
        display.push(crt.draw(&cpu));
        if instructions.len() == cpu.pc {
            break;
        }
    }

    display
}

#[derive(Debug, Clone, Copy)]
struct CpuState {
    ticks: usize,
    pc: usize,
    x: i16,
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            ticks: 0,
            x: 1,
            pc: 0,
        }
    }
}

impl CpuState {
    fn exec(&mut self, instructions: &[Instruction]) {
        let instruction = &instructions[self.pc];
        match instruction {
            Instruction::Nop => {}
            Instruction::AddX { immediate } => {
                self.x += *immediate;
            }
        }
        self.ticks += instruction.ticks();
        self.pc += 1;
    }

    fn exec_backward(&mut self, instructions: &[Instruction]) {
        self.pc -= 1;
        let instruction = &instructions[self.pc];
        self.ticks -= instruction.ticks();
        match instruction {
            Instruction::Nop => {}
            Instruction::AddX { immediate } => {
                self.x -= *immediate;
            }
        }
    }

    // step to the tick specified, without going over
    fn fast_forward(&mut self, instructions: &[Instruction], tick: usize) {
        loop {
            self.exec(&instructions);
            if self.ticks == tick {
                // we stop exact
                break;
            }
            if self.ticks > tick {
                // we went one too far
                self.exec_backward(&instructions);
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop,
    AddX { immediate: i16 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Nop)
        } else if let Some((_, val)) = s.split_once("addx ") {
            let immediate = val.parse().map_err(drop)?;
            Ok(Self::AddX { immediate })
        } else {
            Err(())
        }
    }
}

impl Instruction {
    const fn ticks(&self) -> usize {
        match self {
            Instruction::Nop => 1,
            Instruction::AddX { .. } => 2,
        }
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, ()> {
    input.lines().map(|l| l.parse::<Instruction>()).collect()
}

#[derive(Debug, Clone, Copy)]
struct CrtState {
    ticks: usize,
}

impl Default for CrtState {
    fn default() -> Self {
        Self { ticks: 1 }
    }
}

impl CrtState {
    fn draw(&mut self, cpu: &CpuState) -> char {
        let c = if (self.column() - 2..=self.column()).contains(&cpu.x) {
            '#'
        } else {
            '.'
        };
        self.ticks += 1;
        c
    }

    fn column(&self) -> i16 {
        (self.ticks % 40) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(13140, part1(TEST_INPUT));
    }
    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
