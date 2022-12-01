framework::day!(01, parse => pt1, pt2);

fn pt1(elves: &[Vec<u32>]) -> u32 {
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn pt2(elves: &[Vec<u32>]) -> u32 {
    elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .sorted()
        .skip(elves.len() - 3)
        .sum()
}

fn parse(input: &[u8]) -> Result<Vec<Vec<u32>>> {
    use parsers::*;
    let nr = number::<u32>();
    let elf = nr.sep_by(token(b'\n'));
    elf.sep_by(token(b"\n\n")).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    test_pt!(parse, pt1, EXAMPLE => 24000);
    test_pt!(parse, pt2, EXAMPLE => 45000);
}
