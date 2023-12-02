const DIGIT_PATTERNS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    let text = std::fs::read_to_string("data/day1.txt").unwrap();
    {
        let mut sum = 0;
        for line in text.lines() {
            let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
            sum += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
        }
        println!("{}", sum);
    }
    {
        let mut sum = 0;
        for line in text.lines() {
            let first_digit = DIGIT_PATTERNS.iter().min_by_key(|i| line.find(i.0).unwrap_or(line.len())).unwrap().1;
            let last_digit = DIGIT_PATTERNS.iter().min_by_key(|i| line.rfind(i.0).map_or(-1, |i| i as i32)).unwrap().1;
            sum += first_digit * 10 + last_digit;
        }
        println!("{}", sum);
    }
}
