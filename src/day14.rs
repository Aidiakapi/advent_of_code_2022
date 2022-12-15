framework::day!(14, parse => pt1, pt2);
type Path = Vec<Vec2>;
type Vec2 = framework::vecs::Vec2<usize>;
type Grid = VecGrid<Cell>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

fn simulate<const PADDING: usize>(
    paths: &[Path],
    mut process_grid: impl FnMut(&mut Grid),
) -> Result<Grid> {
    let (min, max) = paths.iter().flat_map(|path| path.iter()).fold(
        (Vec2::new(500, 0), Vec2::new(500, 0)),
        |(min, max), &curr| (min.min_comp(curr), max.max_comp(curr)),
    );
    let (min, max) = (
        min - Vec2::new(PADDING, 0),
        max + Vec2::new(PADDING * 2, PADDING),
    );
    let size = max + 1 - min;
    let mut grid = VecGrid::new(size.x, size.y, |_| Cell::Air);
    for (from, to) in paths
        .iter()
        .flat_map(|path| path.iter().map(|&p| p - min).tuple_windows())
    {
        if from.x == to.x {
            let (min, max) = from.y.minmax(to.y);
            for y in min..max + 1 {
                grid[(from.x, y)] = Cell::Rock;
            }
        } else {
            let (min, max) = from.x.minmax(to.x);
            for x in min..max + 1 {
                grid[(x, from.y)] = Cell::Rock;
            }
        }
    }

    process_grid(&mut grid);
    let origin = (Vec2::new(500, 0) - min).to_usize();

    'spawning: loop {
        let mut p = origin;
        'fall: loop {
            for offset in [Offset::Y_POS, Offset::X_NEG_Y_POS, Offset::X_POS_Y_POS] {
                let new_p = if let Some(p) = p.neighbor(offset) {
                    p
                } else {
                    break 'spawning;
                };
                match grid.get(new_p) {
                    Some(Cell::Air) => {
                        p = new_p;
                        continue 'fall;
                    }
                    Some(_) => continue,
                    None => break 'spawning,
                }
            }
            break;
        }
        if let Some(cell) = grid.get_mut(p) {
            if *cell == Cell::Sand {
                break;
            }
            *cell = Cell::Sand;
            continue 'spawning;
        }
        break;
    }

    Ok(grid)
}

fn pt1(paths: &[Path]) -> Result<usize> {
    let grid = simulate::<0>(paths, |_| {})?;
    Ok(grid.cells().iter().filter(|c| **c == Cell::Sand).count())
}

fn pt2(paths: &[Path]) -> Result<usize> {
    let grid = simulate::<2>(paths, |grid| {
        let bottom = grid.height() - 1;
        for x in 0..grid.width() {
            grid[(x, bottom)] = Cell::Rock;
        }
        let right = grid.width() - 1;
        for y in 0..grid.height() {
            grid[(0, y)] = Cell::Rock;
            grid[(right, y)] = Cell::Rock;
        }
    })?;

    let count_additional = |x: usize| {
        let height = (0..grid.height() - 1)
            .rev()
            .take_while(|&y| grid[(x, y)] == Cell::Sand)
            .count();
        height * (height - 1) / 2
    };
    let additional = count_additional(1) + count_additional(grid.width() - 2);
    Ok(grid.cells().iter().filter(|c| **c == Cell::Sand).count() + additional)
}

fn parse(input: &[u8]) -> Result<Vec<Path>> {
    use parsers::*;
    let nr = number::<usize>();
    let point = nr.and(token(b',').then(nr)).map(Vec2::from);
    let path = point.sep_by(token(b" -> "));
    let paths = path.sep_by(token(b'\n'));
    paths.execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    test_pt!(parse, pt1, EXAMPLE => 24);
    test_pt!(parse, pt2, EXAMPLE => 93);
}
