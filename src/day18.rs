framework::day!(18, parse => pt1, pt2);
type Vec3 = framework::vecs::Vec3<i32>;

fn get_bounds(cubes: &[Vec3]) -> (Vec3, Vec3) {
    cubes.iter().cloned().fold(
        (Vec3::from(i32::MAX), Vec3::from(i32::MIN)),
        |(min, max), point| (min.min_comp(point), max.max_comp(point)),
    )
}

fn neighbors(cube: Vec3) -> impl Iterator<Item = Vec3> + Clone + std::iter::FusedIterator {
    [-1, 1].into_iter().flat_map(move |offset| {
        [
            Vec3::new(cube.x + offset, cube.y, cube.z),
            Vec3::new(cube.x, cube.y + offset, cube.z),
            Vec3::new(cube.x, cube.y, cube.z + offset),
        ]
    })
}

fn pt1(cubes: &[Vec3]) -> usize {
    let (min, max) = get_bounds(cubes);
    let (min, max) = (min - 1, max + 1);
    let size = max + 1 - min;
    let mut grid = vec![false; (size.x * size.y * size.z) as usize];
    let index = |p: Vec3| -> usize {
        let p = p - min;
        ((p.x * size.y + p.y) * size.z + p.z) as usize
    };

    for &cube in cubes {
        grid[index(cube)] = true;
    }
    (cubes.iter().cloned())
        .flat_map(neighbors)
        .filter(|&p| !grid[index(p)])
        .count()
}

fn pt2(cubes: &[Vec3]) -> usize {
    let (min, max) = get_bounds(cubes);
    let (min, max) = (min - 1, max + 1);
    let size = max + 1 - min;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Exterior,
        Cube,
        Pocket,
    }
    let mut grid = vec![State::Pocket; (size.x * size.y * size.z) as usize];
    let index = |p: Vec3| -> usize {
        let p = p - min;
        ((p.x * size.y + p.y) * size.z + p.z) as usize
    };

    for &cube in cubes {
        grid[index(cube)] = State::Cube;
    }

    graph::dfs(Vec3::zero(), |point, next| -> Option<!> {
        let state = match grid.get_mut(index(point)) {
            Some(state @ &mut State::Pocket) => state,
            _ => return None,
        };
        *state = State::Exterior;
        next.extend(neighbors(point));
        None
    });

    (cubes.iter().cloned())
        .flat_map(neighbors)
        .filter(|&p| matches!(grid.get(index(p)), Some(&State::Exterior)))
        .count()
}

fn parse(input: &[u8]) -> Result<Vec<Vec3>> {
    use parsers::*;
    let nr = number::<i32>();
    let comma_nr = token(b',').then(nr);
    let cube = nr.and(comma_nr).and(comma_nr);
    let cube = cube.map(|((x, y), z)| Vec3::new(x, y, z));
    cube.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    test_pt!(parse, pt1, b"1,1,1\n2,1,1" => 10, EXAMPLE => 64);
    test_pt!(parse, pt2, EXAMPLE => 58);
}
