use aoc2025::read_file;
use aoc2025::parse_lines;

fn check_neighbors(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let r = row as isize;
    let c = col as isize;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut neighbors = 0;

    for (dr, dc) in directions {
        let nr = r + dr;
        let nc = c + dc;

        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            let nr = nr as usize;
            let nc = nc as usize;
            if matrix[nr][nc] == '@' {
                neighbors += 1;
            }
        }
    }

    return neighbors;
}


fn main() {

    let input = read_file("input/day04.txt");
    let lines = parse_lines(&input);

    let matrix: Vec<Vec<char>> = lines
    .iter()
    .map(|line| line.chars().collect())
    .collect();

    let mut available_rolls = 0;

    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            if ch != '@' {
                continue;
            }
            let neighbors = check_neighbors(&matrix, row_idx, col_idx);

            if neighbors < 4 {
                println!("Available roll at ({}, {}) with {} neighbors", row_idx, col_idx, neighbors);
                available_rolls += 1;
            }
        }
    }
    println!("Available rolls: {}", available_rolls);
}