framework::day!(06, parse => pt1, pt2);

fn pts<const N: usize>(input: &[u8]) -> Result<usize> {
    let mut i = N;
    'outer: while i < input.len() {
        let mut set = 0u32;
        let slice = unsafe { input.get_unchecked(i + 1 - N..i + 1) };
        for (j, &value) in slice.iter().enumerate().rev() {
            let bit = 1u32 << (value - b'a');
            if set | bit == set {
                i += j + 1;
                continue 'outer;
            }
            set |= bit;
        }
        if set.count_ones() as usize == N {
            return Ok(i + 1);
        }
        i += 1;
    }
    return Err(Error::NoSolution);
}

// The following solution always run in O(n), instead of a worst case O(n * w).
// However, in practice, it runs within margin of error for part 1, but
// significantly slower for part 2. This is mostly explained by the high density
// of repeated characters, allowing it to jump ahead in a large step, whilst
// minimizing the overhead of the linear search for duplicates.
//
// The original implementation above, jumps ahead on average 2.00 characters for
// part 1, and 10.45 characters for part 2. This results in reading 90.0% of
// the input length for part 1, and only 42.3% for part 2.
//
//                Part 1   Part 2
// O(n*w) above  1.43 µs  0.64 µs
// O(n)   below  1.40 µs  2.52 µs
//
// fn pts<const N: usize>(input: &[u8]) -> Result<usize> {
//     let mut last_indices = [0; 26];
//     let mut minimum_index = N;
//     for (index, &c) in input.iter().enumerate() {
//         let last_index = unsafe { last_indices.get_unchecked_mut((c - b'a') as usize) };
//         let delta_index = index - *last_index;
//         if delta_index >= N {
//             if index >= minimum_index {
//                 return Ok(index + 1);
//             }
//         } else {
//             minimum_index = minimum_index.max(index + N - delta_index);
//         }
//         *last_index = index;
//     }
//     Err(Error::NoSolution)
// }

fn pt1(input: &[u8]) -> Result<usize> {
    pts::<4>(input)
}

fn pt2(input: &[u8]) -> Result<usize> {
    pts::<14>(input)
}

fn parse(mut input: &[u8]) -> Result<&[u8]> {
    input = input.trim_ascii_end();
    if input.iter().all(|&c| c >= b'a' && c <= b'z') {
        Ok(input)
    } else {
        Err(Error::InvalidInput("invalid characters in input"))
    }
}

tests! {
    test_pt!(parse, pt1,
        b"mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7,
        b"bvwbjplbgvbhsrlpgdmjqwftvncz" => 5,
        b"nppdvjthqldpwncqszvftbrmjlhg" => 6,
        b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10,
        b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11,
    );
    test_pt!(parse, pt2,
        b"mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 19,
        b"bvwbjplbgvbhsrlpgdmjqwftvncz" => 23,
        b"nppdvjthqldpwncqszvftbrmjlhg" => 23,
        b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 29,
        b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 26,
    );
}
