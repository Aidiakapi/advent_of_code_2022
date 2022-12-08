framework::day!(08, parse => pt1, pt2);

type Grid = VecGrid<u8>;

fn iter_cells(grid: &Grid) -> impl Iterator<Item = Vec2<usize>> + '_ {
    (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| (x, y).into()))
}

fn pt1(grid: &Grid) -> usize {
    iter_cells(grid)
        .filter(|&position| {
            let height = grid[position];
            Offset::ORTHOGONAL.into_iter().any(|dir| {
                position
                    .neighbors_along(dir)
                    .take_while_map(|p| grid.get(p))
                    .all(|h| *h < height)
            })
        })
        .count()
}

fn count_trees_seen_in_dir(grid: &Grid, dir: Offset, position: Vec2<usize>) -> usize {
    let height = grid[position];
    let mut seen_count = 0;
    for neighbor in position.neighbors_along(dir) {
        match grid.get(neighbor) {
            Some(&h) => {
                seen_count += 1;
                if h >= height {
                    break;
                }
            }
            None => break,
        }
    }
    seen_count
}

fn pt2(grid: &Grid) -> MulOutput<[usize; 4]> {
    let res = iter_cells(grid)
        .map(|position| -> [usize; 4] {
            Offset::ORTHOGONAL
                .into_iter()
                .map(|dir| count_trees_seen_in_dir(grid, dir, position))
                .collect_array()
                .unwrap()
        })
        .max_by_key(|&v| v.into_iter().product::<usize>())
        .unwrap();
    MulOutput(res)
}

fn parse(input: &[u8]) -> Result<Grid> {
    use parsers::*;
    grid(digit(), token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
30373
25512
65332
33549
35390
";

    test_pt!(parse, pt1, EXAMPLE => 21);
    test_pt!(parse, pt2, EXAMPLE => MulOutput([2, 2, 1, 2]));
}
