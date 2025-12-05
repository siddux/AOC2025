use aoc2025::read_file;
use aoc2025::parse_lines;

fn split_at_empty_line(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    if let Some(idx) = lines.iter().position(|l| l.is_empty()) {
        let left = lines[..idx].to_vec();
        let right = lines[idx + 1..].to_vec();
        (left, right)
    } else {
        (lines, Vec::new())
    }
}

struct Range {
    min: u64,
    max: u64,
}

fn split_by_dash(s: &str) -> Range {
    let parts: Vec<&str> = s.split('-').collect();
    let min = parts[0].parse::<u64>().unwrap();
    let max = parts[1].parse::<u64>().unwrap();
    Range { min, max }
}

fn main() {

    let input = read_file("input/day05.txt");
    let lines: Vec<String> = parse_lines(&input).into_iter().map(|s| s.to_string()).collect();


    let (database, ingredients) = split_at_empty_line(lines);
    let mut fresh_ingredients = 0;

    for ingredient in &ingredients {
        for entry in &database {
            let Range {min, max}  = split_by_dash(&entry);
            if ingredient.parse::<u64>().unwrap() >= min && ingredient.parse::<u64>().unwrap() <= max {
                println!("Ingredient {} matches range {}-{}", ingredient, min, max);
                fresh_ingredients += 1;
                break;
            }
        }
    }
    println!("Number of fresh ingredients: {}", fresh_ingredients);
}