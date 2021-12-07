const INPUT1: &str = include_str!("day1.input");
const INPUT2: &str = include_str!("day2.input");
const INPUT3: &str = include_str!("day3.input");
const INPUT4: &str = include_str!("day4.input");

mod day1 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|line| line.parse::<usize>().unwrap())
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        parse(input)
            .into_iter()
            .fold((0, 0), |(sum, last), val| {
                (sum + (if val > last { 1 } else { 0 }), val)
            })
            .0
            - 1
    }

    pub fn part2(input: &str) -> usize {
        parse(input)
            .windows(3)
            .map(|slice| slice[0] + slice[1] + slice[2])
            .fold((0, 0), |(sum, last), val| {
                (sum + (if val > last { 1 } else { 0 }), val)
            })
            .0
            - 1
    }
}

mod day2 {
    fn parse(input: &str) -> Vec<(&str, usize)> {
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|line| {
                let mut parts = line.split(' ');

                (
                    parts.next().unwrap(),
                    parts.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        let (depth, distance) = parse(input).into_iter().fold(
            (0, 0),
            |(depth, distance), (cmd, magnitude)| match cmd {
                "forward" => (depth, distance + magnitude),
                "down" => (depth + magnitude, distance),
                "up" => (depth - magnitude, distance),
                _ => panic!("Unrecognized command"),
            },
        );

        depth * distance
    }

    pub fn part2(input: &str) -> usize {
        let (depth, distance, _) =
            parse(input)
                .into_iter()
                .fold(
                    (0, 0, 0),
                    |(depth, distance, aim), (cmd, magnitude)| match cmd {
                        "forward" => (depth + aim * magnitude, distance + magnitude, aim),
                        "down" => (depth, distance, aim + magnitude),
                        "up" => (depth, distance, aim - magnitude),
                        _ => panic!("Unrecognized command"),
                    },
                );

        depth * distance
    }
}

mod day3 {
    fn parse(input: &str) -> Vec<Vec<char>> {
        input
            .split('\n')
            .filter(|v| !v.is_empty())
            .map(|line| line.chars().collect())
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        let input = parse(input);
        let line_count = input.len();
        let counts = input.into_iter().fold([0; 12], |mut counts, vals| {
            vals.into_iter().enumerate().for_each(|(idx, val)| {
                if val == '1' {
                    counts[idx] += 1;
                }
            });

            counts
        });

        let (gamma, epsilon) = counts
            .iter()
            .map(|count| if *count > (line_count / 2) { 1 } else { 0 })
            .fold((0, 0), |(gamma, epsilon), avg| {
                ((gamma << 1) + avg, (epsilon << 1) + (1 - avg))
            });

        gamma * epsilon
    }

    fn part2_reduce(input: &mut Vec<Vec<char>>, most_common: bool, idx: usize) -> usize {
        let line_count = input.len();

        let count = input
            .iter()
            .filter(|bin| (bin[idx] == '1') == most_common)
            .count();

        let select = match (count * 2, line_count, most_common) {
            (a, b, true) if a == b => '1',
            (a, b, false) if a == b => '0',
            (a, b, _) => {
                if a > b {
                    '1'
                } else {
                    '0'
                }
            }
        };

        input.retain(|bin| (bin[idx] == select));

        if input.len() > 1 {
            return part2_reduce(input, most_common, idx + 1);
        }

        input[0]
            .iter()
            .fold(0, |res, bit| (res << 1) + (if *bit == '1' { 1 } else { 0 }))
    }

    pub fn part2(input: &str) -> usize {
        let mut input = parse(input);

        part2_reduce(&mut input.clone(), true, 0) * part2_reduce(&mut input, false, 0)
    }
}

mod day4 {
    use std::cell::RefCell;

    struct Board {
        cells: RefCell<Vec<usize>>,
        requirement_variants: RefCell<Vec<Vec<usize>>>,
    }

    impl From<Vec<usize>> for Board {
        fn from(items: Vec<usize>) -> Self {
            Board {
                cells: RefCell::new(items.clone()),
                requirement_variants: RefCell::new(vec![
                    vec![items[0], items[1], items[2], items[3], items[4]],
                    vec![items[5], items[6], items[7], items[8], items[9]],
                    vec![items[10], items[11], items[12], items[13], items[14]],
                    vec![items[15], items[16], items[17], items[18], items[19]],
                    vec![items[20], items[21], items[22], items[23], items[24]],
                    vec![items[0], items[5], items[10], items[15], items[20]],
                    vec![items[1], items[6], items[11], items[16], items[21]],
                    vec![items[2], items[7], items[12], items[17], items[22]],
                    vec![items[3], items[8], items[13], items[18], items[23]],
                    vec![items[4], items[9], items[14], items[19], items[24]],
                ]),
            }
        }
    }

    impl Board {
        fn consume(&mut self, item: usize) {
            let mut vecs = self.requirement_variants.borrow_mut();
            for vec in &mut *vecs {
                vec.retain(|v| *v != item)
            }
            let mut cells = self.cells.borrow_mut();
            cells.retain(|v| *v != item);
        }

        fn winner(&self) -> Option<usize> {
            let vecs = self.requirement_variants.borrow_mut();
            let cells = self.cells.borrow_mut();

            if vecs.iter().any(|v| v.is_empty()) {
                return Some(cells.iter().sum());
            }

            None
        }
    }

    fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
        let mut input = input.split("\n\n");

        let stream = input
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let boards = input
            .map(|table| {
                table
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .into()
            })
            .collect();

        (stream, boards)
    }

    pub fn part1(input: &str) -> usize {
        let (input, mut boards) = parse(input);

        for input in input {
            for board in &mut boards {
                board.consume(input);
                if let Some(v) = board.winner() {
                    return v * input;
                }
            }
        }

        panic!("Did not terminate");
    }

    pub fn part2(input: &str) -> usize {
        let (input, mut boards) = parse(input);

        for input in input {
            boards.retain(|board| board.winner().is_none());
            let num_boards = boards.len();

            for board in &mut boards {
                board.consume(input);
                if let (Some(v), 1) = (board.winner(), num_boards) {
                    return v * input;
                }
            }
        }

        panic!("Did not terminate");
    }
}

fn main() -> std::io::Result<()> {
    println!("Day  1, part 1: {}", day1::part1(INPUT1));
    println!("Day  1, part 2: {}", day1::part2(INPUT1));
    println!("Day  2, part 1: {}", day2::part1(INPUT2));
    println!("Day  2, part 2: {}", day2::part2(INPUT2));
    println!("Day  3, part 1: {}", day3::part1(INPUT3));
    println!("Day  3, part 2: {}", day3::part2(INPUT3));
    println!("Day  4, part 1: {}", day4::part1(INPUT4));
    println!("Day  4, part 2: {}", day4::part2(INPUT4));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day3_part2() {
        let input = r##"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"##;
        assert_eq!(day3::part2(input), 230);
    }

    #[test]
    fn day4_part1() {
        let input = r##"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"##;

        assert_eq!(day4::part1(input), 4512);
    }
}
