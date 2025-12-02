use aoc2025::read_file;


fn parse_comma_separated(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.trim().to_string()).collect()
}

struct ProtductRange {
    first_id: u64,
    last_id: u64,
}

fn parse_ids_by_dash(input: &str) -> ProtductRange {
    input
        .split('-')
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .as_slice()
        .try_into()
        .map(|arr: [u64; 2]| ProtductRange {
            first_id: arr[0],
            last_id: arr[1],
        })
        .unwrap()
}

fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let n = s.len();

    if n <=1 {
        return false;
    }

    for sub_len in 1..=n / 2 {
        if n % sub_len != 0 {
            continue;
        }

        let pattern = &s[0..sub_len];
        let mut ok = true;

        let mut i = sub_len;
        while i < n {
            if &s[i..i + sub_len] != pattern {
                ok = false;
                break;
            }
            i += sub_len;
        }

        if ok {
            return true;
        }
    }
    false
}

fn main() {

    let mut result: u64 = 0;

    let input = read_file("input/day02.txt");
    let list = parse_comma_separated(&input);
    for item in list {
        let ProtductRange { first_id, last_id } = parse_ids_by_dash(item.as_str());

        for id in first_id..=last_id {
            if is_invalid_id(id) {
                result += id;
                println!("Invalid ID found: {}", id);
            }
        }
    }
    println!("Sum of invalid IDs: {}", result);
}