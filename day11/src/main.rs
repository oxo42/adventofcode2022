use std::{cell::RefCell, collections::VecDeque, fmt::Display};

use itertools::Itertools;

mod parser;

#[derive(Debug)]
#[allow(dead_code)]
struct Monkey {
    index: u32,
    items: VecDeque<i64>,
    operation: Operation,
    test_divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

impl Monkey {
    fn receive(&mut self, value: i64) {
        self.items.push_back(value);
    }
}

#[derive(Debug)]
enum Operand {
    Num(i64),
    Old,
}

impl Operand {
    fn num(&self, old: i64) -> i64 {
        match self {
            Operand::Num(n) => *n,
            Operand::Old => old,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    // Subtract(Operand),
    Multiply(Operand),
    // Divide(Operand),
}

impl Operation {
    fn from(value: char, operand: Operand) -> Self {
        use Operation::*;
        match value {
            '*' => Multiply(operand),
            '+' => Add(operand),
            // '/' => Divide(operand),
            // '-' => Subtract(operand),
            _ => panic!("invalid op"),
        }
    }

    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(op) => old + op.num(old),
            Operation::Multiply(op) => old * op.num(old),
        }
    }
}

#[derive(Debug)]
struct BusinessMachine {
    monkeys: Vec<RefCell<Monkey>>,
    divisor_product: i64,
}

impl Display for BusinessMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for monkey in &self.monkeys {
            let monkey = monkey.borrow();
            writeln!(
                f,
                "Monkey {}: {:?} ({})",
                monkey.index, monkey.items, monkey.inspections
            )?;
        }
        Ok(())
    }
}

impl BusinessMachine {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let divisor_product = monkeys.iter().map(|m| m.test_divisor).product();
        let monkeys = monkeys.into_iter().map(RefCell::new).collect_vec();
        Self {
            monkeys,
            divisor_product,
        }
    }
    fn round(&mut self) {
        for monkey_cell in &self.monkeys {
            let mut monkey = monkey_cell.borrow_mut();
            loop {
                // inspect
                let mut item = match monkey.items.pop_front() {
                    Some(x) => x,
                    None => break,
                };
                monkey.inspections += 1;
                item %= self.divisor_product;
                // worry up
                item = monkey.operation.apply(item);
                // test
                let dest_monkey = if item % monkey.test_divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                // throw
                self.monkeys[dest_monkey].borrow_mut().receive(item);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|m| m.borrow().inspections)
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .map(|(a, b)| a * b)
            .unwrap()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    better_panic::install();
    // let monkeys = parser::parse_monkeys(include_str!("sample.txt"))?;
    let monkeys = parser::parse_monkeys(include_str!("input.txt"))?;
    for m in &monkeys {
        println!("{:?}", m);
    }
    let mut machine = BusinessMachine::new(monkeys);
    println!("{machine}");
    for _ in 0..10_000 {
        machine.round();
    }
    println!("{machine}");

    println!("Monkey Business: {}", machine.monkey_business());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_input() -> color_eyre::Result<()> {
        let monkeys = parser::parse_monkeys(include_str!("sample.txt"))?;
        let mut machine = BusinessMachine::new(monkeys);
        for _ in 0..10_000 {
            machine.round();
        }
        assert_eq!(2713310158, machine.monkey_business());
        Ok(())
    }
}
