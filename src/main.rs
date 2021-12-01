use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
    str::Split,
};

use oops::Oops;

use itertools::Itertools;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::take_while_m_n,
    bytes::complete::{tag, take_while},
    character::complete::one_of,
    character::{is_alphabetic, is_digit, is_hex_digit},
    sequence::preceded,
};
use nom::{Finish, IResult};
use tinyvec::ArrayVec;

const INPUT1: &str = include_str!("day1.input");

mod day1 {
    use crate::*;

    fn parse(input: &str) -> Vec<usize> {
       input.split('\n').filter(|v| v.len() > 0).map(|line| line.parse::<usize>().unwrap()).collect()
    }

    pub fn part1(input: &str) -> usize {
      parse(input).into_iter()
          .fold((0, 0), |(sum, last), val| (sum + (if val > last { 1 } else { 0 }), val)).0 - 1
    }

    pub fn part2(input: &str) -> usize {
      parse(input).windows(3).map(|slice| slice[0] + slice[1] + slice[2])
          .fold((0, 0), |(sum, last), val| (sum + (if val > last { 1 } else { 0 }), val)).0 - 1
    }
}

fn main() -> std::io::Result<()> {
    println!("Day  1, part 1: {}", day1::part1(INPUT1));
    println!("Day  1, part 2: {}", day1::part2(INPUT1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
}
