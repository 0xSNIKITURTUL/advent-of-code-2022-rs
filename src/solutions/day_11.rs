// The credit for the second part of this solution goes to u/fasterthanlime on reddit.
use crate::solver::Solver;

fn round(monkeys: &mut Vec<Monkey>, product: usize) {
    let number_of_monkeys = monkeys.len();
    for i in 0..number_of_monkeys {
        let mut mc;
        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len();
        }
        mc.items.iter_mut().for_each(|item| {
            *item %= product;
            *item = mc.operation.calc(*item);
            if *item % mc.divisor == 0 {
                monkeys[mc.if_true].items.push(*item);
            } else {
                monkeys[mc.if_false].items.push(*item);
            }
        });
        monkeys[i].items.clear();
    }
}

pub struct Day11;

impl Solver<usize> for Day11 {
    fn part_one(input: impl Into<String>) -> usize {
        let input: String = input.into();
        let mut monkeys = input
            .trim()
            .split("\n\n")
            .map(|monkey| Monkey::from(monkey))
            .collect::<Vec<Monkey>>();
        let div_product = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .product::<usize>();
        (0..20).for_each(|_| round(&mut monkeys, div_product));
        let mut inspected = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .collect::<Vec<usize>>();
        inspected.sort_by_key(|&c| std::cmp::Reverse(c));
        inspected.iter().take(2).product()
    }

    fn part_two(input: impl Into<String>) -> usize {
        let input: String = input.into();
        let mut monkeys = input
            .trim()
            .split("\n\n")
            .map(|monkey| Monkey::from(monkey))
            .collect::<Vec<Monkey>>();
        let div_product = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .product::<usize>();
        (0..10000).for_each(|_| round(&mut monkeys, div_product));
        let mut inspected = monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .collect::<Vec<usize>>();
        inspected.sort_by_key(|&c| std::cmp::Reverse(c));
        inspected.iter().take(2).product()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day11;

    static INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_day_08_part_one() {
        test_solution::<usize, Day11>(INPUT, 10605, Part::One);
    }

    #[test]
    fn test_day_08_part_two() {
        test_solution::<usize, Day11>(INPUT, 2713310158, Part::Two);
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition(Operand, Operand),
    Multiplication(Operand, Operand),
}

impl Operation {
    pub fn calc(self, old: usize) -> usize {
        match self {
            Operation::Addition(left, right) => left.eval(old) + right.eval(old),
            Operation::Multiplication(left, right) => left.eval(old) * right.eval(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Constant(usize),
}

impl Operand {
    pub fn eval(self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Constant(constant) => constant,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    if_true: usize,
    if_false: usize,
    items_inspected: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        divisor: usize,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            divisor,
            if_true,
            if_false,
            items_inspected: 0,
        }
    }

    fn from(monkey_str: &str) -> Self {
        let stats = monkey_str.trim().lines().collect::<Vec<&str>>();
        let items = stats[1]
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|n| n.trim().parse::<usize>().ok())
            .collect();
        let operation = if stats[2].contains('*') {
            let opp = stats[2]
                .split('=')
                .nth(1)
                .unwrap()
                .split('*')
                .map(|o| match o.trim().parse::<usize>() {
                    Ok(x) => Operand::Constant(x),
                    _ => Operand::Old,
                })
                .collect::<Vec<Operand>>();
            Operation::Multiplication(opp[0], opp[1])
        } else {
            let opp = stats[2]
                .split('=')
                .nth(1)
                .unwrap()
                .split('+')
                .map(|o| match o.trim().parse::<usize>() {
                    Ok(x) => Operand::Constant(x),
                    _ => Operand::Old,
                })
                .collect::<Vec<Operand>>();
            Operation::Addition(opp[0], opp[1])
        };
        let divisor = stats[3]
            .split("by")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let if_true = stats[4]
            .split("monkey")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let if_false = stats[5]
            .split("monkey")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        Self::new(items, operation, divisor, if_true, if_false)
    }
}
