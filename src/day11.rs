framework::day!(11, parse => pt1, pt2);
use num::Integer;
use std::mem::swap;

type Int = u64;

fn pts<const ROUNDS: usize, const WORRY_DIVISOR: Int>(monkeys: &[Monkey]) -> MulOutput<[usize; 2]> {
    let mut items = monkeys
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect_vec();
    let mut inspect_count = vec![0; monkeys.len()];

    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.denominator)
        .fold(1, |a, d| a.lcm(&d));

    let mut temp_items = Vec::new();
    for _ in 0..ROUNDS {
        for (i, monkey) in monkeys.iter().enumerate() {
            debug_assert!(temp_items.is_empty());
            swap(&mut temp_items, &mut items[i]);
            inspect_count[i] += temp_items.len();

            for item in temp_items.drain(..) {
                let (a, o, b) = monkey.operation;
                let (a, b) = (a.evaluate(item), b.evaluate(item));
                let worry_level = match o {
                    Operation::Add => a + b,
                    Operation::Multiply => a * b,
                } / WORRY_DIVISOR
                    % modulo;
                let target_monkey = if worry_level % monkey.denominator == 0 {
                    monkey.if_divisible
                } else {
                    monkey.if_not_divisible
                };
                items[target_monkey].push(worry_level);
            }
        }
    }

    inspect_count.sort_by(|a, b| b.cmp(a));
    MulOutput([inspect_count[1], inspect_count[0]])
}

fn pt1(monkeys: &[Monkey]) -> MulOutput<[usize; 2]> {
    pts::<20, 3>(monkeys)
}

fn pt2(monkeys: &[Monkey]) -> MulOutput<[usize; 2]> {
    pts::<10_000, 1>(monkeys)
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<Int>,
    operation: (Value, Operation, Value),
    denominator: Int,
    if_divisible: usize,
    if_not_divisible: usize,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Constant(Int),
}

impl Value {
    fn evaluate(self, old: Int) -> Int {
        match self {
            Value::Old => old,
            Value::Constant(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

fn parse(input: &[u8]) -> Result<Vec<Monkey>> {
    use parsers::*;

    let starting_items = number::<Int>().sep_by(token(b", "));
    let value = token((b"old", Value::Old)).or(number::<Int>().map(Value::Constant));
    let op = token((b" + ", Operation::Add)).or(token((b" * ", Operation::Multiply)));
    let operation = token(b"new = ")
        .then(value)
        .and(op)
        .and(value)
        .map(|((a, o), b)| (a, o, b));

    let monkey = (token(b":\n  Starting items: ").then(starting_items))
        .and(token(b"\n  Operation: ").then(operation))
        .and(token(b"\n  Test: divisible by ").then(number::<Int>()))
        .and(token(b"\n    If true: throw to monkey ").then(number::<usize>()))
        .and(token(b"\n    If false: throw to monkey ").then(number::<usize>()))
        .map(
            |((((starting_items, operation), denominator), if_divisible), if_not_divisible)| {
                Monkey {
                    starting_items,
                    operation,
                    denominator,
                    if_divisible,
                    if_not_divisible,
                }
            },
        );

    (token(b"Monkey ").then(number::<usize>()).and(monkey))
        .trailed(token(b"\n\n").opt())
        .fold(Some(Vec::new()), |monkeys, (monkey_index, monkey)| {
            let mut monkeys = monkeys?;
            if monkey_index == monkeys.len() {
                monkeys.push(monkey);
                Some(monkeys)
            } else {
                None
            }
        })
        .map_res(|v| v.ok_or(ParseError::Custom("monkey index does not line up")))
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Monkey 0:
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
    If false: throw to monkey 1
";

    test_pt!(parse, pt1, EXAMPLE => MulOutput([101, 105]));
    test_pt!(parse, pt2, EXAMPLE => MulOutput([52013, 52166]));
}
