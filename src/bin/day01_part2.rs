use aoc2025::read_file;
use aoc2025::parse_lines;


struct Rotation {
    direction: String,
    distance: u64,
}

fn parse_rotation(input: &str) -> Rotation {
    let direction = input.chars().next().unwrap().to_string();
    let distance = input[1..].parse::<u64>().unwrap();
    Rotation { direction, distance }
}

fn main() {

    const MAX_POSITION: u64 = 99;
    let mut position: i64 = 50;
    let mut position_counter: u64 = 0;

    let input = read_file("input/day01.txt");
    let lines = parse_lines(&input);
    for line in lines {
        let Rotation { direction, distance } = parse_rotation(line);

        let step: i64 = if direction == "L" { -1 } else { 1 };

        for _ in 0..distance {
            position += step;

            if position > MAX_POSITION as i64 {
                position = 0;
            } else if position < 0 {
                position = MAX_POSITION as i64;
            }

            if position == 0 {
                position_counter += 1;
            }
        }
    }

    print!("Number of times at position 0: {}", position_counter);
}