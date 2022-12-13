framework::day!(13, parse => pt1, pt2);
use std::{cmp::Ordering, ptr};

type Int = u32;

fn pt1(packet_pairs: &[(Packet, Packet)]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn pt2(packet_pairs: &[(Packet, Packet)]) -> MulOutput<[usize; 2]> {
    let mut packets = packet_pairs.iter().flat_map(|(a, b)| [a, b]).collect_vec();

    let d2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let d6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(&d2);
    packets.push(&d6);

    packets.sort();

    let i2 = (packets.iter().position(|p| ptr::eq(&d2, *p))).unwrap() + 1;
    let i6 = (packets[i2..].iter().position(|p| ptr::eq(&d6, *p))).unwrap() + i2 + 1;

    MulOutput([i2, i6])
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(Int),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(x), Packet::Number(y)) => x.cmp(y),
            (Packet::List(x), Packet::List(y)) => {
                for (n, m) in x.iter().zip(y.iter()) {
                    match n.cmp(m) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                x.len().cmp(&y.len())
            }
            (Packet::Number(x), ys @ Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*x)]).cmp(ys)
            }
            (xs @ Packet::List(_), Packet::Number(y)) => {
                xs.cmp(&Packet::List(vec![Packet::Number(*y)]))
            }
        }
    }
}

fn parse(input: &[u8]) -> Result<Vec<(Packet, Packet)>> {
    use parsers::*;
    fn list(input: &[u8]) -> ParseResult<'_, Packet> {
        let (o, s, c) = (token(b'['), token(b','), token(b']'));
        (o.then(packet.sep_by(s).opt()).trailed(c))
            .map(Option::unwrap_or_default)
            .map(Packet::List)
            .parse(input)
    }
    fn packet(input: &[u8]) -> ParseResult<'_, Packet> {
        let nr = number::<Int>().map(Packet::Number);
        nr.or(list).parse(input)
    }

    let parser = packet.and(token(b'\n').then(packet)).sep_by(token(b"\n\n"));
    parser.execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    test_pt!(parse, pt1, EXAMPLE => 13);
    test_pt!(parse, pt2, EXAMPLE => MulOutput([10, 14]));
}
