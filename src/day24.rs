use num::Integer;
framework::day!(24, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<usize>;
type Grid = VecGrid<Option<Direction>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn pts<const BACK_AND_FORTH: bool>(grid: &Grid) -> Result<[usize; 3]> {
    let size = grid.size();
    let mut blizzards = [
        vec![Vec::new(); size.y],
        vec![Vec::new(); size.y],
        vec![Vec::new(); size.x],
        vec![Vec::new(); size.x],
    ];
    for (position, direction) in grid
        .iter()
        .filter_map(|(position, direction)| direction.map(move |direction| (position, direction)))
    {
        match direction {
            Direction::Left => blizzards[0][position.y].push(position.x),
            Direction::Right => blizzards[1][position.y].push(position.x),
            Direction::Up => blizzards[2][position.x].push(position.y),
            Direction::Down => blizzards[3][position.x].push(position.y),
        }
    }

    for direction in &mut blizzards {
        for row_or_column in direction {
            row_or_column.sort_unstable();
        }
    }

    let search = #[inline]
    |collection: usize, distance: usize, target: usize| {
        blizzards[collection][distance]
            .binary_search(&target)
            .is_ok()
    };

    let is_free = |time: usize, position: Vec2| {
        let offset_neg = (position + time) % size;
        if search(0, position.y, offset_neg.x) || search(2, position.x, offset_neg.y) {
            return false;
        }
        let offset_pos = (position + size - Vec2::new(time, time) % size) % size;
        if search(1, position.y, offset_pos.x) || search(3, position.x, offset_pos.y) {
            return false;
        }
        true
    };

    let bottom_right = size - 1;
    let cycle_length = size.x.lcm(&size.y);
    let find_path = |from: Vec2, to: Vec2, starting_time: usize| -> Result<usize> {
        graph::astar_path_cost(
            |starting_points| {
                for time in starting_time..starting_time + cycle_length {
                    if is_free(time, from) {
                        starting_points.push(((time, from), time));
                    }
                }
            },
            |&(time, position), next| {
                let time = time + 1;
                let mut add_if_free = |position: Vec2| {
                    if is_free(time, position) {
                        next.push(((time, position), 1));
                    }
                };
                add_if_free(position);
                if position.x > 0 {
                    add_if_free(Vec2::new(position.x - 1, position.y));
                }
                if position.x < bottom_right.x {
                    add_if_free(Vec2::new(position.x + 1, position.y));
                }
                if position.y > 0 {
                    add_if_free(Vec2::new(position.x, position.y - 1));
                }
                if position.y < bottom_right.y {
                    add_if_free(Vec2::new(position.x, position.y + 1));
                }
            },
            |&(_, position)| position.manhathan_distance(to),
            |&(_, position)| position == to,
        ).ok_or(Error::NoSolution)
        .map(|cost| cost + 1)
    };

    let time = find_path(Vec2::zero(), bottom_right, 0)?;
    if !BACK_AND_FORTH {
        return Ok([time, 0, 0]);
    }

    let back = find_path(bottom_right, Vec2::zero(), time)?;
    let forth = find_path(Vec2::zero(), bottom_right, back)?;
    Ok([time, back - time, forth - back])
}

fn pt1(grid: &Grid) -> Result<usize> {
    pts::<false>(grid).map(|[time, ..]| time)
}

fn pt2(grid: &Grid) -> Result<AddOutput<[usize; 3]>> {
    pts::<true>(grid).map(AddOutput)
}

fn parse(input: &[u8]) -> Result<Grid> {
    use parsers::*;
    let (input, width) = {
        let line = input
            .lines()
            .next()
            .ok_or(Error::ParseError(ParseError::EmptyInput))?;
        if line[0] != b'#' || line[1] != b'.' || line[2..].iter().any(|&c| c != b'#') {
            return Err(Error::ParseError(ParseError::UnexpectedChar));
        }
        (&input[line.len() + 1..], line.len())
    };

    let cell = any().map_res(|c| match c {
        b'.' => Ok(None),
        b'<' => Ok(Some(Direction::Left)),
        b'>' => Ok(Some(Direction::Right)),
        b'^' => Ok(Some(Direction::Up)),
        b'v' => Ok(Some(Direction::Down)),
        _ => Err(ParseError::TokenDoesNotMatch),
    });
    let grid = token(b'#').then(grid(cell, token(b"#\n#")));
    let final_row = token(b'#').fold(0, |n, _| n + 1).trailed(token(b".#"));
    grid.and(final_row)
        .map_res(|(grid, final_row_len): (Grid, usize)| {
            if grid.width() + 2 != width || final_row_len + 1 + 2 != width {
                Err(ParseError::GridIncompleteRow)
            } else {
                Ok(grid)
            }
        })
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    test_pt!(parse, pt1, EXAMPLE => 18);
    test_pt!(parse, pt2, EXAMPLE => AddOutput([18, 23, 13]));
}
