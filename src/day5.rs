mod part1 {
    pub struct Map<'a> {
        pub from: &'a str,
        pub to: &'a str,
        pub items: Vec<(i64, i64, i64)>,
    }

    impl<'a> Map<'a> {
        pub fn map(&self, input: i64) -> i64 {
            for &(dest_start, source_start, length) in &self.items {
                if input >= source_start && input < source_start + length {
                    return input - source_start + dest_start;
                }
            }
            input
        }
    }

    mod parser {
        use super::*;
        use nom::{
            IResult,
            Parser,
            bytes::complete::tag,
            character::complete::{alpha1, i64, line_ending, space1},
            combinator::all_consuming,
            multi::{many1, separated_list1},
            sequence::{delimited, terminated, tuple},
        };

        pub fn parser(input: &str) -> IResult<&str, (Vec<i64>, Vec<Map>)> {
            all_consuming(tuple((
                delimited(tag("seeds: "), separated_list1(space1, i64), tag("\n\n")),
                separated_list1(line_ending, tuple((
                    terminated(alpha1, tag("-to-")),
                    terminated(alpha1, tag(" map:\n")),
                    many1(tuple((
                        terminated(i64, space1),
                        terminated(i64, space1),
                        terminated(i64, line_ending),
                    ))),
                )).map(|(from, to, items)| Map { from, to, items })),
            )))(input)
        }
    }

    pub fn main() {
        let text = std::fs::read_to_string("data/day5.txt").unwrap();
        let (seeds, maps) = parser::parser(&text).unwrap().1;
        println!("{}", seeds.iter().map(|&i| {
            let mut value = i;
            for map in &maps {
                value = map.map(value);
            }
            value
        }).min().unwrap());
    }
}

mod part2 {
    pub struct Map<'a> {
        pub from: &'a str,
        pub to: &'a str,
        pub items: Vec<(i64, i64, i64)>,
    }

    impl<'a> Map<'a> {
        pub fn map(&self, input: i64) -> (i64, i64) {
            for &(dest_start, source_start, length) in &self.items {
                if input >= source_start && input < source_start + length {
                    return (input - source_start + dest_start, source_start + length - input);
                }
            }
            (
                input,
                self.items.iter()
                    .filter(|&&(_, source_start, _)| input < source_start)
                    .map(|&(_, source_start, _)| source_start - input)
                    .min().unwrap(),
            )
        }
    }

    mod parser {
        use super::*;
        use nom::{
            IResult,
            Parser,
            bytes::complete::tag,
            character::complete::{alpha1, i64, line_ending, space1},
            combinator::all_consuming,
            multi::{many1, separated_list1},
            sequence::{delimited, terminated, separated_pair, tuple},
        };

        pub fn parser(input: &str) -> IResult<&str, (Vec<(i64, i64)>, Vec<Map>)> {
            all_consuming(tuple((
                delimited(tag("seeds: "), separated_list1(space1, separated_pair(i64, space1, i64)), tag("\n\n")),
                separated_list1(line_ending, tuple((
                    terminated(alpha1, tag("-to-")),
                    terminated(alpha1, tag(" map:\n")),
                    many1(tuple((
                        terminated(i64, space1),
                        terminated(i64, space1),
                        terminated(i64, line_ending),
                    ))),
                )).map(|(from, to, items)| Map { from, to, items })),
            )))(input)
        }
    }

    pub fn main() {
        let text = std::fs::read_to_string("data/day5.txt").unwrap();
        let (seeds, maps) = parser::parser(&text).unwrap().1;
        let seed_to_location = |seed| {
            let mut value = seed;
            let mut length = i64::MAX;
            for map in &maps {
                let r = map.map(value);
                value = r.0;
                length = length.min(r.1);
            }
            (value, length)
        };
        println!("{}", seeds.iter().map(|&(start, length)| {
            let mut current = start;
            let mut min_location = i64::MAX;
            while current < start + length {
                let r = seed_to_location(current);
                min_location = min_location.min(r.0);
                current += r.1;
            }
            min_location
        }).min().unwrap());
    }
}

fn main() {
    part1::main();
    part2::main();
}
