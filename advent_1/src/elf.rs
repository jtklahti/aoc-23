mod tests;

fn parse_digit_textual(s: &str) -> u32 {
    let digit_map = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for (str, num) in digit_map {
        if s.len() >= str.len() && str == &s[0..str.len()] {
            return num;
        }
    }
    return 0;
}

fn parse_digit_numeral(s: &str) -> u32 {
    s.chars()
        .nth(0)
        .unwrap_or_default()
        .to_digit(10)
        .unwrap_or_default()
}

pub fn parse_digit(s: &str) -> u32 {
    let numeral = parse_digit_numeral(s);
    if numeral > 0 {
        return numeral;
    } 
    return parse_digit_textual(s);
}

pub fn parse_line(line: String) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    if 0 == line.len() {
        return 0
    }
    for index in 0..line.len() {
        let str = &line[index..];
        let digit = parse_digit(str);
        if digit == 0 {
            continue;
        }
        if first == 0 {
            first = digit;
        }
        last = digit;
    }
    return 10 * first + last;
}

pub fn parse_content(content: String) -> i32 {
    let numbers: Vec<i32> = content
        .lines()
        .into_iter()
        .map(|line| parse_line(line.to_string()) as i32)
        .collect();
    numbers.iter().sum()
}
