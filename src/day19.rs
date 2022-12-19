use rayon::prelude::*;
use std::collections::BinaryHeap;
framework::day!(19, parse => pt1, pt2);

type Int = u32;
type Vec4 = framework::vecs::Vec4<Int>;
struct Blueprint {
    ore_robot_ore: Int,
    clay_robot_ore: Int,
    obsidian_robot_ore: Int,
    obsidian_robot_clay: Int,
    geode_robot_ore: Int,
    geode_robot_obsidian: Int,
}

#[derive(Debug, Clone)]
struct State {
    time: u32,
    robots: Vec4,
    materials: Vec4,
    minimum_geodes: u32,
    maximum_geodes: u32,
}
util::impl_eq_ord_by!(State, maximum_geodes, minimum_geodes);

fn rev_triangle_number(triangle: u32) -> f64 {
    ((triangle as f64 * 8.0 + 1.0).sqrt() - 1.0) / 2.0
}

fn find_maximum_geodes<const TIME: usize>(bp: &Blueprint) -> Int {
    let mut total_maximum_geodes = 0;
    let make_state = |time: u32, robots: Vec4, materials: Vec4| {
        let remaining_time = (TIME as u32).saturating_sub(time);
        let minimum_geodes = robots.x * remaining_time + materials.x;

        // Calculate the minimum amount of time we have to spend until
        // we have our first piece of clay/obsidian.
        let minimum_till_clay = if robots.z == 0 {
            rev_triangle_number(bp.obsidian_robot_clay).ceil() as u32
        } else {
            0
        };
        let minimum_till_obsidian = if robots.y == 0 {
            rev_triangle_number(bp.geode_robot_obsidian).ceil() as u32
        } else {
            0
        };
        let minimum_time = minimum_till_clay + minimum_till_obsidian;

        // Run a simple simulation on how much obsidian we have, and how we can
        // turn those into geodes, assuming an infinite supply of ore, and
        // creating a new geode robot every timestep.
        // This gives us a well balanced upper bound on how many geodes could
        // be produced.
        let mut potential_geodes = 0;
        let mut potential_geode_robots = 0;
        let mut potential_obsidian = materials.y + robots.y * minimum_time;
        let mut potential_obsidian_robots = robots.y + minimum_till_obsidian;
        for _ in 0..remaining_time.saturating_sub(minimum_time) {
            potential_geodes += potential_geode_robots;
            if potential_obsidian >= bp.geode_robot_obsidian {
                potential_obsidian -= bp.geode_robot_obsidian;
                potential_geode_robots += 1;
            }
            potential_obsidian += potential_obsidian_robots;
            potential_obsidian_robots += 1;
        }

        let maximum_geodes = minimum_geodes + potential_geodes;
        State {
            time,
            robots,
            materials,
            minimum_geodes,
            maximum_geodes,
        }
    };

    let mut queue = BinaryHeap::new();
    queue.push(make_state(0, Vec4::new(0, 0, 0, 1), Vec4::zero()));

    let max_ore_robots = bp
        .ore_robot_ore
        .max(bp.clay_robot_ore)
        .max(bp.obsidian_robot_ore)
        .max(bp.geode_robot_ore);

    while let Some(State {
        time,
        robots,
        materials,
        minimum_geodes,
        maximum_geodes,
    }) = queue.pop()
    {
        if maximum_geodes <= total_maximum_geodes {
            break;
        }
        if minimum_geodes > total_maximum_geodes {
            total_maximum_geodes = minimum_geodes;
        }

        // Make geode robot
        if robots.y != 0 {
            let time_until_enough_ore =
                (bp.geode_robot_ore.saturating_sub(materials.w) + robots.w - 1) / robots.w;
            let time_until_enough_obsidian =
                (bp.geode_robot_obsidian.saturating_sub(materials.y) + robots.y - 1) / robots.y;
            let time_taken = time_until_enough_ore.max(time_until_enough_obsidian) + 1;
            let produced_at = time_taken + time;
            if produced_at < TIME as u32 {
                let cost = Vec4::new(0, bp.geode_robot_obsidian, 0, bp.geode_robot_ore);
                let materials = materials + robots * time_taken - cost;
                let robots = robots + Vec4::new(1, 0, 0, 0);
                queue.push(make_state(produced_at, robots, materials));
            }
        }

        // Make obsidian robot
        if robots.z != 0 && robots.y < bp.geode_robot_obsidian {
            let time_until_enough_ore =
                (bp.obsidian_robot_ore.saturating_sub(materials.w) + robots.w - 1) / robots.w;
            let time_until_enough_clay =
                (bp.obsidian_robot_clay.saturating_sub(materials.z) + robots.z - 1) / robots.z;
            let time_taken = time_until_enough_ore.max(time_until_enough_clay) + 1;
            let produced_at = time_taken + time;
            if produced_at < TIME as u32 {
                let cost = Vec4::new(0, 0, bp.obsidian_robot_clay, bp.obsidian_robot_ore);
                let materials = materials + robots * time_taken - cost;
                let robots = robots + Vec4::new(0, 1, 0, 0);
                queue.push(make_state(produced_at, robots, materials));
            }
        }

        // Make clay robot
        if robots.z < bp.obsidian_robot_clay {
            let time_until_enough_ore =
                (bp.clay_robot_ore.saturating_sub(materials.w) + robots.w - 1) / robots.w;
            let time_taken = time_until_enough_ore + 1;
            let produced_at = time_taken + time;
            if produced_at < TIME as u32 {
                let cost = Vec4::new(0, 0, 0, bp.clay_robot_ore);
                let materials = materials + robots * time_taken - cost;
                let robots = robots + Vec4::new(0, 0, 1, 0);
                queue.push(make_state(produced_at, robots, materials));
            }
        }

        // Make ore robot
        if robots.x < max_ore_robots {
            let time_until_enough_ore =
                (bp.ore_robot_ore.saturating_sub(materials.w) + robots.w - 1) / robots.w;
            let time_taken = time_until_enough_ore + 1;
            let produced_at = time_taken + time;
            if produced_at < TIME as u32 {
                let cost = Vec4::new(0, 0, 0, bp.ore_robot_ore);
                let materials = materials + robots * time_taken - cost;
                let robots = robots + Vec4::new(0, 0, 0, 1);
                queue.push(make_state(produced_at, robots, materials));
            }
        }
    }
    total_maximum_geodes
}

fn pt1(blueprints: &[Blueprint]) -> Int {
    blueprints
        .into_par_iter()
        .enumerate()
        .map(|(index, bp)| (index, find_maximum_geodes::<24>(bp)))
        .map(|(index, maximum)| (index as Int + 1) * maximum)
        .sum::<Int>()
}

fn pt2(blueprints: &[Blueprint]) -> MulOutput<Vec<Int>> {
    MulOutput(
        blueprints[..3.min(blueprints.len())]
            .into_par_iter()
            .map(find_maximum_geodes::<32>)
            .collect(),
    )
}

fn parse(input: &[u8]) -> Result<Vec<Blueprint>> {
    use parsers::*;
    let nr = number::<Int>();
    #[rustfmt::skip]
    let costs = (token(b": Each ore robot costs ").then(nr))
        .and(token(b" ore. Each clay robot costs ").then(nr))
        .and(token(b" ore. Each obsidian robot costs ").then(nr))
        .and(token(b" ore and ").then(nr))
        .and(token(b" clay. Each geode robot costs ").then(nr))
        .and(token(b" ore and ").then(nr))
        .trailed(token(b" obsidian."))
        .map(|(((((
                ore_robot_ore,
                clay_robot_ore),
                obsidian_robot_ore),
                obsidian_robot_clay),
                geode_robot_ore),
                geode_robot_obsidian)|
            Blueprint {
                ore_robot_ore,
                clay_robot_ore,
                obsidian_robot_ore,
                obsidian_robot_clay,
                geode_robot_ore,
                geode_robot_obsidian,
            },
        );
    let header = token(b"Blueprint ").then(number::<usize>());
    header
        .and(costs)
        .sep_by(token(b'\n'))
        .map_res(|blueprints: Vec<_>| {
            for (index, &(found_index, _)) in blueprints.iter().enumerate() {
                if index + 1 != found_index {
                    return Err(ParseError::Custom("invalid index"));
                }
            }
            Ok(blueprints.into_iter().map(|(_, costs)| costs).collect_vec())
        })
        .execute(input)
}

tests! {
    const EXAMPLE1: &[u8] = b"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    const EXAMPLE2: &[u8] = b"Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    test_pt!(parse, pt1,
        EXAMPLE1 => 9,
        EXAMPLE2 => 12);
    test_pt!(parse, pt2,
        EXAMPLE1 => MulOutput(vec![56]),
        EXAMPLE2 => MulOutput(vec![62]));
}
