use std::collections::{BTreeMap, HashMap, HashSet};

const INPUT1: &str = include_str!("day1.input");
const INPUT2: &str = include_str!("day2.input");
const INPUT3: &str = include_str!("day3.input");
const INPUT4: &str = include_str!("day4.input");
const INPUT5: &str = include_str!("day5.input");
const INPUT6: &str = include_str!("day6.input");
const INPUT7: &str = include_str!("day7.input");
const INPUT8: &str = include_str!("day8.input");
const INPUT9: &str = include_str!("day9.input");
const INPUT10: &str = include_str!("day10.input");
const INPUT11: &str = include_str!("day11.input");
const INPUT12: &str = include_str!("day12.input");

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

mod day5 {
    use crate::*;
    use std::cmp::{max, min};

    #[derive(Debug)]
    struct LineSegment {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        curr: (usize, usize),
    }

    impl From<&str> for LineSegment {
        fn from(item: &str) -> LineSegment {
            let mut leftright = item.split(" -> ");

            let left = leftright
                .next()
                .unwrap()
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let right = leftright
                .next()
                .unwrap()
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let left = (left[0], left[1]);
            let right = (right[0], right[1]);

            LineSegment {
                x1: min(left, right).0,
                y1: min(left, right).1,
                x2: max(left, right).0,
                y2: max(left, right).1,
                curr: (min(left, right).0, min(left, right).1),
            }
        }
    }

    impl Iterator for LineSegment {
        type Item = (usize, usize);

        fn next(&mut self) -> Option<Self::Item> {
            let this = self.curr;
            self.curr = if self.x1 == self.x2 {
                (self.curr.0, self.curr.1 + 1)
            } else if self.y1 == self.y2 {
                (self.curr.0 + 1, self.curr.1)
            } else if self.y2 > self.y1 {
                (self.curr.0 + 1, self.curr.1 + 1)
            } else {
                (self.curr.0 + 1, self.curr.1.wrapping_sub(1))
            };

            if this.0 <= max(self.x1, self.x2) && this.1 <= max(self.y1, self.y2) {
                return Some(this);
            }

            None
        }
    }

    impl LineSegment {
        fn is_axis_aligned(&self) -> bool {
            self.x1 == self.x2 || self.y1 == self.y2
        }
    }

    fn parse(input: &str) -> Vec<LineSegment> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(LineSegment::from)
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        let mut map = HashMap::new();
        parse(input)
            .into_iter()
            .filter(LineSegment::is_axis_aligned)
            .map(LineSegment::into_iter)
            .flatten()
            .for_each(|cell| {
                map.entry(cell).and_modify(|v| *v += 1).or_insert(1);
            });

        map.retain(|_, val| *val > 1);

        map.len()
    }

    pub fn part2(input: &str) -> usize {
        let mut map = HashMap::new();
        parse(input)
            .into_iter()
            .map(LineSegment::into_iter)
            .flatten()
            .for_each(|cell| {
                map.entry(cell).and_modify(|v| *v += 1).or_insert(1);
            });

        map.retain(|_, val| *val > 1);

        map.len()
    }
}

mod day6 {
    use crate::*;

    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
    }

    pub fn part1(input: &str, days: usize) -> usize {
        let mut map = HashMap::new();
        parse(input).into_iter().for_each(|days| {
            map.entry(days).and_modify(|v| *v += 1).or_insert(1);
        });

        let mut swap = HashMap::new();
        (0..days).for_each(|_| {
            map.iter().for_each(|(days, count)| match days {
                0 => {
                    swap.entry(8)
                        .and_modify(|v: &mut usize| *v += *count)
                        .or_insert(*count);
                    swap.entry(6)
                        .and_modify(|v: &mut usize| *v += *count)
                        .or_insert(*count);
                }
                v => {
                    swap.entry(v - 1)
                        .and_modify(|z: &mut usize| *z += *count)
                        .or_insert(*count);
                }
            });

            std::mem::swap(&mut map, &mut swap);

            swap.clear();
        });

        map.iter()
            .fold(0, |cumu, (_iter_key, iter_val)| cumu + iter_val)
    }

    pub fn part2(input: &str) -> usize {
        part1(input, 256)
    }
}

mod day7 {
    use crate::*;
    use std::cmp::{max, min};

    trait Euclidian1 {
        fn distance(&self, other: usize) -> usize;
    }

    impl Euclidian1 for usize {
        fn distance(&self, other: usize) -> usize {
            max(self, &other) - min(self, &other)
        }
    }

    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        // Minimize cost(pos) = sum{0position : 0positions}((0position - pos) * count_at[0position])
        let mut map = BTreeMap::new();
        parse(input).into_iter().for_each(|v| {
            map.entry(v)
                .and_modify(|v: &mut usize| *v += 1)
                .or_insert(1);
        });

        let max = *map.keys().max().unwrap();

        (0..=max)
            .map(|idx| {
                map.iter()
                    .map(|(pos0, count)| pos0.distance(idx) * count)
                    .sum()
            })
            .min()
            .unwrap()
    }

    pub fn part2(input: &str) -> usize {
        // Minimize cost(pos) = sum{0position : 0positions}(dist_fn(0position - pos) * count_at[0position])
        let mut map = BTreeMap::new();
        parse(input).into_iter().for_each(|v| {
            map.entry(v)
                .and_modify(|v: &mut usize| *v += 1)
                .or_insert(1);
        });

        let max = *map.keys().max().unwrap();

        // Arithmetic series
        let dist_fn = |dist| dist * (dist + 1) / 2;

        (0..=max)
            .map(|idx| {
                map.iter()
                    .map(|(pos0, count)| dist_fn(pos0.distance(idx)) * count)
                    .sum()
            })
            .min()
            .unwrap()
    }
}

mod day8 {
    use crate::*;
    struct DigitSignalDecoder<'a> {
        digit_encodings: HashMap<&'a DigitSignal, u8>,
    }

    impl<'a> DigitSignalDecoder<'a> {
        fn new(signals: &'a [DigitSignal]) -> Self {
            let mut digit_encodings = HashMap::new();

            signals.iter().for_each(|signal| match signal.segments() {
                2 => {
                    digit_encodings.insert(signal, 1);
                }
                3 => {
                    digit_encodings.insert(signal, 7);
                }
                4 => {
                    digit_encodings.insert(signal, 4);
                }
                7 => {
                    digit_encodings.insert(signal, 8);
                }
                _ => {}
            });

            // Rules:
            // - Signals with 5 segments: { 2, 3, 5 }. The one that has matches '1' signal's segments is 3.
            // - Signals with 6 segments: { 0, 6, 9 }. The '3' signal plus one other segment is 9. That segment is 'b'.
            // - The 5-segment signal with 'b' is 5. The remaining 5-segment signal is 2.
            // - The 6-segment signal that is not nine and has all of 5's segments is 6.
            // - The remaining signal is 0.

            let sig3 = signals
                .iter()
                .filter(|signal| signal.segments() == 5)
                .find(|signal| {
                    let sig1 = digit_encodings
                        .iter()
                        .find(|(_, val)| **val == 1)
                        .unwrap()
                        .0;

                    sig1.data.iter().all(|seg| signal.data.contains(seg))
                })
                .unwrap();

            digit_encodings.insert(sig3, 3);

            let sig9 = signals
                .iter()
                .filter(|signal| signal.segments() == 6)
                .find(|signal| sig3.data.iter().all(|seg| signal.data.contains(seg)))
                .unwrap();

            digit_encodings.insert(sig9, 9);

            let sig9_set = sig9.data.iter().collect::<HashSet<_>>();
            let sig3_set = sig3.data.iter().collect::<HashSet<_>>();
            let b = sig9_set.difference(&sig3_set).into_iter().next().unwrap();

            let sig5 = signals
                .iter()
                .filter(|signal| signal.segments() == 5)
                .find(|signal| signal.data.iter().any(|seg| seg == *b))
                .unwrap();

            digit_encodings.insert(sig5, 5);

            let sig2 = signals
                .iter()
                .filter(|signal| signal.segments() == 5)
                .find(|signal| *signal != sig3 && *signal != sig5)
                .unwrap();

            digit_encodings.insert(sig2, 2);

            let sig6 = signals
                .iter()
                .filter(|signal| signal.segments() == 6 && *signal != sig9)
                .find(|signal| sig5.data.iter().all(|seg| signal.data.contains(seg)))
                .unwrap();

            digit_encodings.insert(sig6, 6);

            let sig0 = signals
                .iter()
                .find(|signal| digit_encodings.get(signal).is_none())
                .unwrap();

            digit_encodings.insert(sig0, 0);

            DigitSignalDecoder { digit_encodings }
        }

        fn decode(&mut self, outputs: &'a [DigitSignal]) -> usize {
            let vals = outputs
                .iter()
                .map(|signal| self.digit_encodings.get(signal).unwrap())
                .collect::<Vec<_>>();

            *vals[0] as usize * 1000
                + *vals[1] as usize * 100
                + *vals[2] as usize * 10
                + *vals[3] as usize
        }
    }

    #[derive(PartialEq, Hash, Eq, Debug)]
    struct DigitSignal {
        data: Vec<char>,
    }

    impl From<Vec<char>> for DigitSignal {
        fn from(data: Vec<char>) -> Self {
            DigitSignal { data }
        }
    }

    impl DigitSignal {
        fn segments(&self) -> usize {
            self.data.len()
        }
    }

    fn parse(input: &str) -> Vec<(Vec<DigitSignal>, Vec<DigitSignal>)> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut halves = line.split(" | ");

                let into_digital_signal = |str: &str| {
                    let mut bytes = str.chars().collect::<Vec<_>>();

                    bytes.sort_unstable();

                    bytes.into()
                };

                let lhs = halves
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(into_digital_signal)
                    .collect::<Vec<_>>();
                let rhs = halves
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(into_digital_signal)
                    .collect::<Vec<_>>();

                (lhs, rhs)
            })
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        parse(input)
            .into_iter()
            .map(|(_, output)| output)
            .map(|output| {
                output
                    .into_iter()
                    .filter(|v| [2, 4, 3, 7].iter().any(|z| v.segments() == *z))
            })
            .flatten()
            .count()
    }

    pub fn part2(input: &str) -> usize {
        parse(input)
            .into_iter()
            .map(|(signals, outputs)| {
                let mut decoder = DigitSignalDecoder::new(&signals);

                decoder.decode(&outputs)
            })
            .sum()
    }
}

mod day9 {
    use crate::*;
    use std::collections::BTreeSet;

    type Index = (usize, usize);

    trait Paddable<T, U> {
        fn padded(&self, pad: U) -> T;
    }

    impl Paddable<Vec<u8>, u8> for Vec<u8> {
        fn padded(&self, pad: u8) -> Vec<u8> {
            [pad]
                .iter()
                .chain(self.iter())
                .chain([pad].iter())
                .cloned()
                .collect()
        }
    }

    impl Paddable<Vec<Vec<u8>>, u8> for Vec<Vec<u8>> {
        fn padded(&self, pad: u8) -> Vec<Vec<u8>> {
            let inner_padded = self.iter().map(|line| line.padded(9)).collect::<Vec<_>>();
            let len = inner_padded[0].len();

            vec![vec![pad; len]]
                .iter()
                .cloned()
                .chain(inner_padded)
                .chain(vec![vec![pad; len]].iter().cloned())
                .collect()
        }
    }

    fn parse(input: &str) -> Vec<Vec<u8>> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.bytes()
                    .map(|ch| std::str::from_utf8(&[ch]).unwrap().parse::<u8>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1_memo(
        memo: &mut HashMap<Index, (Index, u8)>,
        input: &[Vec<u8>],
        idx: Index,
    ) -> (Index, u8) {
        if memo.contains_key(&idx) {
            return *memo.get(&idx).unwrap();
        }

        memo.entry(idx).or_insert((idx, 9));

        let max_idx = (input.len() - 1, input[0].len() - 1);

        let local_min = [
            (idx.0 - 1, idx.1),
            (idx.0 + 1, idx.1),
            (idx.0, idx.1 - 1),
            (idx.0, idx.1 + 1),
        ]
        .iter()
        .filter(|neighbor_index| {
            neighbor_index.0 > 0
                && neighbor_index.1 > 0
                && neighbor_index.0 < max_idx.0
                && neighbor_index.1 < max_idx.1
                && input[neighbor_index.0][neighbor_index.1] <= input[idx.0][idx.1]
        })
        .map(|idx| part1_memo(memo, input, *idx))
        .min_by(|v1, v2| v1.1.cmp(&v2.1))
        .unwrap_or_else(|| ((idx.0, idx.1), input[idx.0][idx.1]));

        memo.insert(idx, local_min);

        local_min
    }

    fn calculate_mins_and_padded(input: &str) -> (HashSet<Index>, Vec<Vec<u8>>) {
        let mut memo: HashMap<Index, (Index, u8)> = HashMap::new();

        let padded = parse(input).padded(9);

        let mut mins = HashSet::new();

        (1..padded.len() - 1).for_each(|ii| {
            (1..padded[0].len() - 1).for_each(|jj| {
                mins.insert(part1_memo(&mut memo, &padded, (ii, jj)).0);
            });
        });

        (mins, padded)
    }

    pub fn part1(input: &str) -> usize {
        let (mins, padded) = calculate_mins_and_padded(input);

        mins.into_iter()
            .map(|loc| padded[loc.0][loc.1] as usize + 1)
            .sum()
    }

    pub fn part2(input: &str) -> usize {
        let (mins, padded) = calculate_mins_and_padded(input);

        let max_idx = (padded.len(), padded[0].len());

        let mut basin_sizes = mins
            .into_iter()
            .map(|min| {
                let mut working_set = HashSet::new();
                let mut working_frontier = BTreeSet::new();
                working_frontier.insert(min);

                while !working_frontier.is_empty() {
                    let popped = *{
                        let v = working_frontier.iter().next().unwrap();

                        v
                    };

                    working_frontier.remove(&popped);
                    working_set.insert(popped);

                    [
                        (popped.0 - 1, popped.1),
                        (popped.0 + 1, popped.1),
                        (popped.0, popped.1 - 1),
                        (popped.0, popped.1 + 1),
                    ]
                    .iter()
                    .filter(|idx| {
                        idx.0 > 0
                            && idx.1 > 0
                            && idx.0 < max_idx.0
                            && idx.1 < max_idx.1
                            && !working_set.contains(idx)
                            && padded[idx.0][idx.1] < 9
                    })
                    .for_each(|idx| {
                        working_frontier.insert(*idx);
                    })
                }

                working_set.len()
            })
            .collect::<Vec<_>>();

        basin_sizes.sort_unstable();

        let mut iter = basin_sizes.iter().rev();

        let (large0, large1, large2) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        large0 * large1 * large2
    }
}

mod day10 {
    fn parse(input: &str) -> Vec<&str> {
        input.split('\n').filter(|line| !line.is_empty()).collect()
    }

    trait ParenMatchable<T> {
        fn open_for(&self) -> Option<&T>;
        fn close_for(&self) -> Option<&T>;
    }

    impl ParenMatchable<char> for char {
        fn open_for(&self) -> Option<&char> {
            match self {
                ')' => Some(&'('),
                ']' => Some(&'['),
                '}' => Some(&'{'),
                '>' => Some(&'<'),
                _ => None,
            }
        }

        fn close_for(&self) -> Option<&char> {
            match self {
                '(' => Some(&')'),
                '[' => Some(&']'),
                '{' => Some(&'}'),
                '<' => Some(&'>'),
                _ => None,
            }
        }
    }

    fn determine_corruptions(input: &str) -> Vec<(Vec<char>, Option<char>)> {
        parse(input)
            .into_iter()
            .map(|line| {
                line.chars()
                    .fold((vec![], None), |(mut stack, status), ch| {
                        if status.is_some() {
                            return (stack, status);
                        }

                        match ch {
                            '(' | '{' | '<' | '[' => {
                                stack.push(ch);
                                (stack, None)
                            }
                            v if stack.is_empty()
                                || (stack.last().is_some() && stack.last() == v.open_for()) =>
                            {
                                stack.pop();
                                (stack, None)
                            }
                            _ => (stack, Some(ch)),
                        }
                    })
            })
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        determine_corruptions(input)
            .into_iter()
            .filter_map(|(_, v)| v)
            .map(|v| match v {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unexpected corruption"),
            })
            .sum()
    }

    pub fn part2(input: &str) -> usize {
        let mut scores = determine_corruptions(input)
            .into_iter()
            .filter(|(_, corruption)| corruption.is_none())
            .map(|(stack, _)| stack)
            .map(|stack| {
                stack
                    .into_iter()
                    .rev()
                    .fold(0, |acc, ch| match ch.close_for() {
                        Some('>') => 5 * acc + 4,
                        Some(']') => 5 * acc + 2,
                        Some('}') => 5 * acc + 3,
                        Some(')') => 5 * acc + 1,
                        _ => panic!("Unexpected closing bracket"),
                    })
            })
            .collect::<Vec<_>>();

        scores.sort_unstable();

        scores[scores.len() / 2]
    }
}

mod day11 {
    use itertools::Itertools;
    use std::cell::RefCell;
    use std::convert::TryInto;

    struct Board {
        board: Vec<Vec<u8>>,
    }

    impl Board {
        fn sizes(&self) -> (usize, usize) {
            (self.board.len(), self.board[0].len())
        }

        fn count_flashes(&self) -> usize {
            let (y_len, x_len) = self.sizes();

            (0..y_len)
                .cartesian_product(0..x_len)
                .map(|(yy, xx)| self.board[yy][xx])
                .filter(|v| v >= &10)
                .count()
        }

        fn neighbors(&self, idx: (isize, isize)) -> Vec<(isize, isize)> {
            let (y_len, x_len) = self.sizes();

            [
                (idx.0 - 1, idx.1),
                (idx.0 + 1, idx.1),
                (idx.0, idx.1 - 1),
                (idx.0, idx.1 + 1),
                (idx.0 - 1, idx.1 + 1),
                (idx.0 - 1, idx.1 - 1),
                (idx.0 + 1, idx.1 + 1),
                (idx.0 + 1, idx.1 - 1),
            ]
            .iter()
            .filter(|neighbor_index| {
                neighbor_index.0 >= 0
                    && neighbor_index.0 < y_len.try_into().unwrap()
                    && neighbor_index.1 >= 0
                    && neighbor_index.1 < x_len.try_into().unwrap()
                    && self.board[neighbor_index.0 as usize][neighbor_index.1 as usize] < 10
            })
            .cloned()
            .collect()
        }

        fn step(&mut self) -> usize {
            let (y_len, x_len) = self.sizes();

            (0..y_len).cartesian_product(0..x_len).for_each(|(yy, xx)| {
                let mut frontier = vec![(yy, xx)];

                while let Some(idx) = frontier.pop() {
                    self.board[idx.0][idx.1] += 1;
                    if self.board[idx.0][idx.1] == 10 {
                        self.neighbors((idx.0 as isize, idx.1 as isize))
                            .into_iter()
                            .for_each(|idx| {
                                frontier.push((idx.0 as usize, idx.1 as usize));
                            });
                    }
                }
            });

            let flashes = self.count_flashes();

            (0..y_len).cartesian_product(0..x_len).for_each(|(yy, xx)| {
                if self.board[yy][xx] > 9 {
                    self.board[yy][xx] = 0;
                }
            });

            flashes
        }
    }

    fn parse(input: &str) -> Board {
        let board = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split("")
                    .filter(|ch| !ch.is_empty())
                    .map(|ch| ch.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();

        Board { board }
    }

    pub fn part1(input: &str) -> usize {
        let mut board = parse(input);

        (0..100).fold(0, |accum, _| accum + board.step())
    }

    pub fn part2(input: &str) -> usize {
        let board = RefCell::new(parse(input));

        (1..)
            .map(|idx| (idx, &board))
            .find(|(_, board)| {
                let mut board = board.borrow_mut();
                board.step() == board.sizes().0 * board.sizes().1
            })
            .unwrap()
            .0
    }
}

mod day12 {
    fn parse(input: &str) -> Vec<char> {
        unimplemented!()
    }

    pub fn part1(input: &str) -> usize {
        parse(input);

        0
    }

    pub fn part2(input: &str) -> usize {
        parse(input);

        0
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
    println!("Day  5, part 1: {}", day5::part1(INPUT5));
    println!("Day  5, part 2: {}", day5::part2(INPUT5));
    println!("Day  6, part 1: {}", day6::part1(INPUT6, 80));
    println!("Day  6, part 2: {}", day6::part2(INPUT6));
    println!("Day  7, part 1: {}", day7::part1(INPUT7));
    println!("Day  7, part 2: {}", day7::part2(INPUT7));
    println!("Day  8, part 1: {}", day8::part1(INPUT8));
    println!("Day  8, part 2: {}", day8::part2(INPUT8));
    println!("Day  9, part 1: {}", day9::part1(INPUT9));
    println!("Day  9, part 2: {}", day9::part2(INPUT9));
    println!("Day 10, part 1: {}", day10::part1(INPUT10));
    println!("Day 10, part 2: {}", day10::part2(INPUT10));
    println!("Day 11, part 1: {}", day11::part1(INPUT11));
    println!("Day 11, part 2: {}", day11::part2(INPUT11));
    println!("Day 12, part 1: {}", day12::part1(INPUT12));
    println!("Day 12, part 2: {}", day12::part2(INPUT12));

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

    #[test]
    fn day5_part1() {
        let input = r##"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"##;
        assert_eq!(day5::part1(input), 5);
    }

    #[test]
    fn day5_part2() {
        let input = r##"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"##;
        assert_eq!(day5::part2(input), 12);
    }

    #[test]
    fn day9_part1() {
        let input = r##"2199943210
3987894921
9856789892
8767896789
9899965678
"##;

        assert_eq!(day9::part1(input), 15);
    }

    #[test]
    fn day9_part2() {
        let input = r##"2199943210
3987894921
9856789892
8767896789
9899965678"##;

        assert_eq!(day9::part2(input), 1134);
    }

    #[test]
    fn day10_part1() {
        let input = r##"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"##;

        assert_eq!(day10::part1(input), 26397);
    }

    #[test]
    fn day11_part1() {
        let input = r##"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"##;

        assert_eq!(day11::part1(input), 1656);
    }

    #[test]
    fn day11_part2() {
        let input = r##"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"##;

        assert_eq!(day11::part2(input), 195);
    }
}
