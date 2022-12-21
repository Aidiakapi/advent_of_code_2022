framework::day!(21, parse => pt1, pt2);

type Word = [u8; 4];
type Value = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Constant(Value),
    Add(Word, Word),
    Subtract(Word, Word),
    Multiply(Word, Word),
    Divide(Word, Word),
}

impl Op {
    fn get_words(self) -> Option<(Word, Word)> {
        match self {
            Op::Constant(_) => None,
            Op::Add(a, b) => Some((a, b)),
            Op::Subtract(a, b) => Some((a, b)),
            Op::Multiply(a, b) => Some((a, b)),
            Op::Divide(a, b) => Some((a, b)),
        }
    }
}

fn get_value(monkeys: &mut HashMap<Word, Op>, monkey: Word) -> Value {
    let new_value = match monkeys[&monkey] {
        Op::Constant(value) => return value,
        Op::Add(a, b) => get_value(monkeys, a) + get_value(monkeys, b),
        Op::Subtract(a, b) => get_value(monkeys, a) - get_value(monkeys, b),
        Op::Multiply(a, b) => get_value(monkeys, a) * get_value(monkeys, b),
        Op::Divide(a, b) => get_value(monkeys, a) / get_value(monkeys, b),
    };
    monkeys.insert(monkey, Op::Constant(new_value));
    new_value
}

fn pt1(monkeys: &[(Word, Op)]) -> Value {
    let mut monkeys = monkeys.iter().cloned().collect();

    get_value(&mut monkeys, *b"root")
}

fn pt2(monkeys: &[(Word, Op)]) -> Value {
    let mut monkeys = monkeys.iter().cloned().collect::<HashMap<_, _>>();
    let mut path = Vec::new();

    fn find_path_to_humn(monkeys: &HashMap<Word, Op>, path: &mut Vec<Word>, monkey: Word) -> bool {
        if monkey == *b"humn" {
            return true;
        }
        let (a, b) = match monkeys[&monkey].get_words() {
            Some(words) => words,
            None => return false,
        };

        path.push(monkey);
        if find_path_to_humn(monkeys, path, a) {
            return true;
        }
        if find_path_to_humn(monkeys, path, b) {
            return true;
        }
        path.pop();
        false
    }
    find_path_to_humn(&monkeys, &mut path, *b"root");

    let Op::Add(a, b) = monkeys[b"root"] else { unreachable!() };
    monkeys.insert(*b"root", Op::Subtract(a, b));

    let mut should_be = 0;
    path.push(*b"humn");
    for &[monkey, next_monkey] in path.array_windows() {
        let op = monkeys[&monkey];
        let (a, b) = op.get_words().unwrap();
        let a_next = a == next_monkey;
        let other_monkey = if a_next { b } else { a };
        let other_value = get_value(&mut monkeys, other_monkey);
        should_be = match op {
            Op::Constant(_) => unreachable!(),
            Op::Add(_, _) => should_be - other_value,
            Op::Subtract(_, _) => {
                if a_next {
                    other_value + should_be
                } else {
                    other_value - should_be
                }
            }
            Op::Multiply(_, _) => should_be / other_value,
            Op::Divide(_, _) => {
                if a_next {
                    other_value * should_be
                } else {
                    other_value / should_be
                }
            }
        };
    }

    should_be as Value
}

fn parse(input: &[u8]) -> Result<Vec<(Word, Op)>> {
    use parsers::*;
    let word = pattern!(b'a'..=b'z').many_n::<4>();
    let value = number::<Value>();
    let op = word
        .and(token(b' ').then(any()).trailed(token(b' ')))
        .and(word)
        .map_res(|((a, op), b)| match op {
            b'+' => Ok(Op::Add(a, b)),
            b'-' => Ok(Op::Subtract(a, b)),
            b'*' => Ok(Op::Multiply(a, b)),
            b'/' => Ok(Op::Divide(a, b)),
            _ => Err(ParseError::TokenDoesNotMatch),
        })
        .or(value.map(Op::Constant));

    let monkey = word.and(token(b": ").then(op));
    monkey.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    test_pt!(parse, pt1, EXAMPLE => 152);
    test_pt!(parse, pt2, EXAMPLE => 301);
}
