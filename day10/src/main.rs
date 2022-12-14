use std::fmt::Display;

mod parser;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn cycle_cost(&self) -> usize {
        use Instruction::*;
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Machine {
    clock: usize,
    reg_x: i64,
    instructions: Vec<Instruction>,
    current_instruction: usize,
    pipeline_length: usize,
    signals: Vec<i64>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Machine[ clock: {}, reg_x: {}, ins: {}, op: {:?} ]",
            self.clock,
            self.reg_x,
            self.current_instruction,
            self.instructions.get(self.current_instruction)
        )
    }
}

impl Machine {
    fn new(reg_x: i64, instructions: Vec<Instruction>) -> Self {
        let cycle_cost = instructions[0].cycle_cost();
        Self {
            clock: 0,
            reg_x,
            instructions,
            current_instruction: 0,
            pipeline_length: cycle_cost,
            signals: Vec::new(),
        }
    }

    fn write(&self) {
        let x = (self.clock % 40) as i64;
        let c = if x.abs_diff(self.reg_x) < 2 { "#" } else { "." };
        print!("{}", c);
        if self.clock % 40 == 0 {
            println!();
        }
    }

    fn tick(&mut self) {
        use Instruction::*;
        self.clock += 1;
        if self.is_finished() {
            return;
        }

        self.pipeline_length -= 1;
        if (self.clock > 20 && (self.clock - 20) % 40 == 0) || self.clock == 20 {
            self.signals.push(self.reg_x * self.clock as i64);
        }
        // op still processing
        if self.pipeline_length > 0 {
            return;
        }
        // Handle op
        match self.instructions[self.current_instruction] {
            Noop => {}
            AddX(operand) => self.reg_x += operand,
        }

        // Load up new op
        self.current_instruction += 1;
        if !self.is_finished() {
            self.pipeline_length = self.instructions[self.current_instruction].cycle_cost();
        }
    }

    fn is_finished(&self) -> bool {
        self.current_instruction >= self.instructions.len()
    }

    fn signal_sum(&self) -> i64 {
        self.signals.iter().sum()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let instructions = parser::parse_instructions(INPUT);
    let instructions = parser::parse_instructions(include_str!("input.txt"));
    let mut machine = Machine::new(1, instructions);
    while !machine.is_finished() {
        machine.tick();
        machine.write();
        // if machine.clock % 20 == 0 {
        //     println!("Clock: {}, signal: {:?}", machine.clock, machine.signals);
        // }
    }

    println!("Signal sum: {}", machine.signal_sum());

    Ok(())
}

#[allow(dead_code)]
const INPUT: &str = "addx 15
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
noop
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_machine() {
        use Instruction::*;
        let instructions = vec![Noop, AddX(3), AddX(-5)];
        let mut machine = Machine::new(1, instructions);
        machine.tick();
        assert_eq!(1, machine.reg_x);
        machine.tick();
        assert_eq!(1, machine.reg_x);
        machine.tick();
        assert_eq!(4, machine.reg_x);
        machine.tick();
        assert_eq!(4, machine.reg_x);
        machine.tick();
        assert_eq!(-1, machine.reg_x);
    }

    #[test]
    fn test_full() {
        let instructions = parser::parse_instructions(INPUT);
        let mut machine = Machine::new(1, instructions);
        while !machine.is_finished() {
            machine.tick();
        }
        assert_eq!(13140, machine.signal_sum());
    }
}
