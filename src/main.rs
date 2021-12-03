const INPUT1: &str = include_str!("day1.input");
const INPUT2: &str = include_str!("day2.input");
const INPUT3: &str = include_str!("day3.input");

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

fn main() -> std::io::Result<()> {
    println!("Day  1, part 1: {}", day1::part1(INPUT1));
    println!("Day  1, part 2: {}", day1::part2(INPUT1));
    println!("Day  2, part 1: {}", day2::part1(INPUT2));
    println!("Day  2, part 2: {}", day2::part2(INPUT2));
    println!("Day  3, part 1: {}", day3::part1(INPUT3));
    println!("Day  3, part 2: {}", day3::part2(INPUT3));

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
}
