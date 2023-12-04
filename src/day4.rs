use std::{collections::HashSet, vec};

pub struct Card {
    pub id: i64,
    pub winning_numbers: HashSet<i64>,
    pub my_numbers: Vec<i64>,
}

impl Card {
    pub fn win_count(&self) -> i64 {
        self.my_numbers.iter().filter(|i|
            self.winning_numbers.contains(i)
        ).count() as i64
    }
}

mod parser {
    use super::*;
    use nom::{
        IResult,
        Parser,
        bytes::complete::tag,
        character::complete::{i64, line_ending, space1},
        combinator::all_consuming,
        multi::{many0, many1},
        sequence::{delimited, preceded, terminated, tuple},
    };

    pub fn parser(input: &str) -> IResult<&str, Vec<Card>> {
        all_consuming(many0(
            tuple((
                delimited(tag("Card"), preceded(space1, i64), tag(":")),
                terminated(many1(preceded(space1, i64)), preceded(space1, tag("|"))).map(|v| v.into_iter().collect()),
                terminated(many1(preceded(space1, i64)), line_ending),
            )).map(|(id, winning_numbers, my_numbers)| Card { id, winning_numbers, my_numbers })
        ))(input)
    }
}

fn main() {
    let text = std::fs::read_to_string("data/day4.txt").unwrap();
    let cards = parser::parser(&text).unwrap().1;
    println!("{}", cards.iter().map(|card|2_i64.pow(card.win_count() as u32) / 2).sum::<i64>());
    let mut card_count = vec![1; cards.len()];
    for i in 0..cards.len() {
        let count = cards[i].win_count() as usize;
        for j in 0..count {
            if i + j + 1 < cards.len() {
                card_count[i + j + 1] += card_count[i];
            }
        }
    }
    println!("{}", card_count.iter().sum::<i64>());
}
