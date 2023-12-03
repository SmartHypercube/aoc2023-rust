use std::collections::HashMap;

fn main() {
    let text = std::fs::read_to_string("data/day3.txt").unwrap();
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    for (i, line) in text.lines().enumerate() {
        let mut number_start = None;
        for (j, c) in line.chars().enumerate() {
            if c == '\n' {
                continue;
            }
            if c.is_digit(10) {
                number_start = number_start.or(Some(j));
            } else {
                if let Some(start) = number_start {
                    numbers.push((i as i64, start as i64, j as i64 - 1, line[start..=j-1].parse::<i64>().unwrap()));
                    number_start = None;
                }
                if c != '.' {
                    symbols.insert((i as i64, j as i64), (c, Vec::new()));
                }
            }
        }
        if let Some(start) = number_start {
            numbers.push((i as i64, start as i64, line.len() as i64 - 1, line[start..].parse::<i64>().unwrap()));
        }
    }
    let mut sum = 0;
    for (row, start, end, number) in numbers {
        let mut neighbors = Vec::new();
        let i = row - 1;
        for j in start-1..=end+1 {
            neighbors.push((i, j));
        }
        neighbors.push((row, start-1));
        neighbors.push((row, end+1));
        let i = row + 1;
        for j in start-1..=end+1 {
            neighbors.push((i, j));
        }
        let mut is_part_number = false;
        for (i, j) in neighbors {
            if let Some((_, l)) = symbols.get_mut(&(i, j)) {
                is_part_number = true;
                l.push(number);
            }
        }
        if is_part_number {
            sum += number;
        }
    }
    println!("{}", sum);
    let mut sum = 0;
    for (c, l) in symbols.values() {
        if *c == '*' && l.len() == 2 {
            sum += l[0] * l[1];
        }
    }
    println!("{}", sum);
}
