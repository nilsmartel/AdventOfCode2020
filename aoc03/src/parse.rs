use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::Cable;
use crate::Direction::*;

pub fn cables(i: &str) -> IResult<&str, (Cable, Cable)> {
    separated_pair(cable, char('\n'), cable)(i)
}

pub fn cable(i: &str) -> IResult<&str, Cable> {}

pub fn entry(i: &str) -> IResult<&str, Cable> {
    let (rest, (dir, neg)) = map(one_of("RDUL"), |c| match c {
        'R' => (Right, false),
        'U' => (Up, false),
        'L' => (Right, true),
        'D' => (Up, true),
        _ => unreachable!(),
    })(i)?;
}
