use std::collections::hash_map::Entry;

framework::day!(17, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

type Row = u8;

#[rustfmt::skip]
const ROCKS: [(usize, &[Row]); 5] = [
    (3, &[
        0b1111000,
    ]),
    (4, &[
        0b0100000,
        0b1110000,
        0b0100000,
    ]),
    (4, &[
        0b0010000,
        0b0010000,
        0b1110000,
    ]),
    (6, &[
        0b1000000,
        0b1000000,
        0b1000000,
        0b1000000,
    ]),
    (5, &[
        0b1100000,
        0b1100000,
    ]),
];

fn count_empty_rows(tower: &[Row]) -> usize {
    tower.iter().rev().position(|&row| row != 0).unwrap_or(0)
}

fn drop_rock(
    tower: &mut Vec<u8>,
    (max_x, rock): (usize, &[u8]),
    mut moves: impl Iterator<Item = Move>,
) {
    let empty_row_count = count_empty_rows(tower);
    tower.resize(tower.len() + rock.len() + 3 - empty_row_count, 0);
    let mut x = 2_usize;
    let mut y = tower.len() - 1;

    let can_fit = |x, y| {
        for (ry, &row) in rock.iter().enumerate() {
            let moved_row = row >> x;
            let existing_row = &tower[y - ry];
            if moved_row & existing_row != 0 {
                return false;
            }
        }
        true
    };

    loop {
        // Gas pushes the rock
        let mv = moves.next().unwrap();
        let new_x = match mv {
            Move::Left => x.saturating_sub(1),
            Move::Right => (x + 1).min(max_x),
        };
        if new_x != x && can_fit(new_x, y) {
            x = new_x;
        }

        // Gravity pulls the rock down
        if y >= rock.len() && can_fit(x, y - 1) {
            y -= 1;
        } else {
            break;
        }
    }

    // Place rock
    for (ry, &row) in rock.iter().enumerate() {
        let moved_row = row >> x;
        let existing_row = &mut tower[y - ry];
        *existing_row |= moved_row;
    }
}

fn move_iter<'a>(moves: &'a [Move], move_index: &'a mut usize) -> impl Iterator<Item = Move> + 'a {
    std::iter::from_fn(|| {
        let mv = moves[*move_index];
        *move_index = (*move_index + 1) % moves.len();
        Some(mv)
    })
}

fn pt1(moves: &[Move]) -> usize {
    let mut tower = Vec::new();

    let mut moves = moves.iter().cloned().cycle();
    let mut rocks = ROCKS.into_iter().cycle();

    for _ in 0..2022 {
        drop_rock(&mut tower, rocks.next().unwrap(), &mut moves);
    }

    tower.len() - count_empty_rows(&tower)
}

fn pt2(moves: &[Move]) -> u64 {
    const TARGET_DROPPED_COUNT: u64 = 1000000000000;
    // For my particular input, this needs to be at least 30.
    // I've set it to 48, so that it has a wide margin, and
    // such that the hashmap key does not exceed 64 bytes.
    const ROW_HASH_SIZE: usize = 48;
    let mut tower = Vec::new();

    let mut rock_index = 0;
    let mut move_index = 0;

    let mut map = HashMap::new();

    let mut dropped_count = 0u64;
    let additional_height = loop {
        drop_rock(
            &mut tower,
            ROCKS[rock_index],
            move_iter(moves, &mut move_index),
        );
        dropped_count += 1;
        rock_index = (rock_index + 1) % ROCKS.len();
        let hash = (tower.iter().rev().cloned())
            .take(ROW_HASH_SIZE)
            .pad_using(ROW_HASH_SIZE, |_| 0)
            .next_array::<ROW_HASH_SIZE>()
            .unwrap();
        match map.entry((rock_index, move_index, hash)) {
            Entry::Occupied(prev) => {
                let (previous_dropped_count, previous_height) = *prev.get();
                let delta = dropped_count - previous_dropped_count;
                let remainder = TARGET_DROPPED_COUNT - dropped_count;
                let skipped = remainder / delta;
                dropped_count += skipped * delta;
                break skipped * (tower.len() - previous_height) as u64;
            }
            Entry::Vacant(slot) => {
                slot.insert((dropped_count, tower.len()));
            }
        }
    };

    while dropped_count < TARGET_DROPPED_COUNT {
        drop_rock(
            &mut tower,
            ROCKS[rock_index],
            move_iter(moves, &mut move_index),
        );
        dropped_count += 1;
        rock_index = (rock_index + 1) % ROCKS.len();
    }

    (tower.len() - count_empty_rows(&tower)) as u64 + additional_height
}

fn parse(input: &[u8]) -> Result<Vec<Move>> {
    use parsers::*;
    let mv = token((b'<', Move::Left)).or(token((b'>', Move::Right)));
    mv.repeat_into().execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

    test_pt!(parse, pt1, EXAMPLE => 3068);
    test_pt!(parse, pt2, EXAMPLE => 1514285714288);
}
