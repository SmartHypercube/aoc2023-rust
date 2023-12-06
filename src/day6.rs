mod parser {
    use nom::{
        IResult,
        Parser,
        bytes::complete::tag,
        character::complete::{i64, line_ending, space0, space1},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::{delimited, pair},
    };

    pub fn parser(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
        all_consuming(pair(
            delimited(pair(tag("Time:"), space0), separated_list1(space1, i64), line_ending),
            delimited(pair(tag("Distance:"), space0), separated_list1(space1, i64), line_ending),
        )).map(|(times, distances)| times.into_iter().zip(distances.into_iter()).collect()).parse(input)
    }
}

pub fn main() {
    let text = std::fs::read_to_string("data/day6.txt").unwrap();
    let records = parser::parser(&text).unwrap().1;
    println!("{}", records.into_iter().map(|(time, distance)| (0..=time).filter(|i| i * (time - i) > distance).count()).product::<usize>());
    let records = parser::parser(&text.replace(" ", "")).unwrap().1;
    println!("{}", records.into_iter().map(|(time, distance)| (0..=time).filter(|i| i * (time - i) > distance).count()).product::<usize>());
}
