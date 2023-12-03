mod tests;

pub fn parse_line(line: String) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for c in line.chars() {
        if !c.is_ascii_digit() {
            continue;
        }
        if first == 0 {
            first = c.to_digit(10).unwrap_or_default();
        }
        last = c.to_digit(10).unwrap_or_default();
    }
    return 10 * first + last;
}

pub fn parse_content(content: String) -> i32 {
    let numbers: Vec<i32> = content.lines().into_iter().map(
        |line| parse_line(line.to_string()) as i32
    ).collect();
    numbers.iter().sum()
}
