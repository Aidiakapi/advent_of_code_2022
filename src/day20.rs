framework::day!(20, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    offset: usize,
    next: usize,
    prev: usize,
}

fn make_list(nrs: &[i64]) -> Vec<Entry> {
    let modulo = nrs.len() as i64 - 1;
    nrs.iter()
        .enumerate()
        .map(|(index, &nr)| Entry {
            offset: ((nr % modulo + modulo) % modulo) as usize,
            prev: (index + nrs.len() - 1) % nrs.len(),
            next: (index + 1) % nrs.len(),
        })
        .collect()
}

fn find_nth_after_index(list: &[Entry], index: usize, n: usize) -> usize {
    let mut curr = index;
    if n * 2 >= list.len() {
        for _ in 0..list.len() - n {
            curr = list[curr].prev;
        }
    } else {
        for _ in 0..n {
            curr = list[curr].next;
        }
    }
    curr
}

fn remove_from_list(list: &mut [Entry], index: usize) {
    let Entry { prev, next, .. } = list[index];
    list[prev].next = next;
    list[next].prev = prev;
}

fn insert_into_list(list: &mut [Entry], index: usize, after: usize) {
    let insertion_point = &mut list[after];
    let next_index = insertion_point.next;
    insertion_point.next = index;
    let entry = &mut list[index];
    entry.prev = after;
    entry.next = next_index;
    list[next_index].prev = index;
}

fn mix_list(list: &mut [Entry]) {
    for index in 0..list.len() {
        let offset = list[index].offset;
        if offset == 0 {
            continue;
        }
        let insertion_index = find_nth_after_index(list, index, offset);
        remove_from_list(list, index);
        insert_into_list(list, index, insertion_index);
    }
}

fn get_grove_number(nrs: &[i64], list: &[Entry]) -> [i64; 3] {
    let start = nrs.iter().position(|&nr| nr == 0).unwrap();
    let mut curr = start;
    let mut res = [0; 3];
    for res in res.iter_mut() {
        curr = find_nth_after_index(list, curr, 1000 % list.len());
        *res = nrs[curr];
    }
    res
}

fn pt1(nrs: &[i64]) -> AddOutput<[i64; 3]> {
    let mut list = make_list(nrs);
    mix_list(&mut list);
    AddOutput(get_grove_number(nrs, &list))
}

fn pt2(nrs: &[i64]) -> i64 {
    let nrs = nrs.iter().map(|&nr| nr * 811_589_153).collect_vec();
    let mut list = make_list(&nrs);
    for _ in 0..10 {
        mix_list(&mut list);
    }
    get_grove_number(&nrs, &list).into_iter().sum()
}

fn parse(input: &[u8]) -> Result<Vec<i64>> {
    use parsers::*;
    number::<i64>().sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
1
2
-3
3
-2
0
4
";

    test_pt!(parse, pt1, EXAMPLE => AddOutput([4, -3, 2]));
    test_pt!(parse, pt2, EXAMPLE => 1623178306);
}
