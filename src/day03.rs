framework::day!(03, parse => pt1, pt2);

fn letter_to_index(letter: u8) -> u8 {
    match letter {
        b'a'..=b'z' => letter - b'a',
        b'A'..=b'Z' => letter - b'A' + 26,
        _ => unreachable!(),
    }
}

fn backpack_to_bitset(backpack: &[u8]) -> u64 {
    let mut set = 0u64;
    for &letter in backpack {
        set |= 1 << letter_to_index(letter);
    }
    set
}

fn pt1(backpacks: &[&[u8]]) -> Result<u32> {
    backpacks
        .iter()
        .map(|backpack| {
            let (section1, section2) = backpack.split_at(backpack.len() / 2);
            let used = backpack_to_bitset(section1);
            for &letter in section2 {
                let index = letter_to_index(letter);
                if used & 1 << index != 0 {
                    return Ok(index as u32 + 1);
                }
            }
            Err(Error::InvalidInput(
                "no duplicate item type between bag sections",
            ))
        })
        .sum()
}

fn pt2(backpacks: &[&[u8]]) -> u32 {
    backpacks
        .iter()
        .map(|backpack| backpack_to_bitset(*backpack))
        .tuples()
        .map(|(s1, s2, s3)| {
            let overlap = s1 & s2 & s3;
            overlap.trailing_zeros() + 1
        })
        .sum()
}

fn parse(input: &[u8]) -> Result<Vec<&[u8]>> {
    use parsers::*;
    take_while((), |_, l| matches!(l, b'a'..=b'z' | b'A'..=b'Z'))
        .sep_by(token(b'\n'))
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    test_pt!(parse, pt1, EXAMPLE => 157);
    test_pt!(parse, pt2, EXAMPLE => 70);
}
