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

impl Range {
    fn len(&self) -> u64 {
        self.max - self.min + 1   // inclusive range
    }
}

fn split_by_dash(s: &str) -> Range {
    let parts: Vec<&str> = s.split('-').collect();
    let min = parts[0].parse::<u64>().unwrap();
    let max = parts[1].parse::<u64>().unwrap();
    Range { min, max }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.min);

    let mut merged: Vec<Range> = Vec::new();

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.min <= last.max + 1 {
                if r.max > last.max {
                    last.max = r.max;
                }
            } else {
                merged.push(r);
            }
        } else {
            merged.push(r);
        }
    }
    merged
}

fn main() {

    let input = read_file("input/day05.txt");
    let lines: Vec<String> = parse_lines(&input).into_iter().map(|s| s.to_string()).collect();


    let (database, _) = split_at_empty_line(lines);
    
    let ranges: Vec<Range> = database
        .iter()
        .map(|entry| split_by_dash(entry))
        .collect();

    let merged_ranges = merge_ranges(ranges);

    let fresh_ingredients: u64 = merged_ranges.iter().map(|r| r.len()).sum();

    println!("Number of fresh ingredients: {}", fresh_ingredients);
}