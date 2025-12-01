pub fn read_file(name: &str) -> String {
    std::fs::read_to_string(name).expect("Unable to read file")
}

pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}