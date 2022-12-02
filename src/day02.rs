framework::day!(02, parse => pt1, pt2);

fn pt1(moves: &[(u8, u8)]) -> u32 {
    moves
        .iter()
        .map(|&(opponent, own)| {
            let outcome_score = (4 + own - opponent) % 3 * 3;
            (outcome_score + own + 1) as u32
        })
        .sum()
}

fn pt2(moves: &[(u8, u8)]) -> u32 {
    moves
        .iter()
        .map(|&(opponent, outcome)| {
            let own = (2 + outcome + opponent) % 3;
            let outcome_score = outcome * 3;
            (outcome_score + own + 1) as u32
        })
        .sum()
}

fn parse(input: &[u8]) -> Result<Vec<(u8, u8)>> {
    use parsers::*;
    let opponent = pattern!(b'A'..=b'C').map(|c| c - b'A');
    let instruction = pattern!(b'X'..=b'Z').map(|c| c - b'X');
    let hand = opponent.and(token(b' ').then(instruction));
    hand.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
A Y
B X
C Z";

    test_pt!(parse, pt1, EXAMPLE => 15);
    test_pt!(parse, pt2, EXAMPLE => 12);
}
