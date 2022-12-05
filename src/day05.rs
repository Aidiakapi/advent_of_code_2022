framework::day!(05, parse => pt1, pt2);

type Stacks = Vec<Vec<u8>>;

struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

#[derive(Clone, Copy)]
struct Move {
    from: u8,
    to: u8,
    count: u8,
}

fn pts<const REVERSE: bool>(input: &Input) -> Result<AString> {
    let mut stacks = input.stacks.clone();
    for mv in &input.moves {
        let (from, to) = stacks
            .get_two_mut(mv.from as usize, mv.to as usize)
            .unwrap();
        let count = mv.count as usize;
        to.extend(from.drain(from.len() - count..));
        if REVERSE {
            let target_range = to.len() - count..;
            to[target_range].reverse();
        }
    }

    let mut res = AString::new();
    for stack in &stacks {
        let head = *stack
            .last()
            .ok_or(Error::InvalidInput("ends with empty stack"))?;
        res.push(head);
    }
    Ok(res)
}

fn pt1(input: &Input) -> Result<AString> {
    pts::<true>(input)
}

fn pt2(input: &Input) -> Result<AString> {
    pts::<false>(input)
}

fn parse(input: &[u8]) -> Result<Input> {
    use parsers::*;
    fn parse_stacks(input: &[u8]) -> Result<(Stacks, &[u8])> {
        let tower_lines = input.lines().take_while(|l| !l.is_empty()).collect_vec();
        if tower_lines.len() < 2 {
            return Err(Error::InvalidInput("expected stacks"));
        }
        let last_line = tower_lines[tower_lines.len() - 1];
        if last_line.len() % 4 != 3 {
            return Err(Error::InvalidInput("no stack numbering"));
        }
        let mut stacks = vec![Vec::new(); last_line.len() / 4 + 1];
        for (line_index, line) in tower_lines.iter().rev().skip(1).enumerate() {
            if line.len() != last_line.len() {
                return Err(Error::InvalidInput("invalid line length"));
            }
            for (index, stack) in stacks.iter_mut().enumerate() {
                let start = index * 4;
                if &line[start..start + 3] == b"   " {
                    continue;
                }
                if line[start] != b'[' || line[start + 2] != b']' {
                    return Err(Error::ParseError(ParseError::TokenDoesNotMatch));
                }
                if stack.len() != line_index {
                    return Err(Error::InvalidInput("stack has a gap"));
                }
                stack.push(line[start + 1]);
            }
        }
        Ok((
            stacks,
            &input[(last_line.len() + 1) * tower_lines.len() + 1..],
        ))
    }

    let (stacks, remainder) = parse_stacks(input)?;
    let nr = number::<u8>();
    let idx = nr.map_res(|n| n.checked_sub(1).ok_or(ParseError::Overflow));
    let moves = token(b"move ")
        .then(nr)
        .and(token(b" from ").then(idx))
        .and(token(b" to ").then(idx))
        .map(|((count, from), to)| Move { count, from, to })
        .sep_by(token(b'\n'))
        .execute(remainder)?;
    Ok(Input { stacks, moves })
}

tests! {
    const EXAMPLE: &[u8; 124] = b"    [D]    \n\
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    test_pt!(parse, pt1, EXAMPLE => b"CMZ");
    test_pt!(parse, pt2, EXAMPLE => b"MCD");
}
