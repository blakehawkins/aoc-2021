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

const INPUT: [i32; 200] = [
    1228, 1584, 1258, 1692, 1509, 1927, 1177, 1854, 1946, 1815, 1925, 1531, 1529, 1920, 1576, 1392,
    1744, 1937, 1636, 1615, 1944, 1949, 1931, 1253, 1587, 1860, 1874, 1611, 2008, 1182, 1900, 1515,
    1978, 1996, 116, 1588, 1322, 1680, 1174, 1712, 1513, 1778, 1443, 1569, 1453, 708, 1783, 1926,
    1959, 2001, 1776, 1643, 1654, 1934, 1983, 1630, 1382, 1486, 1422, 1836, 1728, 1315, 1843, 1521,
    1995, 1403, 1897, 1280, 1981, 1901, 1870, 1519, 1945, 1857, 591, 1329, 1954, 1679, 1726, 1846,
    1709, 1695, 1293, 1602, 1665, 1940, 1921, 1861, 1710, 1524, 1303, 1849, 1742, 1892, 1913, 1530,
    1484, 1903, 1545, 1609, 1652, 1908, 1923, 1188, 1649, 1994, 1790, 1832, 140, 867, 1664, 1598,
    1371, 1018, 35, 1833, 1161, 1898, 1482, 1767, 1252, 1882, 1448, 1032, 1459, 1661, 1391, 1770,
    1806, 1465, 1295, 1546, 1355, 1358, 1321, 1368, 1514, 1756, 1775, 1957, 1468, 1975, 631, 1812,
    1151, 1167, 1251, 1960, 1991, 1972, 1936, 1552, 1419, 1577, 1549, 1580, 1974, 1830, 1813, 1893,
    1492, 1389, 1454, 1522, 1556, 1172, 1653, 1822, 1328, 1907, 1999, 1281, 1912, 1919, 1896, 1722,
    1341, 1720, 1201, 1512, 1298, 1254, 1947, 1505, 1594, 1334, 1592, 1943, 1405, 1589, 1263, 1930,
    1736, 1180, 1984, 1401, 1340, 1292, 1979, 1876,
];

const INPUT2: &'static str = include_str!("day2.input");

const INPUT3: &'static str = include_str!("day3.input");

const INPUT4: &'static str = include_str!("day4.input");

const INPUT5: &'static str = include_str!("day5.input");

const INPUT6: &'static str = include_str!("day6.input");

const INPUT7: &'static str = include_str!("day7.input");

const INPUT8: &'static str = include_str!("day8.input");

const INPUT9: &'static str = include_str!("day9.input");

const INPUT10: &'static str = include_str!("day10.input");

const INPUT11: &'static str = include_str!("day11.input");

const INPUT12: &'static str = include_str!("day12.input");

const INPUT13: &'static str = include_str!("day13.input");

const INPUT14: &'static str = include_str!("day14.input");

mod day1 {
    use crate::*;
    pub fn day1_part1() {
        let mut seen = HashSet::new();

        INPUT.iter().for_each(|year| {
            let pair = 2020 - year;
            if seen.contains(&pair) {
                println!("{}", year * pair);
            }

            seen.insert(year);
        });
    }

    pub fn day1_part2() {
        let mut singles = HashSet::new();
        let mut pairs: HashMap<i32, (i32, i32)> = HashMap::new();

        INPUT.iter().for_each(|year| {
            if let Some(year_data) = pairs.get(year) {
                println!("{}", year * year_data.0 * year_data.1);
            } else {
                // Check singles
                singles.iter().for_each(|single| {
                    if single + year < 2020 {
                        pairs.insert(2020 - single - year, (*single, *year));
                    }
                });

                // Insert into singles
                singles.insert(*year);
            }
        });
    }
}

mod day2 {
    use crate::*;
    pub fn day2_parse<'a>(lines: Split<'a, char>) -> Vec<(RangeInclusive<i32>, char, &'a str)> {
        lines
            .map(|line| {
                let mut chunks = line.trim().split(' ');

                let mut range = chunks.next().unwrap().split('-');
                let range = range
                    .next()
                    .oops("Missing range-begin")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(|_| format!("{:?}", range))
                    .unwrap()
                    ..=range
                        .next()
                        .oops("Missing range-end")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                let limitation = chunks
                    .next()
                    .unwrap()
                    .split(':')
                    .next()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                let phrase = chunks.next().unwrap();

                (range, limitation, phrase)
            })
            .collect()
    }

    pub fn day2_part1() {
        let mut valid_count = 0;

        let data = day2_parse(INPUT2.split('\n'));

        data.iter().for_each(|(range, limitation, phrase)| {
            if range.contains(&(phrase.chars().filter(|ch| ch == limitation).count() as i32)) {
                valid_count += 1;
            }
        });

        println!("{}", valid_count);
    }

    pub fn day2_part2() {
        let mut valid_count = 0;

        let data = day2_parse(INPUT2.split('\n'));

        data.iter().for_each(|(range, limitation, phrase)| {
            let first = phrase.chars().nth(((*range.start()) - 1) as usize).unwrap();
            let second = phrase.chars().nth(((*range.end()) - 1) as usize).unwrap();

            if (first == *limitation) ^ (second == *limitation) {
                valid_count += 1;
            }
        });

        println!("{}", valid_count);
    }
}

mod day3 {
    use crate::*;
    pub fn day3_with_slope(right: usize, input: Box<dyn Iterator<Item = &str>>) -> usize {
        input
            .map(|line| line.chars().cycle())
            .skip(1)
            .fold((right, 0), |(idex, mut sum), line| {
                if line.clone().nth(idex).unwrap() == '#' {
                    sum += 1;
                }

                (idex + right, sum)
            })
            .1
    }

    pub fn day3_part1() {
        let count = day3_with_slope(3, Box::new(INPUT3.split('\n')));

        println!("{}", count);
    }

    pub fn day3_part2() {
        println!(
            "{}",
            day3_with_slope(1, Box::new(INPUT3.split('\n')))
                * day3_with_slope(3, Box::new(INPUT3.split('\n')))
                * day3_with_slope(5, Box::new(INPUT3.split('\n')))
                * day3_with_slope(7, Box::new(INPUT3.split('\n')))
                * day3_with_slope(
                    1,
                    Box::new(
                        INPUT3
                            .split('\n')
                            .zip([true, false].iter().cycle())
                            .filter(|(_, skips)| **skips)
                            .map(|(vals, _)| vals)
                    )
                )
        )
    }
}

mod day4 {
    use crate::*;
    #[derive(Debug, Clone, PartialEq)]
    pub struct Passport<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
        #[allow(dead_code)]
        cid: Option<&'a str>,
    }

    impl<'a> Passport<'a> {
        pub fn new(pairs: HashMap<&'a str, &'a str>) -> Passport {
            Passport {
                byr: pairs.get("byr").map(|v| *v),
                iyr: pairs.get("iyr").map(|v| *v),
                eyr: pairs.get("eyr").map(|v| *v),
                hgt: pairs.get("hgt").map(|v| *v),
                hcl: pairs.get("hcl").map(|v| *v),
                ecl: pairs.get("ecl").map(|v| *v),
                pid: pairs.get("pid").map(|v| *v),
                cid: pairs.get("cid").map(|v| *v),
            }
        }
    }

    trait Validatable {
        fn is_valid(&self) -> bool;
    }

    struct Part1Passport<'a>(Passport<'a>);

    impl<'a> Part1Passport<'a> {
        fn new(passport: Passport) -> Part1Passport {
            Part1Passport(passport)
        }
    }

    impl<'a> Validatable for Part1Passport<'a> {
        fn is_valid(&self) -> bool {
            self.0.byr.is_some()
                && self.0.iyr.is_some()
                && self.0.eyr.is_some()
                && self.0.hgt.is_some()
                && self.0.hcl.is_some()
                && self.0.ecl.is_some()
                && self.0.pid.is_some()
            // Explicitly ignoring cid
        }
    }

    struct Byr<'a>(Option<&'a str>);

    impl<'a> Validatable for Byr<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .filter(|v| v.len() == 4)
                .and_then(move |v| v.parse::<i32>().ok())
                .filter(|v| *v >= 1920 && *v <= 2002)
                .is_some()
        }
    }

    struct Iyr<'a>(Option<&'a str>);

    impl<'a> Validatable for Iyr<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .filter(|v| v.len() == 4)
                .and_then(move |v| v.parse::<i32>().ok())
                .filter(|v| *v >= 2010 && *v <= 2020)
                .is_some()
        }
    }

    struct Eyr<'a>(Option<&'a str>);

    impl<'a> Validatable for Eyr<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .filter(|v| v.len() == 4)
                .and_then(move |v| v.parse::<i32>().ok())
                .filter(|v| *v >= 2020 && *v <= 2030)
                .is_some()
        }
    }

    fn parse_height(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
        nom::sequence::pair(take_while(is_digit), take_while(is_alphabetic))(input)
    }

    struct Hgt<'a>(Option<&'a str>);

    impl<'a> Validatable for Hgt<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .and_then(|v| {
                    parse_height(v.as_bytes())
                        .finish()
                        .ok()
                        .map(|parts| parts.1)
                })
                .and_then(|(q, units)| {
                    std::str::from_utf8(q)
                        .ok()
                        .and_then(|v| v.parse::<u32>().ok())
                        .map(|v| (v, units))
                })
                .filter(|(q, units)| match units {
                    &b"cm" => *q >= 150 && *q <= 193,
                    &b"in" => *q >= 59 && *q <= 76,
                    _ => false,
                })
                .is_some()
        }
    }

    fn parse_hexi(input: &[u8]) -> IResult<&[u8], &[u8]> {
        preceded(tag("#"), take_while_m_n(6, 6, is_hex_digit))(input)
    }

    struct Hcl<'a>(Option<&'a str>);

    impl<'a> Validatable for Hcl<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .and_then(|v| {
                    parse_hexi(v.as_bytes())
                        .finish()
                        .ok()
                        .filter(|(i, _)| i.len() == 0)
                        .map(|(_, out)| out)
                })
                .is_some()
        }
    }

    struct Ecl<'a>(Option<&'a str>);

    impl<'a> Validatable for Ecl<'a> {
        fn is_valid(&self) -> bool {
            let variants = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            self.0.filter(|v| variants.contains(v)).is_some()
        }
    }

    fn parse_passno(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while_m_n(9, 9, is_digit)(input)
    }

    struct Pid<'a>(Option<&'a str>);

    impl<'a> Validatable for Pid<'a> {
        fn is_valid(&self) -> bool {
            self.0
                .and_then(|v| {
                    parse_passno(v.as_bytes())
                        .finish()
                        .ok()
                        .filter(|(i, _)| i.len() == 0)
                        .map(|(_, out)| out)
                })
                .is_some()
        }
    }

    struct Part2Passport<'a>(Passport<'a>);

    impl<'a> Part2Passport<'a> {
        fn new(passport: Passport) -> Part2Passport {
            Part2Passport(passport)
        }
    }

    impl<'a> Validatable for Part2Passport<'a> {
        fn is_valid(&self) -> bool {
            let validators: Vec<Box<dyn Validatable>> = vec![
                Box::new(Part1Passport::new(self.0.clone())),
                Box::new(Byr(self.0.byr)),
                Box::new(Iyr(self.0.iyr)),
                Box::new(Eyr(self.0.eyr)),
                Box::new(Hgt(self.0.hgt)),
                Box::new(Hcl(self.0.hcl)),
                Box::new(Ecl(self.0.ecl)),
                Box::new(Pid(self.0.pid)),
                // Explicitly ignoring cid
            ];

            validators.iter().all(|v| v.is_valid())
        }
    }

    pub fn space(input: &str) -> IResult<&str, char> {
        one_of(" \n")(input)
    }

    fn no_space(input: &str) -> IResult<&str, &str> {
        take_while(move |ch| !(" \t\n".contains(ch)))(input)
    }

    fn no_colon(input: &str) -> IResult<&str, &str> {
        take_while(move |ch| ch != ':')(input)
    }

    pub fn pair(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(no_colon, tag(":"), no_space)(input)
    }

    pub fn pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_list1(space, pair)(input)
    }

    pub fn parse_passport(input: &str) -> IResult<&str, Passport> {
        pairs(input)
            .map(|vec| (vec.0, vec.1.into_iter().collect::<HashMap<_, _>>()))
            .map(|hm| (hm.0, Passport::new(hm.1)))
    }

    pub fn parse_passports(input: &str) -> Vec<Passport> {
        input
            .split("\n\n")
            .map(parse_passport)
            .flat_map(IResult::finish)
            .map(|p| p.1)
            .collect::<Vec<Passport>>()
    }

    pub fn day4_part1() -> std::io::Result<()> {
        let passports = parse_passports(INPUT4);

        let valid_passports: Vec<bool> = passports
            .into_iter()
            .map(Part1Passport::new)
            .map(|pp| pp.is_valid())
            .filter(|v| *v)
            .collect();
        println!("{}", valid_passports.iter().count());

        Ok(())
    }

    pub fn day4_part2() -> std::io::Result<()> {
        let passports = parse_passports(INPUT4);

        let valid_passports = passports
            .into_iter()
            .map(Part2Passport::new)
            .filter(|pp| pp.is_valid())
            .count();
        println!("{}", valid_passports);

        Ok(())
    }
}

mod day5 {
    use crate::*;
    pub fn day5_get_cards() -> Vec<u32> {
        INPUT5
            .split('\n')
            .map(|card| {
                card.chars()
                    .map(|char| match char {
                        'B' => 1u8,
                        'F' => 0u8,
                        'R' => 1u8,
                        'L' => 0u8,
                        _ => panic!("unexpected input"),
                    })
                    .rev()
                    .fold((0, 0), |(idx, sum), iter| {
                        let sum = sum + (iter as u32) * 2u32.pow(idx);
                        let idx = idx + 1;
                        (idx, sum)
                    })
                    .1
            })
            .collect::<Vec<_>>()
    }

    pub fn day5_part1() {
        let cards = day5_get_cards();
        println!("{}", cards.iter().max().unwrap());
    }

    pub fn day5_part2() {
        let cards = day5_get_cards();
        let sorted: Vec<u32> = cards
            .into_iter()
            .map(|v| (v, ()))
            .collect::<BTreeMap<_, ()>>()
            .into_iter()
            .map(|(v, _)| v)
            .collect();
        let gap = sorted
            .iter()
            .zip(sorted.iter().skip(1))
            .filter(|(a, b)| (*b - *a) > 1)
            .next()
            .unwrap();
        println!("{}", gap.0 + 1);
    }
}

mod day6 {
    use crate::*;
    pub fn day6_part1() {
        let group_sums: usize = INPUT6
            .split("\n\n")
            .map(|group| {
                group
                    .chars()
                    .into_iter()
                    .chain(vec!['\n'].into_iter())
                    .collect::<HashSet<_>>()
                    .len()
                    - 1
            })
            .into_iter()
            .sum();

        println!("{}", group_sums);
    }

    pub fn day6_part2() {
        let group_sums: usize = INPUT6
            .split("\n\n")
            .map(|group| {
                let individuals = group
                    .split('\n')
                    .map(|individual| individual.chars().into_iter().collect::<HashSet<_>>())
                    .collect::<Vec<_>>();

                let first = individuals.iter().next().unwrap().clone();

                individuals
                    .into_iter()
                    .fold(first, |set, iter| {
                        set.intersection(&iter)
                            .into_iter()
                            .map(|ch| *ch)
                            .collect::<HashSet<_>>()
                    })
                    .iter()
                    .len()
            })
            .into_iter()
            .sum();

        println!("{}", group_sums);
    }
}

mod day7 {
    use crate::*;
    pub fn day7_pairs(input: &str) -> Vec<(&str, &str)> {
        input
            .split('\n')
            .map(|line| {
                let mut parts = line.split("contain");

                (parts.next().unwrap().trim(), parts.next().unwrap().trim())
            })
            .collect::<Vec<(&str, &str)>>()
    }

    pub fn day7_part1(input: &str) -> usize {
        let pairs = day7_pairs(input);

        let mut count = 0usize;
        let mut io: (VecDeque<&str>, HashSet<&str>) = (VecDeque::new(), HashSet::new());
        io.0.push_back("shiny gold");

        while let Some(pivot) = io.0.pop_front() {
            io.1.insert(pivot);

            pairs
                .iter()
                .filter(|(_, contained)| contained.contains(pivot))
                .map(|(container, _)| container.split(" bags").next().unwrap())
                .for_each(|container| {
                    if !io.1.contains(container) && !io.0.contains(&container) {
                        count += 1;

                        if !io.0.contains(&container) && !io.1.contains(&container) {
                            io.0.push_back(container);
                        }
                    }
                })
        }

        count
    }

    pub fn day7_part2(input: &str) -> usize {
        let bag_mapping = day7_pairs(input)
            .iter()
            .map(|(container, contained)| {
                (
                    container.split(" bag").next().unwrap(),
                    match *contained {
                        "no other bags." => vec![],
                        _ => contained
                            .split(", ")
                            .map(|other| {
                                let no_bag = other.split(" bag").next().unwrap();

                                (
                                    no_bag[0..1]
                                        .parse::<usize>()
                                        .ok()
                                        .oops(&format!(
                                            "Failed to parse {} from {}",
                                            &no_bag[0..1],
                                            contained
                                        ))
                                        .unwrap(),
                                    &no_bag[2..],
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect::<HashMap<&str, Vec<(usize, &str)>>>();

        let mut sum = 0;
        let mut processing = vec![(1usize, "shiny gold")]
            .into_iter()
            .collect::<VecDeque<_>>();

        while let Some((count, container)) = processing.pop_front() {
            let inner = bag_mapping
                .get(container)
                .oops(&format!(
                    "Failed to get {} from {:?}",
                    container, bag_mapping
                ))
                .unwrap();

            if inner.len() > 0 {
                inner.iter().for_each(|(inner_count, inner_container)| {
                    sum += count * inner_count;
                    processing.push_back((count * inner_count, inner_container));
                });
            }
        }

        return sum;
    }
}

mod day8 {
    use crate::*;
    pub fn day8_debugger(input: &str) -> BTreeMap<isize, ((&str, isize), bool)> {
        input
            .split('\n')
            .map(|line| {
                let mut parts = line.split(' ');

                (
                    parts.next().unwrap(),
                    parts
                        .next()
                        .unwrap()
                        .trim_matches('\n')
                        .parse::<isize>()
                        .unwrap(),
                )
            })
            .zip(0..)
            .map(|(ins, idx)| (idx, (ins, false)))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn day8_evaluate_debugger(
        mut debugger: BTreeMap<isize, ((&str, isize), bool)>,
        mut iptr: isize,
    ) -> (BTreeMap<isize, ((&str, isize), bool)>, isize, bool) {
        let mut accum = 0;
        while let Some(false) = debugger.get(&iptr).map(|d| d.1) {
            let data = debugger.get(&iptr).unwrap().0;
            debugger.insert(iptr, (data, true));

            match data.0 {
                "acc" => {
                    accum += data.1;
                    iptr += 1;
                }
                "jmp" => {
                    iptr += data.1;
                }
                "nop" => {
                    iptr += 1;
                }
                _ => panic!("Unexpected instruction"),
            }
        }

        let stuck = debugger.get(&iptr).is_some();

        (debugger, accum, stuck)
    }

    pub fn day8_part1(input: &str) -> isize {
        let debugger = day8_debugger(input);
        let (_, accum, _) = day8_evaluate_debugger(debugger, 0);

        accum
    }

    pub fn day8_part2_validate(
        input: &str,
        mut potential_starting_states: HashSet<isize>,
    ) -> isize {
        let mut accum = 0;
        let mut debugger = day8_debugger(input);
        let mut iptr = 0;
        let mut flipped = None;

        while let Some(false) = debugger.get(&iptr).map(|d| d.1) {
            let data = debugger.get(&iptr).unwrap().0;
            debugger.insert(iptr, (data, true));

            match data.0 {
                "acc" => {
                    accum += data.1;
                    iptr += 1;
                }
                "jmp" => {
                    if let Some(_) = potential_starting_states.get(&(iptr + 1)) {
                        if flipped.is_none() {
                            flipped = Some(iptr);
                            debugger.insert(iptr, (("nop", data.1), false));
                        } else {
                            iptr += data.1;
                        }
                    } else {
                        iptr += data.1;
                    }
                }
                "nop" => {
                    if let Some(_) = potential_starting_states.get(&(iptr + data.1)) {
                        if flipped.is_none() {
                            flipped = Some(iptr);
                            debugger.insert(iptr, (("jmp", data.1), false));
                        } else {
                            iptr += 1;
                        }
                    } else {
                        iptr += 1;
                    }
                }
                _ => panic!("Unexpected instruction"),
            }
        }

        if debugger.get(&iptr).is_some() {
            assert!(potential_starting_states.remove(&flipped.unwrap()));
            day8_part2_validate(input, potential_starting_states)
        } else {
            accum
        }
    }

    // Reverse debugger evaluates all states that achieve the goal. Normal debugger considers flipping each instruction - if
    // doing so would arrive at a reverse debugger state, then that path is taken.
    pub fn day8_part2(input: &str) -> isize {
        let debugger = day8_debugger(input);
        let (stuck_debugger, _, _) = day8_evaluate_debugger(debugger.clone(), 0);

        let starting_states = debugger
            .iter()
            .map(|(k, _)| k)
            .filter(|idx| !stuck_debugger.get(idx).unwrap().1)
            .collect::<Vec<_>>();

        let potential_starting_states = starting_states
            .into_iter()
            .map(|idx| day8_evaluate_debugger(debugger.clone(), *idx))
            .filter(|(_, _, stuck)| !stuck)
            .map(|(dbg, _, _)| {
                dbg.into_iter()
                    .filter(|(_, (_, seen))| *seen)
                    .map(|(key, _)| key)
                    .collect::<HashSet<_>>()
            })
            .fold(HashSet::new(), |set, iter| {
                set.union(&iter).map(|v| *v).collect()
            });

        day8_part2_validate(input, potential_starting_states)
    }
}

mod day9 {
    use crate::*;
    pub fn day9_part1(input: &str) -> usize {
        input
            .split('\n')
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .windows(26)
            .map(|vals| {
                let goal = vals[25];
                let variants = vals[0..=25]
                    .iter()
                    .tuple_combinations()
                    .map(|(a, b)| a + b)
                    .collect::<HashSet<_>>();

                (goal, variants.contains(&goal))
            })
            .filter(|(_, succ)| !succ)
            .next()
            .unwrap()
            .0
    }

    pub fn day9_part2(input: &str, goal: usize) -> usize {
        let parsed = input
            .split('\n')
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let vals = parsed.as_slice();
        let mut ptr0 = 0;
        let mut ptr1 = 1;

        let mut sum: usize = vals[ptr0..=ptr1].iter().sum();
        while sum != goal {
            if sum > goal {
                ptr0 += 1;
            } else {
                ptr1 += 1;
            }

            sum = vals[ptr0..=ptr1].iter().sum();
        }

        vals[ptr0..=ptr1].iter().min().unwrap() + vals[ptr0..=ptr1].iter().max().unwrap()
    }
}

mod day10 {
    use crate::*;
    pub fn day10_part1(input: &str) -> usize {
        let joltages = input
            .split('\n')
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<HashSet<_>>();
        let max_joltage = joltages.iter().max().unwrap();

        let mut joltages1 = 0;
        // let mut joltages2 = 0;
        let mut joltages3 = 1;

        let mut joltage = 0;
        while joltage < *max_joltage {
            if joltages.get(&(joltage + 1)).is_some() {
                joltages1 += 1;
                joltage += 1;
            } else if joltages.get(&(joltage + 2)).is_some() {
                // joltages2 += 1;
                joltage += 2;
            } else if joltages.get(&(joltage + 3)).is_some() {
                joltages3 += 1;
                joltage += 3;
            } else {
                panic!("impossibru");
            }
        }

        joltages1 * joltages3
    }

    pub fn day10_part2_recurse(
        idx: usize,
        decision_indices: &BTreeMap<usize, ArrayVec<[usize; 3]>>,
        memo: &mut HashMap<usize, usize>,
    ) -> usize {
        if let Some(res) = memo.get(&idx) {
            return *res;
        }

        if let None = decision_indices.get(&idx) {
            return 1;
        }

        let res = decision_indices
            .get(&idx)
            .unwrap()
            .into_iter()
            .map(|next| day10_part2_recurse(*next, decision_indices, memo))
            .sum();

        memo.insert(idx, res);
        res
    }

    pub fn day10_part2(input: &str) -> usize {
        let joltages_sorted = input
            .split('\n')
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let max = joltages_sorted[joltages_sorted.len() - 1];

        let counts: BTreeMap<_, _> = vec![0]
            .into_iter()
            .chain(joltages_sorted.into_iter())
            .chain(vec![usize::MAX, usize::MAX, usize::MAX].into_iter())
            .tuple_windows::<(_, _, _, _)>()
            .map(|(a, b, c, d)| {
                (
                    a,
                    [b, c, d]
                        .iter()
                        .filter(move |v| **v <= a + 3)
                        .cloned()
                        .collect::<ArrayVec<_>>(),
                )
            })
            .filter(|(idx, _)| *idx != max)
            .collect();

        day10_part2_recurse(0, &counts, &mut HashMap::new())
    }
}

mod day11 {
    use crate::*;
    #[derive(Copy, Clone, PartialEq)]
    pub enum Seat {
        Floor,
        Empty,
        Occupied,
        Void,
    }

    impl Default for Seat {
        fn default() -> Self {
            Seat::Floor
        }
    }

    pub fn day11_parse_input(input: &str) -> ArrayVec<[ArrayVec<[Seat; 128]>; 128]> {
        input
            .split('\n')
            .map(|line| {
                line.bytes()
                    .map(|byte| match byte {
                        b'.' => Seat::Floor,
                        b'L' => Seat::Empty,
                        b'#' => Seat::Occupied,
                        _ => panic!("Unexpected input"),
                    })
                    .collect::<ArrayVec<[Seat; 128]>>()
            })
            .collect::<ArrayVec<[ArrayVec<[Seat; 128]>; 128]>>()
    }

    pub fn day11_part1(input: &str) -> usize {
        let mut arrays = day11_parse_input(input);

        let mut stable = false;
        let mut arrays_alt = arrays.clone();
        while !stable {
            let mut did_swap = false;
            for ii in 1..arrays.len() - 1 {
                for jj in 1..arrays[0].len() - 1 {
                    // Consider swap
                    let occupied_count = [
                        (ii - 1, jj - 1),
                        (ii - 1, jj),
                        (ii - 1, jj + 1),
                        (ii, jj - 1),
                        (ii, jj + 1),
                        (ii + 1, jj - 1),
                        (ii + 1, jj),
                        (ii + 1, jj + 1),
                    ]
                    .iter()
                    .map(|(i, j)| arrays[*i][*j])
                    .filter(|seat| *seat == Seat::Occupied)
                    .count();

                    let do_swap = match (arrays[ii][jj], occupied_count) {
                        (Seat::Empty, count) if count == 0 => (Seat::Occupied, true),
                        (Seat::Occupied, count) if count >= 4 => (Seat::Empty, true),
                        _ => (arrays[ii][jj], false),
                    };

                    arrays_alt[ii][jj] = do_swap.0;
                    did_swap |= do_swap.1;
                }
            }

            if !did_swap {
                stable = true;
            }

            std::mem::swap(&mut arrays, &mut arrays_alt);
        }

        arrays
            .iter()
            .map(|line| line.iter().filter(|v| **v == Seat::Occupied).count())
            .sum()
    }

    pub fn day11_part2(input: &str) -> usize {
        let mut arrays = day11_parse_input(input);

        let mut stable = false;
        let mut arrays_alt = arrays.clone();
        while !stable {
            let mut did_swap = false;
            for ii in 1..arrays.len() - 1 {
                for jj in 1..arrays[0].len() - 1 {
                    // Consider swap
                    let occupied_count = [
                        (-1isize, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ]
                    .iter()
                    .map(|(i, j)| {
                        (1..)
                            .map(|idx| {
                                arrays
                                    .get((ii as isize + (idx * i)) as usize)
                                    .and_then(|line| line.get((jj as isize + (idx * j)) as usize))
                                    .unwrap_or_else(|| &Seat::Void)
                            })
                            .filter(|seat| **seat != Seat::Floor)
                            .next()
                            .unwrap()
                    })
                    .filter(|seat| **seat == Seat::Occupied)
                    .count();

                    let do_swap = match (arrays[ii][jj], occupied_count) {
                        (Seat::Empty, count) if count == 0 => (Seat::Occupied, true),
                        (Seat::Occupied, count) if count >= 5 => (Seat::Empty, true),
                        _ => (arrays[ii][jj], false),
                    };

                    arrays_alt[ii][jj] = do_swap.0;
                    did_swap |= do_swap.1;
                }
            }

            if !did_swap {
                stable = true;
            }

            std::mem::swap(&mut arrays, &mut arrays_alt);
        }

        arrays
            .iter()
            .map(|line| line.iter().filter(|v| **v == Seat::Occupied).count())
            .sum()
    }
}

mod day12 {
    #[derive(Copy, Clone, Debug)]
    pub enum Directive {
        Forward,
        Right,
        Left,
        North,
        East,
        South,
        West,
    }

    pub fn day12_parse_input(input: &str) -> Vec<(Directive, isize)> {
        input
            .split('\n')
            .map(|line| {
                let directive = match line.bytes().next().unwrap() {
                    b'F' => Directive::Forward,
                    b'R' => Directive::Right,
                    b'L' => Directive::Left,
                    b'N' => Directive::North,
                    b'E' => Directive::East,
                    b'S' => Directive::South,
                    b'W' => Directive::West,
                    _ => panic!("Unexpected directive"),
                };

                let magnitude = line[1..].parse::<isize>().unwrap();

                (directive, magnitude)
            })
            .collect()
    }

    pub fn day12_part1(input: &str) -> usize {
        let values = day12_parse_input(input).into_iter().fold(
            (0, 0, Directive::East),
            |(x, y, face), (directive, magnitude)| match (directive, face, magnitude) {
                (Directive::Forward, Directive::East, _) => (x + magnitude, y, face),
                (Directive::Forward, Directive::North, _) => (x, y + magnitude, face),
                (Directive::Forward, Directive::South, _) => (x, y - magnitude, face),
                (Directive::Forward, Directive::West, _) => (x - magnitude, y, face),

                (Directive::Left, Directive::East, 90) => (x, y, Directive::North),
                (Directive::Left, Directive::North, 90) => (x, y, Directive::West),
                (Directive::Left, Directive::West, 90) => (x, y, Directive::South),
                (Directive::Left, Directive::South, 90) => (x, y, Directive::East),
                (Directive::Left, Directive::East, 180) => (x, y, Directive::West),
                (Directive::Left, Directive::North, 180) => (x, y, Directive::South),
                (Directive::Left, Directive::West, 180) => (x, y, Directive::East),
                (Directive::Left, Directive::South, 180) => (x, y, Directive::North),
                (Directive::Left, Directive::East, 270) => (x, y, Directive::South),
                (Directive::Left, Directive::North, 270) => (x, y, Directive::East),
                (Directive::Left, Directive::West, 270) => (x, y, Directive::North),
                (Directive::Left, Directive::South, 270) => (x, y, Directive::West),
                (Directive::Right, Directive::East, 90) => (x, y, Directive::South),
                (Directive::Right, Directive::North, 90) => (x, y, Directive::East),
                (Directive::Right, Directive::West, 90) => (x, y, Directive::North),
                (Directive::Right, Directive::South, 90) => (x, y, Directive::West),
                (Directive::Right, Directive::East, 180) => (x, y, Directive::West),
                (Directive::Right, Directive::North, 180) => (x, y, Directive::South),
                (Directive::Right, Directive::West, 180) => (x, y, Directive::East),
                (Directive::Right, Directive::South, 180) => (x, y, Directive::North),
                (Directive::Right, Directive::East, 270) => (x, y, Directive::North),
                (Directive::Right, Directive::North, 270) => (x, y, Directive::West),
                (Directive::Right, Directive::West, 270) => (x, y, Directive::South),
                (Directive::Right, Directive::South, 270) => (x, y, Directive::East),

                (Directive::North, _, _) => (x, y + magnitude, face),
                (Directive::East, _, _) => (x + magnitude, y, face),
                (Directive::South, _, _) => (x, y - magnitude, face),
                (Directive::West, _, _) => (x - magnitude, y, face),

                _ => panic!("Unparsed input"),
            },
        );

        values.0.abs() as usize + values.1.abs() as usize
    }

    pub fn day12_part2(input: &str) -> usize {
        let values = day12_parse_input(input).into_iter().fold(
            (0, 0, Directive::East, 10isize, 1isize),
            |(x, y, face, wx, wy), (directive, magnitude)| match (directive, face, magnitude) {
                (Directive::Forward, _, _) => {
                    (x + magnitude * wx, y + magnitude * wy, face, wx, wy)
                }

                (Directive::Left, _, 90) => (x, y, face, -wy, wx),
                (Directive::Left, _, 180) => (x, y, face, -wx, -wy),
                (Directive::Left, _, 270) => (x, y, face, wy, -wx),
                (Directive::Right, _, 90) => (x, y, face, wy, -wx),
                (Directive::Right, _, 180) => (x, y, face, -wx, -wy),
                (Directive::Right, _, 270) => (x, y, face, -wy, wx),

                (Directive::North, _, _) => (x, y, face, wx, wy + magnitude),
                (Directive::East, _, _) => (x, y, face, wx + magnitude, wy),
                (Directive::South, _, _) => (x, y, face, wx, wy - magnitude),
                (Directive::West, _, _) => (x, y, face, wx - magnitude, wy),

                _ => panic!("Unparsed input"),
            },
        );

        values.0.abs() as usize + values.1.abs() as usize
    }
}

mod day13 {
    use crate::*;
    pub fn day13_part1(input: &str) -> usize {
        let mut parts = input.split('\n');
        let goal = parts.next().unwrap().parse::<usize>().unwrap();

        let sorted = parts
            .next()
            .unwrap()
            .split(',')
            .filter(|v| v != &"x")
            .map(|v| v.parse::<usize>().unwrap())
            .map(|x| {
                (
                    (0..)
                        .map(|idx| idx * x)
                        .filter(|v| *v >= goal)
                        .next()
                        .unwrap(),
                    x,
                )
            })
            .collect::<BTreeMap<_, _>>();

        let (mins, bus) = sorted.iter().next().unwrap();

        bus * (mins - goal)
    }

    pub fn day13_part2(input: &str) -> u128 {
        let mut parts = input.split('\n');
        let _ = parts.next().unwrap().parse::<usize>().unwrap();

        let indices = (0u128..)
            .zip(parts.next().unwrap().split(','))
            .filter(|(_, v)| v != &"x")
            .map(|(idx, v)| (idx, v.parse::<u128>().unwrap()))
            .collect::<Vec<_>>();

        let gens = indices
            .iter()
            .map(|(idx, val)| {
                (
                    idx,
                    (0..)
                        .map(move |v| v * val)
                        .filter(|v| v > &100000000000000u128),
                )
            })
            .collect::<Vec<_>>();

        // multizip(gens.next().unwrap(), gens.next().unwrap(), gens.next().unwrap(), gens.next().unwrap(),
        // gens.next().unwrap(), gens.next().unwrap(), gens.next().unwrap(), gens.next().unwrap(), gens.next().unwrap())
        //    .filter(|(a, b, c, d, e, f ,g, h, i)| {

        // }).next().0.1

        0
    }
}

mod day14 {
    pub fn day14_part1(input: &str) -> usize {
        0
    }
}

fn main() -> std::io::Result<()> {
    day1::day1_part1();
    day1::day1_part2();
    day2::day2_part1();
    day2::day2_part2();
    day3::day3_part1();
    day3::day3_part2();
    day4::day4_part1()?;
    day4::day4_part2()?;
    day5::day5_part1();
    day5::day5_part2();
    day6::day6_part1();
    day6::day6_part2();
    println!("Day  7, part 1: {}", day7::day7_part1(INPUT7));
    println!("Day  7, part 2: {}", day7::day7_part2(INPUT7));
    println!("Day  8, part 1: {}", day8::day8_part1(INPUT8));
    println!("Day  8, part 2: {}", day8::day8_part2(INPUT8));
    let day9_goal = day9::day9_part1(INPUT9);
    println!("Day  9, part 1: {}", day9_goal);
    println!("Day  9, part 2: {}", day9::day9_part2(INPUT9, day9_goal));
    println!("Day 10, part 1: {}", day10::day10_part1(INPUT10));
    println!("Day 10, part 2: {}", day10::day10_part2(INPUT10));
    println!("Day 11, part 1: {}", day11::day11_part1(INPUT11));
    println!("Day 11, part 2: {}", day11::day11_part2(INPUT11));
    println!("Day 12, part 1: {}", day12::day12_part1(INPUT12));
    println!("Day 12, part 2: {}", day12::day12_part2(INPUT12));
    println!("Day 13, part 1: {}", day13::day13_part1(INPUT13));
    println!("!Day 13, part 2: {}", day13::day13_part2(INPUT13));
    println!("Day 14, part1: {}", day14::day14_part1(INPUT14));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_day12() {
        assert_eq!(
            day12::day12_part2(
                "F10
N3
F7
R90
F11"
            ),
            286
        )
    }

    #[test]
    fn test_day10() {
        assert_eq!(
            day10::day10_part2(
                "16
10
15
5
1
11
7
19
6
12
4"
            ),
            8
        );
    }

    #[test]
    fn test_day10_2() {
        assert_eq!(
            day10::day10_part2(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            ),
            19208
        )
    }

    #[test]
    fn test_day7() {
        assert_eq!(
            day7::day7_part1(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            4
        );

        assert_eq!(
            day7::day7_part2(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            32
        );

        assert_eq!(
            day7::day7_part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            ),
            126
        )
    }

    #[test]
    fn test_day8() {
        assert_eq!(
            day8::day8_part2(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        )
    }

    #[test]
    fn test_space() {
        assert_eq!(day4::space(" "), Ok(("", ' ')))
    }

    #[test]
    fn test_pair() {
        assert_eq!(day4::pair("pid:324648387"), Ok(("", ("pid", "324648387"))));
        assert_eq!(
            day4::pair("pid:324648387  "),
            Ok(("  ", ("pid", "324648387")))
        );
    }

    #[test]
    fn test_pairs() -> std::io::Result<()> {
        assert_eq!(
            day4::pairs("iyr:2020 cid:82",)
                .finish()
                .ok()
                .oops("failed to parse simple")?
                .1,
            vec![("iyr", "2020"), ("cid", "82"),]
        );

        assert_eq!(
            day4::pairs(
                "iyr:2020 cid:82
hgt:193in hcl:#b6652a
ecl:grn eyr:2034 byr:2026",
            )
            .finish()
            .ok()
            .oops("failed to parse")?
            .1,
            (
                "",
                vec![
                    ("iyr", "2020"),
                    ("cid", "82"),
                    ("hgt", "193in"),
                    ("hcl", "#b6652a"),
                    ("ecl", "grn"),
                    ("eyr", "2034"),
                    ("byr", "2026"),
                ]
            )
                .1
        );

        Ok(())
    }

    #[test]
    fn test_passport() -> std::io::Result<()> {
        assert_eq!(
            day4::parse_passport(
                "iyr:2020 cid:82
hgt:193in hcl:#b6652a
ecl:grn eyr:2034 byr:2026",
            )
            .unwrap()
            .1,
            day4::Passport::new(
                vec![
                    ("iyr", "2020"),
                    ("cid", "82"),
                    ("hgt", "193in"),
                    ("hcl", "#b6652a"),
                    ("ecl", "grn"),
                    ("eyr", "2034"),
                    ("byr", "2026"),
                ]
                .into_iter()
                .collect()
            )
        );

        assert_eq!(
            day4::parse_passport(
                "iyr:2020 cid:82
hgt:193in hcl:#b6652a
ecl:grn eyr:2034 byr:2026

byr:9001",
            )
            .unwrap()
            .1,
            day4::Passport::new(
                vec![
                    ("iyr", "2020"),
                    ("cid", "82"),
                    ("hgt", "193in"),
                    ("hcl", "#b6652a"),
                    ("ecl", "grn"),
                    ("eyr", "2034"),
                    ("byr", "2026"),
                ]
                .into_iter()
                .collect()
            )
        );

        Ok(())
    }

    #[test]
    fn test_passports() -> std::io::Result<()> {
        assert_eq!(
            day4::parse_passports(
                "eyr:2027
ecl:amb iyr:2014 hcl:#fffffd
pid:838758900
hgt:177cm byr:1942

cid:166 iyr:2020 ecl:lzr hgt:70cm eyr:2040 byr:2004 hcl:#733820

eyr:2028 ecl:grn byr:2016 cid:61 iyr:2010
hcl:#cfa07d
hgt:155in
pid:9594283803
",
            )
            .len(),
            3
        );

        Ok(())
    }
}
