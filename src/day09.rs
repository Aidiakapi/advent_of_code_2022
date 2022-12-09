framework::day!(09, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<i32>;

#[derive(Debug, Clone, Copy)]
struct Move {
    dir: Offset,
    dist: u32,
}

fn moves_iter(moves: &[Move]) -> impl Iterator<Item = Offset> + '_ {
    moves
        .iter()
        .flat_map(|mv| (0..mv.dist).map(move |_| mv.dir))
}

fn move_point(point: &mut Vec2, parent: Vec2) {
    let delta = parent - *point;
    if delta.x.abs() > 1 || delta.y.abs() > 1 {
        *point += delta.clamp(-1, 1)
    }
}

fn pts<const N: usize>(moves: &[Move]) -> usize {
    let mut points = [Vec2::zero(); N];
    let mut visited = HashSet::new();
    for dir in moves_iter(moves) {
        points[0] = points[0].neighbor(dir).unwrap();
        let mut iter = points.windows_mut();
        while let Some([head, tail]) = iter.next() {
            move_point(tail, *head);
        }
        visited.insert(points[points.len() - 1]);
    }

    visited.len()
}

fn pt1(moves: &[Move]) -> usize {
    pts::<2>(moves)
}

fn pt2(moves: &[Move]) -> usize {
    pts::<10>(moves)
}

fn parse(input: &[u8]) -> Result<Vec<Move>> {
    use parsers::*;
    let dir = any().map_res(|f| match f {
        b'R' => Ok(Offset::X_POS),
        b'L' => Ok(Offset::X_NEG),
        b'D' => Ok(Offset::Y_POS),
        b'U' => Ok(Offset::Y_NEG),
        _ => Err(ParseError::TokenDoesNotMatch),
    });
    let dist = number::<u32>();
    let mv = dir.and(token(b' ').then(dist));
    let mv = mv.map(|(dir, dist)| Move { dir, dist });
    mv.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    test_pt!(parse, pt1, EXAMPLE => 13);
    test_pt!(parse, pt2, EXAMPLE => 1,
b"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20" => 36);
}
