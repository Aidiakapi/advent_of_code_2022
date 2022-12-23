framework::day!(23, parse => pt1, pt2);

type Grid = VecGrid<bool>;

enum Reservation {
    Single(Vec2<i32>),
    Multiple,
}

fn pts<const LIMITED: bool>(grid: &Grid) -> (HashSet<Vec2<i32>>, usize) {
    let mut elves: HashSet<_> = grid
        .iter()
        .filter(|(_, &value)| value)
        .map(|(position, _)| position.to_i32())
        .collect();
    let mut reservations = HashMap::new();

    const CHECK_DIRECTIONS: [[Offset; 3]; 4] = [
        [Offset::X_NEG_Y_NEG, Offset::Y_NEG, Offset::X_POS_Y_NEG],
        [Offset::X_NEG_Y_POS, Offset::Y_POS, Offset::X_POS_Y_POS],
        [Offset::X_NEG_Y_NEG, Offset::X_NEG, Offset::X_NEG_Y_POS],
        [Offset::X_POS_Y_NEG, Offset::X_POS, Offset::X_POS_Y_POS],
    ];

    let mut checks = CHECK_DIRECTIONS
        .map(|dirs| dirs.map(|dir| Offset::ALL.iter().position(|&d| d == dir).unwrap()));

    let mut rounds = 0;
    loop {
        for &elf in &elves {
            let neighbors = Offset::ALL.map(|n| elves.contains(&elf.neighbor(n).unwrap()));
            if !neighbors.contains(&true) {
                continue;
            }
            for dirs in checks {
                if dirs.iter().any(|&n| neighbors[n]) {
                    continue;
                }
                let new_position = elf.neighbor(Offset::ALL[dirs[1]]).unwrap();
                reservations
                    .entry(new_position)
                    .and_modify(|v| *v = Reservation::Multiple)
                    .or_insert(Reservation::Single(elf));
                break;
            }
        }

        if reservations.is_empty() {
            break;
        }

        for (position, reservation) in reservations.drain() {
            match reservation {
                Reservation::Single(previous_position) => {
                    elves.remove(&previous_position);
                    elves.insert(position);
                }
                Reservation::Multiple => {}
            }
        }

        checks.rotate_left(1);
        rounds += 1;
        if LIMITED && rounds >= 10 {
            break;
        }
    }

    (elves, rounds)
}

fn pt1(grid: &Grid) -> usize {
    let (elves, _) = pts::<true>(grid);

    let (min, max) = elves.iter().fold(
        (Vec2::new(i32::MAX, i32::MAX), Vec2::new(i32::MIN, i32::MIN)),
        |(min, max), &p| (min.min_comp(p), max.max_comp(p)),
    );

    let size = (max + 1 - min).to_usize();
    size.x * size.y - elves.len()
}

fn pt2(grid: &Grid) -> usize {
    let (_, rounds) = pts::<false>(grid);
    rounds + 1
}

fn parse(input: &[u8]) -> Result<Grid> {
    use parsers::*;
    let cell = token((b'.', false)).or(token((b'#', true)));
    grid(cell, token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    test_pt!(parse, pt1, EXAMPLE => 110);
    test_pt!(parse, pt2, EXAMPLE => 20);
}
