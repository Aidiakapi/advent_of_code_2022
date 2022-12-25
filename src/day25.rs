framework::day!(25, parse => pt1, pt2);

fn digit_to_int(digit: u8) -> i64 {
    match digit {
        b'0'..=b'2' => (digit - b'0') as i64,
        b'-' => -1,
        b'=' => -2,
        _ => unreachable!(),
    }
}

fn str_to_int(str: &[u8]) -> i64 {
    str.iter().fold(0, |acc, &nr| acc * 5 + digit_to_int(nr))
}

fn int_to_str(int: i64) -> AString {
    if int == 0 {
        return vec![b'0'];
    }
    assert!(int > 0);
    let mut str = AString::new();
    let mut remainder = int;
    loop {
        let digit = remainder % 5;
        remainder /= 5;
        str.push(match digit {
            0..=2 => b'0' + digit as u8,
            3 => {
                remainder += 1;
                b'='
            }
            4 => {
                remainder += 1;
                b'-'
            }
            _ => unreachable!(),
        });
        if remainder == 0 {
            break;
        }
    }
    str.reverse();
    str
}

fn pt1(input: &[&[u8]]) -> AString {
    int_to_str(input.iter().cloned().map(str_to_int).sum::<i64>())
}

fn pt2(_: &[&[u8]]) -> &'static str {
    "gg"
}

fn parse(input: &[u8]) -> Result<Vec<&[u8]>> {
    use parsers::*;
    take_while((), |_, c| matches!(c, b'0'..=b'2' | b'-' | b'='))
        .sep_by(token(b'\n'))
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    test_pt!(parse, pt1, EXAMPLE => b"2=-1=0");
}
