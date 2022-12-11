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
    let mut device = Device::with_instructions(instructions);
    let mut sum = 0;

    // I think this has to start at one because the ticks are talked about as
    // if they were ordinals, not starting at 0.
    let mut tick = 1;
    for tick_num in [20, 60, 100, 140, 180, 220] {
        while tick != tick_num {
            device.tick();
            tick += 1;
        }
        sum += device.cpu.x as isize * tick as isize;
    }

    sum
}

fn part2(input: &str) -> String {
    let instructions = parse_instructions(input).unwrap();
    let mut device = Device::with_instructions(instructions);
    device.run_until_done();

    device.crt.buffer
}

#[derive(Default)]
struct Device {
    cpu: CpuState,
    crt: CrtState,
}

impl Device {
    fn tick(&mut self) {
        self.crt.tick(&self.cpu);
        self.cpu.tick();
    }

    fn with_instructions(instructions: Vec<Instruction>) -> Self {
        let mut device = Self::default();
        device.cpu = CpuState::with_instructions(instructions);
        device
    }

    fn run_until_done(&mut self) {
        while self.cpu.instructions.len() > self.cpu.pc {
            self.tick();
        }
    }
}

#[derive(Debug, Clone)]
struct CpuState {
    // ticks since last complete instruction
    ticks: usize,
    // program counter
    pc: usize,
    // X register
    x: i16,
    // instructions
    instructions: Vec<Instruction>,
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            ticks: 0,
            x: 1,
            pc: 0,
            instructions: Vec::new(),
        }
    }
}

impl CpuState {
    fn tick(&mut self) {
        let instruction = &self.instructions[self.pc];
        self.ticks += 1;

        if instruction.ticks() > self.ticks {
            return;
        }

        // enough ticks have passed, update the cpu state

        match instruction {
            Instruction::Nop => {}
            Instruction::AddX { immediate } => {
                self.x += *immediate;
            }
        }

        self.pc += 1;
        self.ticks = 0;
    }

    fn with_instructions(instructions: Vec<Instruction>) -> Self {
        let mut s = Self::default();
        s.instructions = instructions;
        s
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

#[derive(Debug, Default, Clone)]
struct CrtState {
    buffer: String,
}

impl CrtState {
    fn tick(&mut self, cpu: &CpuState) {
        if self.buffer.len() % 41 == 0 {
            self.buffer.push('\n');
        }

        self.buffer
            .push(if (self.column() - 2..=self.column()).contains(&cpu.x) {
                '#'
            } else {
                ' ' // spec says to use '.' here but it's less readable
            });
    }

    fn column(&self) -> i16 {
        (self.buffer.len() % 41) as i16
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
