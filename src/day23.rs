framework::day!(23, parse => pt1, pt2);

type Grid = VecGrid<bool>;

// This is the padding applied to the input, in order have space to grow.
// It is significantly larger than was necessary for my input:
const OFFSET: Vec2<usize> = Vec2::new(25, 25);
const ADDITIONAL_SIZE: Vec2<usize> = Vec2::new(88, 88);

// The exact values for my input are:
// const OFFSET: Vec2<usize> = Vec2::new(15, 14);
// const ADDITIONAL_SIZE: Vec2<usize> = Vec2::new(68, 67);

// That said, the actual allocation of the grid is a tiny fraction of the
// overall performance.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Elf,
    Empty(u32),
    Reservation(u32, u32, Offset),
}

fn pts<const LIMITED: bool>(grid: &Grid) -> (Vec<Vec2<usize>>, u32) {
    let mut elves = Vec::new();
    let mut grid = VecGrid::new(grid.size() + ADDITIONAL_SIZE, |pos| {
        if pos.x < OFFSET.x || pos.y < OFFSET.y {
            return Cell::Empty(u32::MAX);
        }
        match grid.get(pos - OFFSET) {
            Some(true) => {
                elves.push(pos);
                Cell::Elf
            }
            _ => Cell::Empty(u32::MAX),
        }
    });

    const CHECK_DIRECTIONS: [[Offset; 3]; 4] = [
        [Offset::X_NEG_Y_NEG, Offset::Y_NEG, Offset::X_POS_Y_NEG],
        [Offset::X_NEG_Y_POS, Offset::Y_POS, Offset::X_POS_Y_POS],
        [Offset::X_NEG_Y_NEG, Offset::X_NEG, Offset::X_NEG_Y_POS],
        [Offset::X_POS_Y_NEG, Offset::X_POS, Offset::X_POS_Y_POS],
    ];

    let mut checks = CHECK_DIRECTIONS
        .map(|dirs| dirs.map(|dir| Offset::ALL.iter().position(|&d| d == dir).unwrap()));

    let mut rounds = 0u32;
    let mut reservations = Vec::new();
    loop {
        for (elf_index, elf) in elves.iter().enumerate() {
            let neighbors = Offset::ALL.map(|n| grid[elf.neighbor(n).unwrap()] == Cell::Elf);
            if !neighbors.contains(&true) {
                continue;
            }
            for dirs in checks {
                if dirs.iter().any(|&n| neighbors[n]) {
                    continue;
                }
                let new_position = elf.neighbor(Offset::ALL[dirs[1]]).unwrap();
                let cell = &mut grid[new_position];
                if let Cell::Empty(key) | Cell::Reservation(key, _, _) = cell && *key == rounds {
                    *cell = Cell::Empty(rounds)
                } else {
                    *cell = Cell::Reservation(rounds, elf_index as u32, Offset::ALL[dirs[1]]);
                    reservations.push(new_position);
                }
                break;
            }
        }

        if reservations.is_empty() {
            break;
        }

        for position in reservations.drain(..) {
            let cell = &mut grid[position];
            if let Cell::Reservation(key, elf_index, dir) = *cell {
                debug_assert_eq!(key, rounds);
                *cell = Cell::Elf;
                grid[position.neighbor(dir.rot_180()).unwrap()] = Cell::Empty(rounds);
                elves[elf_index as usize] = position;
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
        (
            Vec2::new(usize::MAX, usize::MAX),
            Vec2::new(usize::MIN, usize::MIN),
        ),
        |(min, max), &p| (min.min_comp(p), max.max_comp(p)),
    );

    let size = (max + 1 - min).to_usize();
    size.x * size.y - elves.len()
}

fn pt2(grid: &Grid) -> u32 {
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
