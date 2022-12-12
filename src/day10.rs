framework::day!(10, parse => pt1, pt2);

type Val = i32;

fn for_each_cycle(instructions: &[Instruction], mut f: impl FnMut(Val, Val)) {
    let mut cycle = 1;
    let mut register_x = 1;
    let mut instructions = instructions.iter();
    let mut is_adding = None;
    loop {
        f(cycle, register_x);
        if let Some(value) = is_adding {
            register_x += value;
            is_adding = None;
        } else {
            match instructions.next() {
                Some(Instruction::Noop) => {}
                Some(Instruction::AddX(value)) => is_adding = Some(value),
                None => break,
            }
        }
        cycle += 1;
    }
}

fn pt1(instructions: &[Instruction]) -> Val {
    let mut result = 0;
    for_each_cycle(instructions, |cycle, register_x| {
        if (cycle + 20) % 40 == 0 {
            result += cycle * register_x;
        }
    });
    result
}

fn pt2_pixels(instructions: &[Instruction]) -> Result<Vec<bool>> {
    let mut pixels = Vec::new();
    for_each_cycle(instructions, |cycle, register_x| {
        if cycle > 240 {
            return;
        }
        let pixel_index = (cycle - 1) % 40;
        let sprite_index = register_x - 1;
        let is_lit = (sprite_index..sprite_index + 3).contains(&pixel_index);
        pixels.push(is_lit);
    });
    if pixels.len() == 240 {
        Ok(pixels)
    } else {
        Err(Error::InvalidInput("not enough pixels"))
    }
}

fn pt2(instructions: &[Instruction]) -> Result<AString> {
    let pixels = pt2_pixels(instructions)?;
    ocr::recognize_n::<8>(|x, y| pixels[y * 40 + x])
        .map(|o| o.into())
        .ok_or(Error::InvalidInput("failed to OCR"))
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX(Val),
}

fn parse(input: &[u8]) -> Result<Vec<Instruction>> {
    use parsers::*;
    let noop = token((b"noop", Instruction::Noop));
    let addx = token(b"addx ").then(number::<Val>()).map(Instruction::AddX);
    let instruction = noop.or(addx);
    instruction.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    test_pt!(parse, pt1, EXAMPLE => 13140);
    test_pt!(parse, pt2, |input| { super::pt2_pixels(&input) }, EXAMPLE =>
    {
        b"##..##..##..##..##..##..##..##..##..##..\
          ###...###...###...###...###...###...###.\
          ####....####....####....####....####....\
          #####.....#####.....#####.....#####.....\
          ######......######......######......####\
          #######.......#######.......#######....."
    }.iter().map(|&c| c == b'#').collect::<Vec<bool>>()
    );
}
