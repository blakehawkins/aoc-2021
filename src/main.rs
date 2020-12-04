use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::Split,
};

use oops::Oops;

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

fn day1_part1() {
    let mut seen = HashSet::new();

    INPUT.iter().for_each(|year| {
        let pair = 2020 - year;
        if seen.contains(&pair) {
            println!("{}", year * pair);
        }

        seen.insert(year);
    });
}

fn day1_part2() {
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

fn day2_parse<'a>(lines: Split<'a, char>) -> Vec<(RangeInclusive<i32>, char, &'a str)> {
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

fn day2_part1() {
    let mut valid_count = 0;

    let data = day2_parse(INPUT2.split('\n'));

    data.iter().for_each(|(range, limitation, phrase)| {
        if range.contains(&(phrase.chars().filter(|ch| ch == limitation).count() as i32)) {
            valid_count += 1;
        }
    });

    println!("{}", valid_count);
}

fn day2_part2() {
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

fn day3_with_slope(right: usize, input: Box<dyn Iterator<Item = &str>>) -> usize {
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

fn day3_part1() {
    let count = day3_with_slope(3, Box::new(INPUT3.split('\n')));

    println!("{}", count);
}

fn day3_part2() {
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

#[derive(Debug, Clone, PartialEq)]
struct Passport<'a> {
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
    fn new(pairs: HashMap<&'a str, &'a str>) -> Passport {
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

fn space(input: &str) -> IResult<&str, char> {
    one_of(" \n")(input)
}

fn no_space(input: &str) -> IResult<&str, &str> {
    take_while(move |ch| !(" \t\n".contains(ch)))(input)
}

fn no_colon(input: &str) -> IResult<&str, &str> {
    take_while(move |ch| ch != ':')(input)
}

fn pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(no_colon, tag(":"), no_space)(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(space, pair)(input)
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    pairs(input)
        .map(|vec| (vec.0, vec.1.into_iter().collect::<HashMap<_, _>>()))
        .map(|hm| (hm.0, Passport::new(hm.1)))
}

fn parse_passports(input: &str) -> Vec<Passport> {
    // separated_list1(tag("\n\n"), parse_passport)(input)
    input
        .split("\n\n")
        .map(parse_passport)
        .flat_map(IResult::finish)
        .map(|p| p.1)
        .collect::<Vec<Passport>>()
}

fn day4_part1() -> std::io::Result<()> {
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

fn day4_part2() -> std::io::Result<()> {
    let passports = parse_passports(INPUT4);

    let valid_passports = passports
        .into_iter()
        .map(Part2Passport::new)
        .filter(|pp| pp.is_valid())
        .count();
    println!("{}", valid_passports);

    Ok(())
}

fn main() -> std::io::Result<()> {
    day1_part1();
    day1_part2();
    day2_part1();
    day2_part2();
    day3_part1();
    day3_part2();
    day4_part1()?;
    day4_part2()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_space() {
        assert_eq!(space(" "), Ok(("", ' ')))
    }

    #[test]
    fn test_pair() {
        assert_eq!(pair("pid:324648387"), Ok(("", ("pid", "324648387"))));
        assert_eq!(pair("pid:324648387  "), Ok(("  ", ("pid", "324648387"))));
    }

    #[test]
    fn test_pairs() -> std::io::Result<()> {
        assert_eq!(
            pairs("iyr:2020 cid:82",)
                .finish()
                .ok()
                .oops("failed to parse simple")?
                .1,
            vec![("iyr", "2020"), ("cid", "82"),]
        );

        assert_eq!(
            pairs(
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
            parse_passport(
                "iyr:2020 cid:82
hgt:193in hcl:#b6652a
ecl:grn eyr:2034 byr:2026",
            )
            .unwrap()
            .1,
            Passport::new(
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
            parse_passport(
                "iyr:2020 cid:82
hgt:193in hcl:#b6652a
ecl:grn eyr:2034 byr:2026

byr:9001",
            )
            .unwrap()
            .1,
            Passport::new(
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
            parse_passports(
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
