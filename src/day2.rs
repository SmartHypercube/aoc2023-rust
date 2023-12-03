use std::collections::HashMap;

mod nom {
    pub use nom::{
        IResult,
        bytes::complete::*,
        character::complete::*,
        combinator::*,
        multi::*,
        sequence::*,
    };
}

pub struct Game<'a> {
    pub id: i64,
    pub cubes_list: Vec<HashMap<&'a str, i64>>,
}

fn within_limit(cubes_list: &HashMap<&str, i64>) -> bool {
    cubes_list.iter().all(|(&color, &count)| count <= match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    })
}

fn parser<'a>(input: &'a str) -> nom::IResult<&str, Vec<Game>> {
    nom::many0(|input| {
        let (input, _) = nom::tag("Game ")(input)?;
        let (input, id) = nom::i64(input)?;
        let (input, _) = nom::tag(": ")(input)?;
        let (input, cubes_list) = nom::separated_list1(nom::tag("; "), |input| {
            let mut cubes = HashMap::new();
            let (input, _) = nom::separated_list1(nom::tag(", "), |input| {
                let (input, count) = nom::i64(input)?;
                let (input, _) = nom::tag(" ")(input)?;
                let (input, color) = nom::alpha1(input)?;
                *cubes.entry(color).or_insert(0) += count;
                Ok((input, ()))
            })(input)?;
            Ok((input, cubes))
        })(input)?;
        let (input, _) = nom::line_ending(input)?;
        Ok((input, Game { id, cubes_list }))
    })(input)
}

fn main() {
    let text = std::fs::read_to_string("data/day2.txt").unwrap();
    let games = nom::all_consuming(parser)(&text).unwrap().1;
    let mut sum = 0;
    for game in &games {
        if game.cubes_list.iter().all(within_limit) {
            sum += game.id;
        }
    }
    println!("{}", sum);
    let mut sum = 0;
    for game in &games {
        let mut minimum_cubes = HashMap::new();
        for cubes in &game.cubes_list {
            for (&color, &count) in cubes {
                let entry = minimum_cubes.entry(color).or_insert(0);
                *entry = (*entry).max(count);
            }
        }
        sum += minimum_cubes.values().product::<i64>();
    }
    println!("{}", sum);
}
