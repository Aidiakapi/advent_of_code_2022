use framework::util::init_array;
use std::collections::BinaryHeap;
framework::day!(16, parse => pt1, pt2);

#[derive(Debug, Clone, Default)]
struct PathCache {
    cache: HashMap<(usize, usize), u32>,
}

impl PathCache {
    fn get_shortest_nonzero_path_from_cache(&self) -> u32 {
        *self.cache.values().filter(|&&len| len != 0).min().unwrap()
    }

    fn get_cost(&mut self, valves: &[Valve], from: usize, to: usize) -> u32 {
        let (from, to) = from.minmax(to);
        *self.cache.entry((from, to)).or_insert_with(|| {
            graph::bfs((from, 0), |(node, cost), next| {
                if node == to {
                    return Some(cost);
                }
                next.extend(valves[node].connections.iter().map(|&p| (p, cost + 1)));
                None
            })
            .unwrap()
        })
    }
}

fn pop_lsb(mask: &mut u64) -> Option<usize> {
    if *mask == 0 {
        return None;
    }
    let index = mask.trailing_zeros();
    *mask &= !(1 << index);
    Some(index as usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Pending,
    Moving(usize, u32),
    Idle,
}

#[derive(Debug, Clone)]
struct Entry<const UNITS: usize> {
    positions: [usize; UNITS],
    targets: [Target; UNITS],
    remainder: u64,
    remaining_flow_rate: u32,
    vented: u32,
    time: u32,
    potential: u32,
}

impl<const UNITS: usize> Entry<UNITS> {
    fn potential_vented(&self) -> u32 {
        self.vented + self.potential
    }
}
impl<const UNITS: usize> PartialEq for Entry<UNITS> {
    fn eq(&self, other: &Self) -> bool {
        self.potential_vented() == other.potential_vented() && self.time == other.time
    }
}
impl<const UNITS: usize> Eq for Entry<UNITS> {}
impl<const UNITS: usize> PartialOrd for Entry<UNITS> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const UNITS: usize> Ord for Entry<UNITS> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential_vented()
            .cmp(&other.potential_vented())
            .then_with(|| self.time.cmp(&other.time))
    }
}

fn pts<const UNITS: usize, const TIME: usize, const MIN_PATH_LEN: u32>(
    input: &(usize, Vec<Valve>),
) -> u32 {
    let &(initial_position, ref valves) = input;
    let priorities = valves
        .iter()
        .enumerate()
        .filter(|(_, valve)| valve.flow_rate != 0)
        .sorted_by(|(_, a), (_, b)| b.flow_rate.cmp(&a.flow_rate))
        .collect_vec();
    assert!(priorities.len() <= 64);

    // Instead of pathfinding through the initial graph, we can build a weighted
    // graph, which allows us to have a much better heuristic.
    let mut paths = PathCache::default();

    // Calculates an upper bound on how much can be vented, by assuming that the
    // distance between any two valves in the tunnel network is MIN_PATH_LEN.
    // Because all travel is constant time, the optimal strategy is to open the
    // highest flow-rate valves first. The result is that opening a "bad" valve
    // early, will result in tons of loss of potential, for minimal gain in
    // actual vented. Whereas opening a "good" valve early, will result in a
    // slightly greater loss in potential (it can't be opened multiple times),
    // but on the flip side, it adds a much higher amount of actually vented.
    let calculate_potential = |mut time: u32, targets: &[Target; UNITS], mut remainder: u64| {
        let mut time_until_opening: [u32; UNITS] = init_array(|i| {
            Ok::<_, !>(match targets[i] {
                Target::Pending => 2,
                Target::Moving(_, time_until_pathing) => time_until_pathing + 1 + MIN_PATH_LEN,
                Target::Idle => TIME as u32 + 1,
            })
        })
        .unwrap();
        let mut flow_rate = 0;
        let mut potential = 0;
        loop {
            if time >= TIME as u32 {
                return potential;
            }
            for time_until_opening in time_until_opening.iter_mut() {
                *time_until_opening -= 1;
                if *time_until_opening == 0 {
                    *time_until_opening = MIN_PATH_LEN;
                    if let Some(bit) = pop_lsb(&mut remainder) {
                        flow_rate += priorities[bit].1.flow_rate;
                    }
                }
            }
            time += 1;
            potential += flow_rate;
        }
    };

    let remainder = (1 << priorities.len()) - 1;
    let flow_rate_sum = priorities.iter().map(|(_, v)| v.flow_rate).sum();
    let mut queue = BinaryHeap::new();
    queue.push(Entry {
        positions: [initial_position; UNITS],
        targets: [Target::Pending; UNITS],
        remainder,
        remaining_flow_rate: flow_rate_sum,
        vented: 0,
        time: 1,
        potential: calculate_potential(1, &[Target::Pending; UNITS], remainder),
    });

    let mut max_vented = 0;
    'outer: while let Some(mut entry) = queue.pop() {
        if entry.potential_vented() <= max_vented {
            break;
        }

        // If there are any units that don't have a target, select one on them,
        // and continue onwards with those.
        for (i, target) in entry.targets.iter_mut().enumerate() {
            if !matches!(target, Target::Pending) {
                continue;
            }

            let mut remainder = entry.remainder;
            while let Some(bit) = pop_lsb(&mut remainder) {
                let (valve_index, valve) = priorities[bit];
                let cost = paths.get_cost(valves, entry.positions[i], valve_index) + 1;
                let time_till_vented = entry.time + cost;
                if time_till_vented > TIME as u32 {
                    continue;
                }

                let additional_vented = (TIME as u32 + 1 - time_till_vented) * valve.flow_rate;
                let vented = entry.vented + additional_vented;
                if vented > max_vented {
                    max_vented = vented;
                }
                let remainder = entry.remainder & !(1 << bit);
                let remaining_flow_rate = entry.remaining_flow_rate - valve.flow_rate;
                let mut targets = entry.targets;
                targets[i] = Target::Moving(valve_index, cost);
                queue.push(Entry {
                    positions: entry.positions,
                    targets,
                    remainder,
                    remaining_flow_rate,
                    time: entry.time,
                    vented,
                    potential: calculate_potential(entry.time, &targets, remainder),
                });
            }

            let mut targets = entry.targets;
            targets[i] = Target::Idle;
            queue.push(Entry { targets, ..entry });

            continue 'outer;
        }

        // At this point, all units have a target, and we advance in time
        for i in 0..UNITS {
            if let Target::Moving(goal, remainder) = &mut entry.targets[i] {
                *remainder -= 1;
                if *remainder == 0 {
                    entry.positions[i] = *goal;
                    entry.targets[i] = Target::Pending;
                }
            }
        }

        entry.time += 1;
        entry.potential = calculate_potential(entry.time, &entry.targets, entry.remainder);
        queue.push(entry);
    }

    // Generally, and on my input, this optimization gives over a 2x, and is
    // fine. However, this is a fallback to detect a case where it is not a
    // proper assumption on anyone's input, and it'll simply re-run the solution
    // without performing it.
    if MIN_PATH_LEN > paths.get_shortest_nonzero_path_from_cache() {
        return pts::<UNITS, TIME, 1>(input);
    }

    max_vented
}

fn pt1(input: &(usize, Vec<Valve>)) -> u32 {
    pts::<1, 30, 2>(input)
}

fn pt2(input: &(usize, Vec<Valve>)) -> u32 {
    pts::<2, 26, 2>(input)
}

struct Valve {
    flow_rate: u32,
    connections: Vec<usize>,
}

fn parse(input: &[u8]) -> Result<(usize, Vec<Valve>)> {
    use parsers::*;
    let letter = pattern!(b'A'..=b'Z').map(|l| l - b'A');
    let name = letter.and(letter).map(|(a, b)| a as u16 * 26 + b as u16);
    let connections = name.sep_by::<_, Vec<_>>(token(b", "));
    let valve_token = token(b"; tunnels lead to valves ").or(token(b"; tunnel leads to valve "));
    let descriptor = (token(b"Valve ").then(name))
        .and(token(b" has flow rate=").then(number::<u32>()))
        .and(valve_token.then(connections));
    let descriptors: Vec<_> = descriptor.sep_by(token(b'\n')).execute(input)?;
    let mut name_to_index = HashMap::new();
    for (i, ((name, _), _)) in descriptors.iter().enumerate() {
        assert!(name_to_index.insert(*name, i).is_none());
    }
    let name_to_index = |name| name_to_index[&name];
    let valves = descriptors
        .into_iter()
        .map(|((_, flow_rate), connections)| {
            let connections = connections.into_iter().map(name_to_index).collect();
            Valve {
                flow_rate,
                connections,
            }
        })
        .collect();
    let initial_index = name_to_index(0);
    Ok((initial_index, valves))
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    test_pt!(parse, pt1, EXAMPLE => 1651);
    test_pt!(parse, pt2, EXAMPLE => 1707);
}

// Initial solution:
// fn pt1(&(initial_position, ref valves): &(usize, Vec<Valve>)) -> u32 {
//     // Depth-first-search the valves, where each time we path to the nodes with
//     // the highest flow-rate first.
//     //
//     // At each step, maintain the maximum potential. Ideally, this would be the
//     // total amount of venting we could still do, if we opened one valve every
//     // other timestep. However, since overestimating is ok, it's pretending that
//     // all remaining valves were opened 2 time-steps from the present.
//     //
//     // Once the maximum potential is less than any currently visited path, the
//     // search terminates.
//
//     let priorities = valves
//         .iter()
//         .enumerate()
//         .filter(|(_, valve)| valve.flow_rate != 0)
//         .sorted_by(|(_, a), (_, b)| b.flow_rate.cmp(&a.flow_rate))
//         .collect_vec();
//
//     assert!(valves[initial_position].flow_rate == 0);
//     assert!(priorities.len() <= 64);
//
//     let mut paths = PathCache::default();
//     let flow_rate_sum: u32 = priorities.iter().map(|(_, v)| v.flow_rate).sum();
//     struct Step {
//         /// Valve index it's currently at.
//         position: usize,
//         /// Mask of priority indices for valves which aren't opened yet.
//         remainder: u64,
//         /// Mask of priority indices that have yet to be pushed onto the stack.
//         pending: u64,
//         /// Total amount of pressure vented up until visiting this node.
//         vented: u32,
//         /// The 1-based time in minutes since the start.
//         time: u32,
//         /// The sum of the flow-rate of all un-opened valves.
//         remaining_flow_rate: u32,
//     }
//
//     let remainder = (1 << priorities.len()) - 1;
//     let mut stack = vec![Step {
//         position: initial_position,
//         remainder,
//         pending: remainder,
//         vented: 0,
//         time: 1,
//         remaining_flow_rate: flow_rate_sum,
//     }];
//
//     let mut max_vented = 0;
//     while let Some(step) = stack.last_mut() {
//         let max_potential = step.remaining_flow_rate * (30 - step.time);
//         if step.vented + max_potential <= max_vented {
//             stack.pop();
//             continue;
//         }
//         let child = match pop_lsb(&mut step.pending) {
//             Some(child) => child,
//             None => {
//                 stack.pop();
//                 continue;
//             }
//         };
//         let (valve_index, valve) = priorities[child];
//
//         let time = step.time + paths.get_cost(valves, step.position, valve_index) + 1;
//         if time > 30 {
//             continue;
//         }
//
//         let child_vented = (31 - time) * valve.flow_rate;
//         let vented = step.vented + child_vented;
//         if vented > max_vented {
//             max_vented = vented;
//         }
//
//         let remainder = step.remainder & !(1 << child);
//         let remaining_flow_rate = step.remaining_flow_rate - valve.flow_rate;
//         stack.push(Step {
//             position: valve_index,
//             remainder,
//             pending: remainder,
//             vented,
//             time,
//             remaining_flow_rate,
//         });
//     }
//
//     max_vented
// }
