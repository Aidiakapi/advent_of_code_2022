framework::day!(04, parse => pt1, pt2);

type Range = std::ops::RangeInclusive<u32>;

fn pt1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|&(a, b)| {
            let is_inside_of = |a: &Range, b: &Range| a.start() >= b.start() && a.end() <= b.end();
            is_inside_of(a, b) || is_inside_of(b, a)
        })
        .count()
}

fn pt2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|&(a, b)| a.start() <= b.end() && a.end() >= b.start())
        .count()
}

fn parse(input: &[u8]) -> Result<Vec<(Range, Range)>> {
    use parsers::*;
    let nr = number::<u32>();
    let range = nr.and(token(b'-').then(nr)).map(|(from, to)| from..=to);
    let pair = range.and(token(b',').then(range));
    pair.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    test_pt!(parse, pt1, EXAMPLE => 2);
    test_pt!(parse, pt2, EXAMPLE => 4);
}
