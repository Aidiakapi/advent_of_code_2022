use std::ops::Range;
framework::day!(22, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<usize>;
type Grid = VecGrid<Cell>;
type Output = CombiOutput<[usize; 4]>;

const LFT: Offset = Offset::X_NEG;
const RGT: Offset = Offset::X_POS;
const TOP: Offset = Offset::Y_NEG;
const BOT: Offset = Offset::Y_POS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Void,
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn get_boundaries<'a>(
    grid: &Grid,
    storage: &'a mut Vec<Range<usize>>,
) -> (&'a [Range<usize>], &'a [Range<usize>]) {
    macro_rules! count {
        ($iter:expr, $iter_value:ident, $x:ident, $y:ident) => {
            $iter
                .take_while(|&$iter_value| grid[($x, $y)] == Cell::Void)
                .count()
        };
    }
    let mut boundaries = Vec::with_capacity(grid.width() + grid.height());
    for x in 0..grid.width() {
        let top = count!(0..grid.height(), y, x, y);
        let bottom = grid.height() - count!((0..grid.height()).rev(), y, x, y);
        boundaries.push(top..bottom);
    }
    for y in 0..grid.height() {
        let left = count!(0..grid.width(), x, x, y);
        let right = grid.width() - count!((0..grid.width()).rev(), x, x, y);
        boundaries.push(left..right);
    }
    std::mem::swap(storage, &mut boundaries);
    (&storage[..grid.width()], &storage[grid.width()..])
}

fn get_output(position: Vec2, direction: Offset) -> Output {
    let position = position + 1;
    let direction_nr = match direction {
        RGT => 0,
        BOT => 1,
        LFT => 2,
        TOP => 3,
        _ => unreachable!(),
    };
    CombiOutput([
        position.y,
        position.x,
        direction_nr,
        1000 * position.y + 4 * position.x + direction_nr,
    ])
}

fn pt1((grid, instructions): &(Grid, Vec<Instruction>)) -> Output {
    let mut boundaries_storage = Vec::new();
    let (vertical_bounds, horizontal_bounds) = get_boundaries(grid, &mut boundaries_storage);

    let mut position = Vec2::new(horizontal_bounds[0].start, 0);
    let mut direction = RGT;
    for instruction in instructions {
        let distance = match instruction {
            Instruction::Move(n) => *n,
            Instruction::TurnLeft => {
                direction = direction.rot_270();
                continue;
            }
            Instruction::TurnRight => {
                direction = direction.rot_90();
                continue;
            }
        };

        let (range, component): (_, fn(&mut Vec2) -> &mut usize) = if direction.has_x() {
            (horizontal_bounds[position.y].clone(), |p| &mut p.x)
        } else {
            (vertical_bounds[position.x].clone(), |p| &mut p.y)
        };
        let positive = matches!(direction, RGT | BOT);
        for _ in 0..distance {
            let mut next_position = position;
            let comp = component(&mut next_position);
            if positive {
                *comp += 1;
                if *comp == range.end {
                    *comp = range.start;
                }
            } else {
                if *comp == range.start {
                    *comp = range.end;
                }
                *comp -= 1;
            }
            if grid[next_position] == Cell::Wall {
                break;
            }
            position = next_position;
        }
    }

    get_output(position, direction)
}

type Face = framework::vecs::Vec2<i8>;

fn face_exists<const N: usize>(grid: &Grid, face: Face) -> bool {
    (face.x >= 0 && face.y >= 0)
        && matches!(grid.get(face.to_usize() * N), Some(Cell::Open | Cell::Wall))
}

fn get_connection<const N: usize>(
    grid: &Grid,
    from: Face,
    to: Face,
) -> Option<(Offset, Offset, bool)> {
    debug_assert!(face_exists::<N>(grid, from) && face_exists::<N>(grid, to));
    let mut delta = to - from;

    let flipped_x = delta.x < 0;
    if flipped_x {
        delta.x = -delta.x;
    }
    let flipped_y = delta.y < 0;
    if flipped_y {
        delta.y = -delta.y;
    }

    let transposed = delta.y > delta.x;
    if transposed {
        delta = delta.transpose();
    }

    let has = |dx, dy| {
        let mut d = Face::new(dx, dy);
        if transposed {
            d = d.transpose();
        }
        if flipped_y {
            d.y = -d.y;
        }
        if flipped_x {
            d.x = -d.x;
        }
        face_exists::<N>(grid, from + d)
    };

    // Outgoing is the edge on the `from` cell that is being left
    // Incoming is the edge on the `to` cell that is being entered
    //
    // These edges are a 1D structure, a line segment, and one of the position
    // components determines the length along this line. Swapped say whether
    // this component lines up, or goes in the opposite direction.
    //
    // For example, if the top and bottom are connected, then the x-position
    // determines where on this edge the boundary is being crossed. If swapped
    // is true, then the local-space x coordinate in `to` is the reverse of that
    // in `from`, ie: x_to = N - 1 - x_from.
    let (mut outgoing, mut incoming, mut swapped) = match (delta.x, delta.y) {
        (1, 0) => (RGT, LFT, false),
        (1, 1) if has(0, 1) => (RGT, TOP, true),
        (1, 1) if has(1, 0) => (BOT, LFT, true),
        (2, 1) if has(0, 1) && has(1, 1) => (TOP, TOP, true),
        (2, 1) if has(1, 0) && has(2, 0) => (BOT, BOT, true),
        (3, 0) if has(1, 0) && has(2, 0) => (LFT, RGT, false),
        (2, 2) if has(0, 1) && has(1, 1) && has(1, 2) => (TOP, RGT, true),
        (2, 2) if has(1, 0) && has(1, 1) && has(2, 1) => (LFT, BOT, true),
        (3, 1) if has(0, 1) && has(1, 1) && has(2, 1) => (LFT, TOP, true),
        (3, 1) if has(1, 0) && has(1, 1) && has(2, 1) => (TOP, RGT, true),
        (3, 1) if has(1, 0) && has(2, 0) && has(2, 1) => (LFT, BOT, true),
        (3, 1) if has(1, 0) && has(2, 0) && has(3, 0) => (BOT, RGT, true),
        (3, 2) if has(0, 1) && has(1, 1) && has(2, 1) && has(2, 2) => (LFT, RGT, false),
        (3, 2) if has(1, 0) && has(1, 1) && has(2, 1) && has(2, 2) => (TOP, BOT, false),
        (3, 2) if has(1, 0) && has(1, 1) && has(2, 1) && has(3, 1) => (LFT, RGT, false),
        (4, 1) if has(1, 0) && has(2, 0) && has(2, 1) && has(3, 1) => (TOP, BOT, false),
        _ => return None,
    };

    if transposed {
        outgoing = outgoing.transpose();
        incoming = incoming.transpose();
    }

    if flipped_y {
        outgoing = outgoing.flip_y();
        incoming = incoming.flip_y();
        if (outgoing.has_y() as usize + incoming.has_y() as usize) % 2 == 1 {
            swapped = !swapped;
        }
    }

    if flipped_x {
        outgoing = outgoing.flip_x();
        incoming = incoming.flip_x();
        if (outgoing.has_x() as usize + incoming.has_x() as usize) % 2 == 1 {
            swapped = !swapped;
        }
    }

    Some((outgoing, incoming, swapped))
}

#[derive(Debug, Clone, Default)]
struct FaceInfo {
    position: Face,
    lft: (usize, Offset, bool),
    rgt: (usize, Offset, bool),
    top: (usize, Offset, bool),
    bot: (usize, Offset, bool),
}

fn get_face_info<const N: usize>(grid: &Grid) -> [FaceInfo; 6] {
    let mut faces = (0..grid.height() / N)
        .flat_map(|y| (0..grid.width() / N).map(move |x| Face::new(x as i8, y as i8)))
        .filter(|&face| face_exists::<N>(grid, face))
        .map(|face| FaceInfo {
            position: face,
            ..Default::default()
        })
        .collect_array()
        .unwrap();

    for i in 0..6 {
        let face = faces[i].position;
        let Some(connections) = (0..6)
            .filter(|&j| j != i)
            .filter_map(|j| get_connection::<N>(grid, face, faces[j].position).map(move |v| (j, v)))
            .collect_array::<4>()
            else { panic!("failed to get 4 connections for face {i} at position {face}") };

        let face_info = &mut faces[i];

        for (idx, (src, dst, swap)) in connections {
            match src {
                LFT if face_info.lft.1 == Offset::NONE => face_info.lft = (idx, dst, swap),
                RGT if face_info.rgt.1 == Offset::NONE => face_info.rgt = (idx, dst, swap),
                TOP if face_info.top.1 == Offset::NONE => face_info.top = (idx, dst, swap),
                BOT if face_info.bot.1 == Offset::NONE => face_info.bot = (idx, dst, swap),
                LFT => panic!("multiple of LFT {} and {idx}", face_info.lft.0),
                RGT => panic!("multiple of RGT {} and {idx}", face_info.rgt.0),
                TOP => panic!("multiple of TOP {} and {idx}", face_info.top.0),
                BOT => panic!("multiple of BOT {} and {idx}", face_info.bot.0),
                _ => panic!("unknown source face: {src:?}"),
            }
        }
    }

    faces
}

fn move_step<const N: usize>(
    faces: &[FaceInfo; 6],
    face: &mut usize,
    position: &mut Vec2,
    direction: &mut Offset,
) {
    let info = &faces[*face];
    let ((target_face, edge, swap), mut offset) = match *direction {
        LFT if position.x > 0 => {
            position.x -= 1;
            return;
        }
        RGT if position.x < N - 1 => {
            position.x += 1;
            return;
        }
        TOP if position.y > 0 => {
            position.y -= 1;
            return;
        }
        BOT if position.y < N - 1 => {
            position.y += 1;
            return;
        }
        LFT => (info.lft, position.y),
        RGT => (info.rgt, position.y),
        TOP => (info.top, position.x),
        BOT => (info.bot, position.x),
        _ => unreachable!(),
    };

    if swap {
        offset = N - 1 - offset;
    }

    *face = target_face;
    *direction = edge.flip_x().flip_y();
    *position = Vec2::from(match edge {
        LFT => (0, offset),
        RGT => (N - 1, offset),
        TOP => (offset, 0),
        BOT => (offset, N - 1),
        _ => unreachable!(),
    });
}

fn pt2_impl<const N: usize>((grid, instructions): &(Grid, Vec<Instruction>)) -> Output {
    let face_info = get_face_info::<N>(grid);
    let mut face = 0;
    let mut position = Vec2::zero();
    let mut direction = RGT;

    for instruction in instructions {
        let distance = match instruction {
            Instruction::Move(n) => *n,
            Instruction::TurnLeft => {
                direction = direction.rot_270();
                continue;
            }
            Instruction::TurnRight => {
                direction = direction.rot_90();
                continue;
            }
        };

        for _ in 0..distance {
            let mut next_face = face;
            let mut next_position = position;
            let mut next_direction = direction;
            move_step::<N>(
                &face_info,
                &mut next_face,
                &mut next_position,
                &mut next_direction,
            );

            let grid_pos = face_info[next_face].position.to_usize() * N + next_position;
            if grid[grid_pos] == Cell::Wall {
                break;
            }
            (face, position, direction) = (next_face, next_position, next_direction);
        }
    }

    let position = face_info[face].position.to_usize() * N + position;
    get_output(position, direction)
}

fn pt2(input: &(Grid, Vec<Instruction>)) -> Output {
    pt2_impl::<50>(input)
}

fn parse(input: &[u8]) -> Result<(Grid, Vec<Instruction>)> {
    use parsers::*;
    let input = input.trim_ascii_end();
    let (max_len, total_len, count) = input.lines().take_while(|line| !line.is_empty()).fold(
        (0, 0, 0),
        |(max_len, total_len, count), line| {
            (max_len.max(line.len()), total_len + line.len(), count + 1)
        },
    );
    let additional_spacing = count * max_len - total_len;
    let mut space_padded_input = AString::with_capacity(input.len() + additional_spacing);
    for line in input.lines() {
        if !line.is_empty() {
            space_padded_input.extend_from_slice(line);
            let new_size = space_padded_input.len() + max_len.saturating_sub(line.len());
            space_padded_input.resize(new_size, b' ');
        }
        space_padded_input.push(b'\n');
    }
    let space_padded_input = space_padded_input.trim_ascii_end();

    let cell = any().map_res(|c| match c {
        b' ' => Ok(Cell::Void),
        b'.' => Ok(Cell::Open),
        b'#' => Ok(Cell::Wall),
        _ => Err(ParseError::TokenDoesNotMatch),
    });
    let grid = grid(cell, token(b'\n'));

    let turn = any().map_res(|c| match c {
        b'R' => Ok(Instruction::TurnRight),
        b'L' => Ok(Instruction::TurnLeft),
        _ => Err(ParseError::TokenDoesNotMatch),
    });
    let instruction = number::<usize>().map(Instruction::Move).or(turn);
    let instructions = instruction.repeat_into();

    grid.and(token(b'\n').then(instructions))
        .execute(space_padded_input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"        \
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    test_pt!(parse, pt1, EXAMPLE => CombiOutput([6, 8, 0, 6032]));
    test_pt!(parse, pt2, |input| { pt2_impl::<4>(&input) }, EXAMPLE => CombiOutput([5, 7, 3, 5031]));

    // These are the 11 possible nets for a cube in a "parsable" format.
    const NETS: [&'static [u8]; 11] = [
        b"  .\n....\n  .\n\n1",
        b"..\n ...\n   .\n\n1",
        b".\n....\n   .\n\n1",
        b".\n....\n  .\n\n1",
        b"..\n ..\n  ..\n\n1",
        b"..\n ...\n  .\n\n1",
        b"  ..\n...\n  .\n\n1",
        b".\n....\n.\n\n1",
        b" .\n....\n  .\n\n1",
        b"...\n  ...\n\n1",
        b".\n....\n .\n\n1",
    ];

    #[test]
    fn all_nets() {
        let transforms = [
            |g: Grid| Grid::new((g.height(), g.width()), |p| g[p.transpose()]),
            |g: Grid| Grid::new(g.size(), |p| g[Vec2::new(g.width() - 1 - p.x, p.y)]),
            |g: Grid| Grid::new(g.size(), |p| g[Vec2::new(p.x, g.height() - 1 - p.y)]),
        ];

        const N: usize = 4;
        for (net, transform) in NETS.into_iter().zip(0..1 << 3) {
            let mut grid = parse(net).unwrap().0;
            if transform & 0b001 != 0 {
                grid = transforms[0](grid);
            }
            if transform & 0b010 != 0 {
                grid = transforms[1](grid);
            }
            if transform & 0b100 != 0 {
                grid = transforms[2](grid);
            }
            let grid = &VecGrid::new(grid.size() * N, |c| grid[c / N]);
            let faces = get_face_info::<N>(grid);

            for f in 0..faces.len() {
                for (p, d) in [
                    (Vec2::new(0, 1), LFT),
                    (Vec2::new(N - 1, 1), RGT),
                    (Vec2::new(1, 0), TOP),
                    (Vec2::new(1, N - 1), BOT),
                ] {
                    // Test that crossing a border, turining around, crossing
                    // again ends up in the starting position, but turned around.
                    let (mut nf, mut np, mut nd) = (f, p, d);
                    move_step::<N>(&faces, &mut nf, &mut np, &mut nd);
                    nd = nd.rot_180();
                    move_step::<N>(&faces, &mut nf, &mut np, &mut nd);
                    assert_eq!(nf, f);
                    assert_eq!(np, p);
                    assert_eq!(nd.rot_180(), d);
                }
            }
        }
    }
}
