use aoc2025::read_file;
use aoc2025::parse_lines;

fn main() {

    let input = read_file("input/day03.txt");
    let lines = parse_lines(&input);
    let mut joltage = 0;

    for line in lines {
        let mut biggest = 0;
        let mut second_biggest = 0;
        for (i, ch) in line.chars().enumerate() {
            let num: i32 = ch.to_digit(10).expect("not a digit") as i32;
            if num > biggest && i < line.chars().count() - 1 {
                second_biggest = 0;
                biggest = num;
                continue;
            }
            if num > second_biggest {
                second_biggest = num;
            }
        }
        joltage += biggest * 10 + second_biggest;
    }
    println!("Total joltage: {}", joltage);
}