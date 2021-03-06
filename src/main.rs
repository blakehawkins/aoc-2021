use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

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
const INPUT13: &str = include_str!("day13.input");
const INPUT14: &str = include_str!("day14.input");
const INPUT15: &str = include_str!("day15.input");
const INPUT16: &str = include_str!("day16.input");
const INPUT17: &str = include_str!("day17.input");
const INPUT18: &str = include_str!("day18.input");
const INPUT19: &str = include_str!("day19.input");
const INPUT20: &str = include_str!("day20.input");
const INPUT21: &str = include_str!("day21.input");
const INPUT22: &str = include_str!("day22.input");
const INPUT23: &str = include_str!("day23.input");
const INPUT24: &str = include_str!("day24.input");
const INPUT25: &str = include_str!("day25.input");

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
    use crate::*;
    use petgraph::graphmap::UnGraphMap;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    enum CaveCell {
        Start,
        End,
        Big((u8, u8)),
        Small((u8, u8)),
    }

    trait CaveCellTakeable {
        fn can_take(&self, other: &CaveCell) -> bool;
    }

    impl CaveCellTakeable for Vec<CaveCell> {
        fn can_take(&self, other: &CaveCell) -> bool {
            match *other {
                CaveCell::Start => true,
                CaveCell::End => false,
                CaveCell::Big(_) => true,
                CaveCell::Small(_) => {
                    let mut sorted = self
                        .iter()
                        .chain([other])
                        .filter(|it| matches!(it, CaveCell::Small(_)))
                        .collect::<Vec<_>>();

                    sorted.sort();

                    let result = sorted
                        .windows(2)
                        .filter(|cells| cells[0] == cells[1])
                        .count()
                        < 2;

                    result
                }
            }
        }
    }

    impl From<Vec<u8>> for CaveCell {
        fn from(item: Vec<u8>) -> CaveCell {
            match item {
                start if start == "start".bytes().collect::<Vec<_>>() => CaveCell::Start,
                end if end == "end".bytes().collect::<Vec<_>>() => CaveCell::End,
                v if v.iter().all(|by| (*by as char).is_uppercase()) => CaveCell::Big((v[0], v[1])),
                v => CaveCell::Small((v[0], v[1])),
            }
        }
    }

    impl Default for CaveCell {
        fn default() -> Self {
            CaveCell::Start
        }
    }

    struct CaveSystem {
        graph: UnGraphMap<CaveCell, ()>,
    }

    fn parse(input: &str) -> CaveSystem {
        let edges = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut split = line.split('-');

                (
                    split.next().unwrap().bytes().collect::<Vec<_>>().into(),
                    split.next().unwrap().bytes().collect::<Vec<_>>().into(),
                )
            })
            .collect::<Vec<(CaveCell, CaveCell)>>();

        let graph = UnGraphMap::<CaveCell, ()>::from_edges(edges);

        CaveSystem { graph }
    }

    pub fn part1(input: &str) -> usize {
        let graph = parse(input).graph;

        let mut resolved_paths = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(vec![CaveCell::End]);

        while let Some(unfinished_path) = queue.pop_front() {
            graph
                .neighbors(*unfinished_path.last().unwrap())
                .filter(|neighbor| match neighbor {
                    CaveCell::Big(_) => true,
                    n => !unfinished_path.contains(n),
                })
                .for_each(|neighbor| match neighbor {
                    CaveCell::Start => {
                        resolved_paths.insert(unfinished_path.clone());
                    }
                    non_terminal_neighbor => {
                        let mut copy = unfinished_path.clone();
                        copy.push(non_terminal_neighbor);
                        queue.push_back(copy);
                    }
                });
        }

        resolved_paths.len()
    }

    pub fn part2(input: &str) -> usize {
        let graph = parse(input).graph;

        let mut resolved_paths = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(vec![CaveCell::End]);

        while let Some(unfinished_path) = queue.pop_front() {
            graph
                .neighbors(*unfinished_path.last().unwrap())
                .filter(|neighbor| match neighbor {
                    CaveCell::Big(_) => true,
                    n => unfinished_path.can_take(n),
                })
                .for_each(|neighbor| match neighbor {
                    CaveCell::Start => {
                        resolved_paths.insert(unfinished_path.clone());
                    }
                    non_terminal_neighbor => {
                        let mut copy = unfinished_path.clone();
                        copy.push(non_terminal_neighbor);
                        queue.push_back(copy);
                    }
                });
        }

        resolved_paths.len()
    }
}

mod day13 {
    use itertools::Itertools;

    enum FoldAxis {
        X,
        Y,
    }

    impl From<char> for FoldAxis {
        fn from(item: char) -> Self {
            match item {
                'x' => FoldAxis::X,
                'y' => FoldAxis::Y,
                _ => panic!("Unexpected fold axis"),
            }
        }
    }

    struct FoldSpec {
        axis: FoldAxis,
        _offset: usize,
    }

    impl From<&str> for FoldSpec {
        fn from(item: &str) -> Self {
            FoldSpec {
                axis: item.chars().next().unwrap().into(),
                _offset: item[2..].parse().unwrap(),
            }
        }
    }

    trait PaperFoldable<T> {
        fn paper_fold(&self, foldspec: &FoldSpec) -> T;

        fn count_ones(&self) -> usize;
    }

    impl PaperFoldable<Vec<Vec<usize>>> for Vec<Vec<usize>> {
        fn paper_fold(&self, foldspec: &FoldSpec) -> Vec<Vec<usize>> {
            let (x_dim, y_dim) = (self.len(), self[0].len());

            let mat = |x_dim, y_dim| {
                let mut vec = Vec::new();

                while vec.len() < x_dim {
                    vec.push(vec![0; y_dim]);
                }

                vec
            };

            match foldspec.axis {
                FoldAxis::X => {
                    let mut mat = mat(x_dim / 2, y_dim);

                    (0..(x_dim / 2))
                        .cartesian_product(0..y_dim)
                        .for_each(|(xx, yy)| {
                            mat[xx][yy] = self[xx][yy] | self[x_dim - xx - 1][yy];
                        });

                    mat
                }
                FoldAxis::Y => {
                    let mut mat = mat(x_dim, y_dim / 2);

                    (0..x_dim)
                        .cartesian_product(0..(y_dim / 2))
                        .for_each(|(xx, yy)| {
                            mat[xx][yy] = self[xx][yy] | self[xx][y_dim - yy - 1];
                        });

                    mat
                }
            }
        }

        fn count_ones(&self) -> usize {
            self.iter().map(|vec| vec.iter().sum::<usize>()).sum()
        }
    }

    fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<FoldSpec>) {
        let mut halves = input.split("\n\n");

        let mut matrix = halves
            .next()
            .unwrap()
            .split('\n')
            .map(|line| {
                let mut vals = line.split(',').map(|v| v.parse::<usize>().unwrap());

                (vals.next().unwrap(), vals.next().unwrap())
            })
            .fold(vec![], |mut accum, iter| {
                while iter.0 >= accum.len() {
                    accum.push(vec![0; iter.1]);
                }

                if iter.1 >= accum[iter.0].len() {
                    accum[iter.0].resize(iter.1 + 1, 0);
                }

                accum[iter.0][iter.1] = 1;

                accum
            });

        let largest = matrix.iter().map(|vec| vec.len()).max().unwrap();

        matrix.iter_mut().for_each(|vec| {
            if vec.len() < largest {
                vec.resize(largest, 0);
            }
        });

        let fold_specs = halves
            .next()
            .unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|str| &str["fold along ".len()..])
            .map(FoldSpec::from)
            .collect();

        (matrix, fold_specs)
    }

    pub fn part1(input: &str) -> usize {
        let (matrix, folds) = parse(input);

        matrix.paper_fold(&folds[0]).count_ones()
    }

    pub fn part2(input: &str) -> usize {
        let (matrix, folds) = parse(input);

        let result = folds
            .into_iter()
            .fold(matrix, |accum, iter| accum.paper_fold(&iter));

        println!("{:?}", result);

        0
    }
}

mod day14 {
    use crate::*;
    use itertools::Itertools;

    fn parse(input: &str) -> (Vec<u8>, HashMap<(u8, u8), u8>) {
        let mut halves = input.split("\n\n");

        let p0 = halves.next().unwrap().bytes().collect();

        let rules = halves
            .next()
            .unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut halves = line.split(" -> ");

                let mut ab = halves.next().unwrap().bytes();

                (
                    (ab.next().unwrap(), ab.next().unwrap()),
                    halves.next().unwrap().bytes().next().unwrap(),
                )
            })
            .collect();

        (p0, rules)
    }

    pub fn part1(input: &str, iterations: usize) -> usize {
        let (input, rules) = parse(input);

        let mut working_set: HashMap<(u8, u8), usize> = HashMap::new();
        let mut counts = HashMap::new();

        input.iter().tuple_windows::<(_, _)>().for_each(|(a, b)| {
            working_set
                .entry((*a, *b))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });

        input.iter().for_each(|ch| {
            counts.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        });

        (0..iterations).for_each(|_| {
            let mut swap = HashMap::new();

            working_set
                .iter()
                .for_each(|((a, b), count)| match rules.get(&(*a, *b)) {
                    Some(ch) => {
                        swap.entry((*a, *ch))
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                        swap.entry((*ch, *b))
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                        counts
                            .entry(ch)
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                    }
                    None => {
                        swap.entry((*a, *b))
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                    }
                });

            std::mem::swap(&mut working_set, &mut swap);
        });

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    pub fn part2(input: &str) -> usize {
        part1(input, 40)
    }
}

mod day15 {
    use itertools::Itertools;
    use petgraph::algo::dijkstra::dijkstra;
    use petgraph::graphmap::DiGraphMap;

    trait RiskIncrementable<T> {
        fn risk_inc(&mut self, amt: T);
    }

    impl RiskIncrementable<u8> for u8 {
        fn risk_inc(&mut self, amt: u8) {
            *self += amt;
            if *self >= 10 {
                *self -= 9;
            }
        }
    }

    impl RiskIncrementable<u8> for Vec<u8> {
        fn risk_inc(&mut self, amt: u8) {
            self.iter_mut().map(|v| v.risk_inc(amt)).collect()
        }
    }

    type Matrix = Vec<Vec<u8>>;

    fn parse(input: &str, tesselations: usize) -> (Matrix, DiGraphMap<(usize, usize), u8>) {
        let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();

        let line_count = lines.len();

        let mut matrix: Matrix = lines
            .into_iter()
            .map(|line| {
                line.bytes()
                    .map(|b| (b as usize - '0' as usize) as u8)
                    .collect::<Vec<_>>()
            })
            .map(|mut line| {
                let len = line.len();
                line.resize(len * tesselations, 0);

                (len..(len * tesselations)).for_each(|idx| {
                    line[idx] = line[idx - len];
                    line[idx].risk_inc(1);
                });

                line
            })
            .cycle()
            .take(line_count * tesselations)
            .collect();

        (line_count..(line_count * tesselations)).for_each(|idx| {
            matrix[idx].risk_inc((idx / line_count) as u8);
        });

        let edges = (0isize..matrix.len() as isize)
            .cartesian_product(0isize..matrix[0].len() as isize)
            .map(|(xx, yy)| {
                [(xx - 1, yy), (xx, yy - 1), (xx + 1, yy), (xx, yy + 1)]
                    .iter()
                    .filter(|(xx, yy)| {
                        *xx >= 0
                            && *xx < matrix.len() as isize
                            && *yy >= 0
                            && *yy < matrix[0].len() as isize
                    })
                    .map(|neigh| {
                        (
                            (xx as usize, yy as usize),
                            (neigh.0 as usize, neigh.1 as usize),
                            matrix[xx as usize][yy as usize],
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<((usize, usize), (usize, usize), u8)>>();

        (matrix, DiGraphMap::from_edges(edges.iter()))
    }

    pub fn part1(input: &str) -> usize {
        let (matrix, graph) = parse(input, 1);

        let end = (matrix.len() - 1, matrix[0].len() - 1);
        *dijkstra(&graph, end, Some((0, 0)), |e| *e.2 as usize)
            .get(&(0, 0))
            .unwrap()
    }

    pub fn part2(input: &str) -> usize {
        let (matrix, graph) = parse(input, 5);

        let end = (matrix.len() - 1, matrix[0].len() - 1);
        *dijkstra(&graph, end, Some((0, 0)), |e| *e.2 as usize)
            .get(&(0, 0))
            .unwrap()
    }
}

mod day16 {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map_res,
        error::context,
        multi::{length_count, length_value, many0, many1, many_till},
        sequence::{pair, preceded, terminated},
        IResult,
    };

    #[derive(Debug, PartialEq)]
    pub struct Operator {
        pub id: usize,
        pub sub_packets: Vec<Packet>,
    }

    impl Operator {
        fn calculate(&self) -> usize {
            let mut sub_calculated = self.sub_packets.iter().map(Packet::calculate);

            match self.id {
                0 => sub_calculated.sum(),
                1 => sub_calculated.product(),
                2 => sub_calculated.min().unwrap(),
                3 => sub_calculated.max().unwrap(),
                5 => {
                    let next = sub_calculated.next().unwrap();
                    let nnext = sub_calculated.next().unwrap();

                    if next > nnext {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let next = sub_calculated.next().unwrap();
                    let nnext = sub_calculated.next().unwrap();

                    if next < nnext {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let next = sub_calculated.next().unwrap();
                    let nnext = sub_calculated.next().unwrap();

                    if next == nnext {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unexpected id"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum PacketBody {
        Literal(usize),
        Operator(Operator),
    }

    #[derive(Debug, PartialEq)]
    pub struct Packet {
        pub version: usize,
        pub type_: PacketBody,
    }

    struct Bits(Vec<u8>);

    impl std::fmt::Display for Bits {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|v| match v {
                        0 => '0',
                        1 => '1',
                        _ => panic!("Unexpected bit"),
                    })
                    .collect::<String>()
            )
        }
    }

    impl Packet {
        fn new(version: usize, type_: PacketBody) -> Self {
            Packet { version, type_ }
        }

        fn sum_version_numbers(&self) -> usize {
            self.version
                + match &self.type_ {
                    PacketBody::Operator(op) => op
                        .sub_packets
                        .iter()
                        .map(|pkt| pkt.sum_version_numbers())
                        .sum(),
                    _ => 0,
                }
        }

        fn calculate(&self) -> usize {
            match &self.type_ {
                PacketBody::Operator(op) => op.calculate(),
                PacketBody::Literal(v) => *v,
            }
        }
    }

    pub fn int_from_nbits_parse(input: &[u8], len: usize) -> IResult<&[u8], usize> {
        map_res(
            context("int_from_nbits", take(len)),
            |input: &[u8]| -> Result<usize, ()> {
                Ok((0..len).rev().enumerate().fold(0usize, |accum, iter| {
                    accum + ((input[iter.1] as usize) << iter.0)
                }))
            },
        )(input)
    }

    pub fn int_from_3bits_parse(input: &[u8]) -> IResult<&[u8], usize> {
        map_res(
            context("int_from_3bits", take(3usize)),
            |v: &[u8]| -> Result<usize, ()> {
                Ok((((v[0] as usize) << 2) + ((v[1] as usize) << 1) + (v[2] as usize)) as usize)
            },
        )(input)
    }

    fn one_padded_5b_parse(input: &[u8]) -> IResult<&[u8], &[u8]> {
        context("one_padded_5b", preceded(tag([1u8]), take(4usize)))(input)
    }

    fn zero_padded_5b_parse(input: &[u8]) -> IResult<&[u8], &[u8]> {
        context("zero_padded_5b", preceded(tag([0u8]), take(4usize)))(input)
    }

    fn literal_parse(input: &[u8]) -> IResult<&[u8], usize> {
        map_res(
            context(
                "literal_parse",
                many_till(one_padded_5b_parse, zero_padded_5b_parse),
            ),
            |(parts, final_part)| -> Result<usize, ()> {
                let res = final_part
                    .iter()
                    .rev()
                    .chain(
                        parts
                            .iter()
                            .rev()
                            .map(|four_bits| four_bits.iter().rev())
                            .flatten(),
                    )
                    .enumerate()
                    .fold(0usize, |accum, iter| accum + ((*iter.1 as usize) << iter.0));

                Ok(res)
            },
        )(input)
    }

    fn body_literal_parse(input: &[u8]) -> IResult<&[u8], PacketBody> {
        map_res(
            context("body_literal", preceded(tag([1, 0, 0]), literal_parse)),
            |v| -> Result<PacketBody, ()> { Ok(PacketBody::Literal(v)) },
        )(input)
    }

    fn packets_parse(input: &[u8]) -> IResult<&[u8], Vec<Packet>> {
        context("packets", many1(packet_parse))(input)
    }

    pub fn body_operator_by_total_length_parse(input: &[u8]) -> IResult<&[u8], Vec<Packet>> {
        context(
            "body_operator_by_total_length",
            length_value(|input| int_from_nbits_parse(input, 15), packets_parse),
        )(input)
    }

    fn body_operator_by_num_child_packets(input: &[u8]) -> IResult<&[u8], Vec<Packet>> {
        context(
            "body_operator_by_num_child_packets",
            length_count(|input| int_from_nbits_parse(input, 11), packet_parse),
        )(input)
    }

    fn body_operator_parse_with_size(input: &[u8]) -> IResult<&[u8], Vec<Packet>> {
        context(
            "body_operator_parse_with_size",
            alt((
                preceded(tag([0]), body_operator_by_total_length_parse),
                preceded(tag([1]), body_operator_by_num_child_packets),
            )),
        )(input)
    }

    fn body_operator_parse(input: &[u8]) -> IResult<&[u8], PacketBody> {
        map_res(
            context(
                "body_operator",
                pair(
                    int_from_3bits_parse, // Version
                    body_operator_parse_with_size,
                ),
            ),
            |(id, sub_packets)| -> Result<PacketBody, ()> {
                Ok(PacketBody::Operator(Operator { id, sub_packets }))
            },
        )(input)
    }

    fn body_parse(input: &[u8]) -> IResult<&[u8], PacketBody> {
        context("body", alt((body_literal_parse, body_operator_parse)))(input)
    }

    // Parse slice of bits (each encoded as a sparse u8) into a packet.
    pub fn packet_parse(input: &[u8]) -> IResult<&[u8], Packet> {
        map_res(
            context(
                "packet",
                pair(
                    int_from_3bits_parse, // Version
                    body_parse,
                ),
            ),
            |(version, type_)| -> Result<Packet, ()> { Ok(Packet::new(version, type_)) },
        )(input)
    }

    fn packet_parse_null_terminated(input: &[u8]) -> IResult<&[u8], Packet> {
        context(
            "packet_parse_null_terminated",
            terminated(packet_parse, many0(tag([0]))),
        )(input)
    }

    pub fn parse_bits(input: &str) -> Vec<u8> {
        input
            .bytes()
            .map(|by| u8::from_str_radix(std::str::from_utf8(&[by]).unwrap(), 16).unwrap())
            .map(|u8| {
                [
                    (u8 & 0b1000) >> 3,
                    (u8 & 0b0100) >> 2,
                    (u8 & 0b0010) >> 1,
                    (u8 & 0b0001),
                ]
            })
            .flatten()
            .collect()
    }

    pub fn parse(input: &str) -> Packet {
        let bits = parse_bits(input);

        let v = packet_parse_null_terminated(&bits);

        v.unwrap().1
    }

    pub fn part1(input: &str) -> usize {
        parse(input).sum_version_numbers()
    }

    pub fn part2(input: &str) -> usize {
        parse(input).calculate()
    }
}

mod day17 {
    use itertools::Itertools;

    struct Area {
        x0: isize,
        x1: isize,
        y0: isize,
        y1: isize,
    }

    #[derive(Clone, Debug)]
    struct Vec2 {
        x: isize,
        y: isize,
    }

    type Point = Vec2;

    trait MeasurableHeight {
        fn height(&self) -> isize;
    }

    impl MeasurableHeight for Point {
        fn height(&self) -> isize {
            self.y
        }
    }

    impl MeasurableHeight for Vec<Point> {
        fn height(&self) -> isize {
            self.iter().map(Point::height).max().unwrap()
        }
    }

    impl Vec2 {
        fn new(x: isize, y: isize) -> Self {
            Vec2 { x, y }
        }
    }

    impl Area {
        fn new(x0: isize, x1: isize, y0: isize, y1: isize) -> Self {
            Area { x0, x1, y0, y1 }
        }

        fn contains(&self, point: &Point) -> bool {
            self.x0 <= point.x && point.x <= self.x1 && self.y0 <= point.y && point.y <= self.y1
        }

        fn is_above(&self, point: &Point) -> bool {
            self.y0 > point.y
        }
    }

    fn as_isize(input: &str) -> isize {
        input.parse::<isize>().unwrap()
    }

    #[derive(Debug)]
    struct Projection {
        vel: std::cell::RefCell<Vec2>,
        pos: std::cell::RefCell<Point>,
    }

    impl Iterator for Projection {
        type Item = Point;

        fn next(&mut self) -> Option<Self::Item> {
            let mut pos = self.pos.borrow_mut();
            let mut vel = self.vel.borrow_mut();

            *pos = Point::new(pos.x + vel.x, pos.y + vel.y);
            *vel = Vec2::new((vel.x - 1).max(0), vel.y - 1);

            Some(pos.clone())
        }
    }

    impl Projection {
        fn project_until_below(&mut self, area: &Area) -> Vec<Point> {
            (0..)
                .map(|_| self.next().unwrap())
                .take_while(|pt| !area.is_above(pt))
                .collect()
        }

        fn new(vel: Vec2) -> Self {
            Projection {
                vel: std::cell::RefCell::new(vel),
                pos: std::cell::RefCell::new(Point::new(0, 0)),
            }
        }
    }

    fn parse(input: &str) -> Area {
        let coords = &input[15..];
        let mut left_right = coords.split(", y=");
        let mut x = left_right.next().unwrap().split("..");
        let mut y = left_right.next().unwrap().split("..");

        Area::new(
            as_isize(x.next().unwrap()),
            as_isize(x.next().unwrap()),
            as_isize(y.next().unwrap()),
            as_isize(y.next().unwrap()),
        )
    }

    pub fn part1(input: &str) -> usize {
        let target = parse(input);

        // Calculate min-x:
        // n + n - 1 + n - 2 + ... + 0 = area.x0
        // Arithmetic series -> area.x0 = 1/2 * n * n++ -> n^2 + n - 2area.x0 = 0
        // -> (-1 += sqrt(1 + 8*area.x0)) / 2
        let min_x = (-1 + ((1 + 8 * target.x0) as f64).sqrt() as isize) / 2;
        let min_y = 0;

        (min_x..300)
            .cartesian_product(min_y..300)
            .map(|(x, y)| Projection::new(Vec2::new(x, y)))
            .map(|mut projection| projection.project_until_below(&target))
            .filter(|collection| collection.iter().any(|pt| target.contains(pt)))
            .map(|collection| collection.iter().map(|v| v.y).max().unwrap())
            .max()
            .unwrap() as usize
    }

    pub fn part2(input: &str) -> usize {
        let target = parse(input);

        // Calculate min-x:
        // n + n - 1 + n - 2 + ... + 0 = area.x0
        // Arithmetic series -> area.x0 = 1/2 * n * n++ -> n^2 + n - 2area.x0 = 0
        // -> (-1 += sqrt(1 + 8*area.x0)) / 2
        let min_x = (-1 + ((1 + 8 * target.x0) as f64).sqrt() as isize) / 2;
        let min_y = target.y0;
        let max_x = target.x1 + 1;

        (min_x..max_x)
            .cartesian_product(min_y..400)
            .map(|(x, y)| Projection::new(Vec2::new(x, y)))
            .map(|mut projection| projection.project_until_below(&target))
            .filter(|collection| collection.iter().any(|pt| target.contains(pt)))
            .count()
    }
}

mod day18 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day19 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day20 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day21 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day22 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day23 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day24 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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

mod day25 {
    fn parse(input: &str) -> Vec<usize> {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
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
    println!("Day 13, part 1: {}", day13::part1(INPUT13));
    println!("Day 13, part 2: {}", day13::part2(INPUT13));
    println!("Day 14, part 1: {}", day14::part1(INPUT14, 10));
    println!("Day 14, part 2: {}", day14::part2(INPUT14));
    println!("Day 15, part 1: {}", day15::part1(INPUT15));
    println!("Day 15, part 2: {}", day15::part2(INPUT15));
    println!("Day 16, part 1: {}", day16::part1(INPUT16));
    println!("Day 16, part 2: {}", day16::part2(INPUT16));
    println!("Day 17, part 1: {}", day17::part1(INPUT17));
    println!("Day 17, part 2: {}", day17::part2(INPUT17));
    println!("Day 18, part 1: {}", day18::part1(INPUT18));
    println!("Day 18, part 2: {}", day18::part2(INPUT18));
    println!("Day 19, part 1: {}", day19::part1(INPUT19));
    println!("Day 19, part 2: {}", day19::part2(INPUT19));
    println!("Day 20, part 1: {}", day20::part1(INPUT20));
    println!("Day 20, part 2: {}", day20::part2(INPUT20));
    println!("Day 21, part 1: {}", day21::part1(INPUT21));
    println!("Day 21, part 2: {}", day21::part2(INPUT21));
    println!("Day 22, part 1: {}", day22::part1(INPUT22));
    println!("Day 22, part 2: {}", day22::part2(INPUT22));
    println!("Day 23, part 1: {}", day23::part1(INPUT23));
    println!("Day 23, part 2: {}", day23::part2(INPUT23));
    println!("Day 24, part 1: {}", day24::part1(INPUT24));
    println!("Day 24, part 2: {}", day24::part2(INPUT24));
    println!("Day 25, part 1: {}", day25::part1(INPUT25));
    println!("Day 25, part 2: {}", day25::part2(INPUT25));

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

    #[test]
    fn day14_part1() {
        let input = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(day14::part1(input, 10), 1588);
    }

    #[test]
    fn day15_part1() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(day15::part1(input), 40);
    }

    #[test]
    fn day15_part2() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(day15::part2(input), 315);
    }

    #[test]
    fn day16_part1() {
        let input = "D2FE28";

        assert_eq!(
            day16::parse(input),
            day16::Packet {
                version: 6,
                type_: day16::PacketBody::Literal(2021)
            }
        );

        let input = "38006F45291200";

        assert_eq!(
            day16::parse(input),
            day16::Packet {
                version: 1,
                type_: day16::PacketBody::Operator(day16::Operator {
                    id: 6,
                    sub_packets: vec![
                        day16::Packet {
                            version: 6,
                            type_: day16::PacketBody::Literal(10)
                        },
                        day16::Packet {
                            version: 2,
                            type_: day16::PacketBody::Literal(20)
                        }
                    ]
                })
            }
        );
    }

    #[test]
    fn day16_parser_int_from_nbits_parse() {
        let input = &[0, 1, 1, 1, 0, 1];
        assert_eq!(day16::int_from_nbits_parse(input, 5), Ok((&[1][..], 14)))
    }

    #[test]
    fn day16_parser_int_from_3bits_parse() {
        let input = &[1, 0, 1, 0, 0, 0];

        assert_eq!(day16::int_from_3bits_parse(input), Ok((&[0, 0, 0][..], 5)));
    }

    #[test]
    fn day16_parser_body_operator_by_total_length_parse() {
        let input = &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0,
            1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(
            day16::body_operator_by_total_length_parse(input),
            Ok((
                &[0, 0, 0, 0, 0, 0, 0][..],
                (vec![
                    day16::Packet {
                        version: 6,
                        type_: day16::PacketBody::Literal(10)
                    },
                    day16::Packet {
                        version: 2,
                        type_: day16::PacketBody::Literal(20)
                    }
                ])
            ))
        );
    }

    #[test]
    fn day17_part1() {
        let input = "target area: x=20..30, y=-10..-5";

        assert_eq!(day17::part1(input), 45);
    }
}
