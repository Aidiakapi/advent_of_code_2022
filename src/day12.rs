framework::day!(12, parse => pt1, pt2);

fn get_path_length(
    grid: &VecGrid<Cell>,
    initial: Vec2<usize>,
    mut is_done: impl FnMut(Cell) -> bool,
) -> Option<usize> {
    let mut visited = HashSet::new();
    graph::bfs((initial, 0), |(p, cost), nodes| {
        if !visited.insert(p) {
            return None;
        }
        let cell = grid[p];
        if is_done(cell) {
            return Some(cost);
        }
        let min_elevation = cell.get_elevation().saturating_sub(1);
        let next_cost = cost + 1;
        nodes.extend(
            Offset::ORTHOGONAL
                .into_iter()
                .filter_map(|dir| p.neighbor(dir))
                .filter(|&p| {
                    grid.get(p)
                        .filter(|neighbor| neighbor.get_elevation() >= min_elevation)
                        .is_some()
                })
                .map(|p| (p, next_cost)),
        );
        None
    })
}

fn pt1(grid: &VecGrid<Cell>) -> Result<usize> {
    let (end, _) = grid.iter().find(|(_, c)| matches!(c, Cell::End)).unwrap();
    get_path_length(grid, end, |cell| cell == Cell::Start).ok_or(Error::NoSolution)
}

fn pt2(grid: &VecGrid<Cell>) -> Result<usize> {
    let (end, _) = grid.iter().find(|(_, c)| matches!(c, Cell::End)).unwrap();
    get_path_length(grid, end, |cell| cell.get_elevation() == 0).ok_or(Error::NoSolution)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Start,
    Letter(u8),
    End,
}

impl Cell {
    fn get_elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::Letter(l) => l,
            Cell::End => b'z' - b'a',
        }
    }
}

fn parse(input: &[u8]) -> Result<VecGrid<Cell>> {
    use parsers::*;
    let start = token((b'S', Cell::Start));
    let end = token((b'E', Cell::End));
    let letter = pattern!(b'a'..=b'z').map(|c| Cell::Letter(c - b'a'));
    grid(letter.or(start).or(end), token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    test_pt!(parse, pt1, EXAMPLE => 31);
    test_pt!(parse, pt2, EXAMPLE => 29);
}
