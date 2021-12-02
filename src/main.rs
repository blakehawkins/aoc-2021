const INPUT1: &str = include_str!("day1.input");
const INPUT2: &str = include_str!("day2.input");

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

fn main() -> std::io::Result<()> {
    println!("Day  1, part 1: {}", day1::part1(INPUT1));
    println!("Day  1, part 2: {}", day1::part2(INPUT1));
    println!("Day  2, part 1: {}", day2::part1(INPUT2));
    println!("Day  2, part 2: {}", day2::part2(INPUT2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
}
