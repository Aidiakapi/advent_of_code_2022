framework::day!(07, parse => pt1, pt2);

fn visit_all_directories(commands: &[Command], mut f: impl FnMut(u32)) {
    let mut size_stack = Vec::<u32>::new();
    size_stack.push(0);
    fn pop(size_stack: &mut Vec<u32>, f: &mut impl FnMut(u32)) -> bool {
        let child_size = size_stack.pop().unwrap();
        f(child_size);
        if let Some(parent_size) = size_stack.last_mut() {
            *parent_size += child_size;
            true
        } else {
            false
        }
    }
    for command in commands {
        match command {
            Command::OpenChild => {
                size_stack.push(0);
            }
            Command::CloseChild => {
                pop(&mut size_stack, &mut f);
            }
            Command::Listing(file_size) => {
                *size_stack.last_mut().unwrap() += file_size;
            }
        }
    }
    while pop(&mut size_stack, &mut f) {}
}

fn pt1(commands: &[Command]) -> u32 {
    const THRESHOLD: u32 = 100_000;
    let mut result = 0;
    visit_all_directories(commands, |size| {
        if size <= THRESHOLD {
            result += size;
        }
    });
    result
}

fn pt2(commands: &[Command]) -> u32 {
    let mut sizes = Vec::new();
    visit_all_directories(commands, |size| sizes.push(size));
    sizes.sort();
    const DISK_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;
    let remaining_space = DISK_SPACE - *sizes.last().unwrap();
    let space_to_free = REQUIRED_SPACE - remaining_space;
    let index = sizes.binary_search(&space_to_free).unwrap_or_else(|i| i);
    sizes[index]
}

#[derive(Debug, Clone, Copy)]
enum Command {
    OpenChild,
    Listing(u32),
    CloseChild,
}

fn parse(input: &[u8]) -> Result<Vec<Command>> {
    use parsers::*;
    let name = pattern!(b'a'..=b'z' | b'.').repeat();
    let file_size = token((b"dir", 0)).or(number::<u32>());
    let listing_entry = file_size.trailed(token(b' ').then(name).then(token(b'\n')));
    let listing = token(b"$ ls\n").then(listing_entry.fold(0, |a, v| a + v));
    let command = token((b"$ cd ..\n", Command::CloseChild))
        .or(token((b"$ cd ", Command::OpenChild)).trailed(name.then(token(b'\n'))))
        .or(listing.map(Command::Listing));
    token(b"$ cd /\n")
        .then(command.repeat_into())
        .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    test_pt!(parse, pt1, EXAMPLE => 95437);
    test_pt!(parse, pt2, EXAMPLE => 24933642);
}
