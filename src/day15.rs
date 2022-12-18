framework::day!(15, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<i32>;

fn count_points_without_beacons<const Y: i32>(scans: &[Scan]) -> usize {
    let mutator = &mut CBuffer::mutator();
    let mut blocked = CBuffer::new(false);
    let mut beacon_set = HashSet::new();
    for scan in scans {
        let distance = scan.sensor.manhathan_distance(scan.beacon);
        let delta_y = (scan.sensor.y - Y).abs();
        let half_width = distance - delta_y;
        blocked.set(
            scan.sensor.x - half_width..scan.sensor.x + half_width + 1,
            true,
            mutator,
        );
        if scan.beacon.y == Y {
            beacon_set.insert(scan.beacon.x);
        }
    }

    blocked.count_values(&true).unwrap() as usize - beacon_set.len()
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Vec2,
    radius: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Diagonal {
    x_intercept: i32,
    from: i32,
    to: i32,
}

impl Diagonal {
    fn fwd_from_points(a: Vec2, b: Vec2) -> Diagonal {
        let x_intercept = a.x + a.y;
        assert_eq!(x_intercept, b.x + b.y);
        let (from, to) = a.x.minmax(b.x);
        Diagonal {
            x_intercept,
            from,
            to,
        }
    }

    fn bwd_from_points(a: Vec2, b: Vec2) -> Diagonal {
        let x_intercept = a.y - a.x;
        assert_eq!(x_intercept, b.y - b.x);
        let (from, to) = a.x.minmax(b.x);
        Diagonal {
            x_intercept,
            from,
            to,
        }
    }

    fn intersect(fwd: &Diagonal, bwd: &Diagonal) -> Option<Vec2> {
        let point = Vec2::new(-bwd.x_intercept, bwd.x_intercept) + fwd.x_intercept;
        if point % 2 != Vec2::zero() {
            return None;
        }
        let point = point / 2;
        if (fwd.from..=fwd.to).contains(&point.x) && (bwd.from..=bwd.to).contains(&point.x) {
            Some(point)
        } else {
            None
        }
    }
}

fn find_defective<const UPPER: i32>(scans: &[Scan]) -> Result<CombiOutput<[i64; 3]>> {
    let sensors = scans
        .iter()
        .map(|scan| Sensor {
            position: scan.sensor,
            radius: scan.sensor.manhathan_distance(scan.beacon),
        })
        .sorted_by(|a, b| b.radius.cmp(&a.radius))
        .collect_vec();

    // Forward / diagonals
    let mut diagonals_fwd = Vec::with_capacity(sensors.len() * 2);
    // Backward \ diagonals
    let mut diagonals_bwd = Vec::with_capacity(sensors.len() * 2);

    for sensor in &sensors {
        let edge = sensor.radius + 1;
        let lft = sensor.position - Vec2::new(edge, 0);
        let rgt = sensor.position + Vec2::new(edge, 0);
        let top = sensor.position - Vec2::new(0, edge);
        let bot = sensor.position + Vec2::new(0, edge);
        diagonals_fwd.push(Diagonal::fwd_from_points(bot, rgt));
        diagonals_fwd.push(Diagonal::fwd_from_points(top, lft));
        diagonals_bwd.push(Diagonal::bwd_from_points(bot, lft));
        diagonals_bwd.push(Diagonal::bwd_from_points(top, rgt));
    }

    let point = diagonals_fwd
        .iter()
        .filter_map(|fwd| {
            for bwd in &diagonals_bwd {
                if let Some(point) = Diagonal::intersect(fwd, bwd) &&
                point.x >= 0 && point.y >= 0 &&
                point.x <= UPPER && point.y <= UPPER &&
                sensors
                    .iter()
                    .all(|s| s.position.manhathan_distance(point) > s.radius) {
                return Some(point);
            }
            }
            None
        })
        .next()
        .ok_or(Error::NoSolution)?
        .to_i64();
    Ok(CombiOutput([
        point.x,
        point.y,
        point.x * 4_000_000 + point.y,
    ]))
}

fn pt1(scans: &[Scan]) -> usize {
    count_points_without_beacons::<2_000_000>(scans)
}

fn pt2(scans: &[Scan]) -> Result<CombiOutput<[i64; 3]>> {
    find_defective::<4_000_000>(scans)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Scan {
    sensor: Vec2,
    beacon: Vec2,
}

fn parse(input: &[u8]) -> Result<Vec<Scan>> {
    use parsers::*;
    let nr = number::<i32>();
    let vec = token(b"x=")
        .then(nr)
        .and(token(b", y=").then(nr))
        .map(Vec2::from);
    let sensor = token(b"Sensor at ").then(vec);
    let beacon = token(b": closest beacon is at ").then(vec);
    let scan = (sensor.and(beacon)).map(|(sensor, beacon)| Scan { sensor, beacon });
    scan.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    test_pt!(parse, pt1, |scans| { super::count_points_without_beacons::<10>(&scans) },
        EXAMPLE => 26
    );
    test_pt!(parse, pt2, |scans| { super::find_defective::<20>(&scans) },
        EXAMPLE => CombiOutput([14, 11, 56000011])
    );
}
