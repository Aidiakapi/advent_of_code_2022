use rayon::prelude::*;
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

fn find_defective<const UPPER: i32>(scans: &[Scan]) -> Result<CombiOutput<[i64; 3]>> {
    let scans = scans
        .iter()
        .map(|s| (s.sensor, s.sensor.manhathan_distance(s.beacon)))
        .collect_vec();
    (0..UPPER + 1)
        .into_par_iter()
        .find_map_any(|x| {
            let mutator = &mut CBuffer::mutator();
            let mut overlaps = CBuffer::new(true);
            scans
                .iter()
                .filter_map(|(p, d)| {
                    let half_width = d - (p.x - x).abs();
                    if half_width < 0 {
                        None
                    } else {
                        Some(p.y - half_width..p.y + half_width + 1)
                    }
                })
                .for_each(|range| {
                    overlaps.set(range, false, mutator);
                });
            overlaps.set(i32::MIN..0, false, mutator);
            overlaps.set(UPPER + 1..i32::MAX, false, mutator);

            for (range, &can_place_beacon) in overlaps.ranges() {
                if can_place_beacon && range.len() == 1 {
                    let y = range.start;
                    return Some(CombiOutput([
                        x as i64,
                        y as i64,
                        x as i64 * 4_000_000 + y as i64,
                    ]));
                }
            }
            None
        })
        .ok_or(Error::NoSolution)
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
