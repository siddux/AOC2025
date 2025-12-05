use aoc2025::read_file;
use aoc2025::parse_lines;

fn main() {

    let input = read_file("input/day03.txt");
    let lines = parse_lines(&input);
    let mut joltage: usize = 0;

    for line in lines {
        let mut arr = [0; 12];
        let n = line.chars().count();
        for (i, ch) in line.chars().enumerate() {
            let num: usize = ch.to_digit(10).expect("not a digit") as usize;
            for pos in 0..arr.len() {
                if num > arr[pos] && i <= n - (12 - pos) {
                    arr[pos] = num;
                    arr[pos+1 ..].fill(0);
                    break;
                }
            }
        }
        let mut value = 0;
        for d in arr {
            value = value * 10 + d;
        }
        println!("The biggest number is: {}", value);
        joltage += value;
    }
    println!("Total joltage: {}", joltage);
}