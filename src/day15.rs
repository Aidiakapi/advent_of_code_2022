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

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Vec2,
    radius: i32,
}

fn try_find_defective<const UPPER: i32>(
    sensors: &[Sensor],
    y: i32,
) -> Option<CombiOutput<[i64; 3]>> {
    let mut x = 0;
    'outer: while x <= UPPER {
        for sensor in sensors {
            let radius = sensor.radius - (sensor.position.y - y).abs();
            if x >= sensor.position.x - radius && x <= sensor.position.x + radius {
                x = sensor.position.x + radius + 1;
                continue 'outer;
            }
        }

        let (x, y) = (x as i64, y as i64);
        return Some(CombiOutput([x, y, x * 4_000_000 + y]));
    }
    None
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

    const CHUNK_SIZE: i32 = 10_000;
    (0..UPPER + 1)
        .step_by(CHUNK_SIZE as usize)
        .par_bridge()
        .find_map_first(|y: i32| {
            let range = y..(y + CHUNK_SIZE).min(UPPER + 1);
            // Filter sensors down to those that can potentially hit this chunk
            let sensors = sensors
                .iter()
                .filter(|sensor| {
                    let start = sensor.position.y - sensor.radius;
                    let end = sensor.position.y + sensor.radius;
                    start < range.end && range.start <= end
                })
                .cloned()
                .collect_vec();
            for y in range {
                if let Some(defective) = try_find_defective::<UPPER>(&sensors, y) {
                    return Some(defective);
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
