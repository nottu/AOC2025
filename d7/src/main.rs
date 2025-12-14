use std::collections::{HashMap, HashSet};

fn main() {
    let content = include_str!("input.txt");

    let res = process_p1(&content);
    println!("p1: {res}");

    let res = process_p2(&content);
    println!("p2: {res}");
}

fn process_p1(input: &str) -> usize {
    let mut beam_indexes = HashSet::new();
    let mut lines = input.lines();

    // first line, find S
    beam_indexes.insert(
        lines
            .next()
            .expect("expected on line")
            .char_indices()
            .find(|&(_, c)| c == 'S')
            .expect("expected Start")
            .0,
    );

    let mut split_cnt = 0;

    while let Some(line) = lines.next() {
        for (i, c) in line.char_indices() {
            // beam gets split
            if c == '^' && beam_indexes.contains(&i) {
                beam_indexes.remove(&i);
                // could this be out of bounds?
                beam_indexes.insert(i - 1);
                beam_indexes.insert(i + 1);
                split_cnt += 1;
            }
        }
    }

    split_cnt
}

fn process_p2(input: &str) -> usize {
    let mut beam_indexes = HashMap::new();
    let mut lines = input.lines();

    // first line, find S
    beam_indexes.insert(
        lines
            .next()
            .expect("expected on line")
            .char_indices()
            .find(|&(_, c)| c == 'S')
            .expect("expected Start")
            .0,
        1,
    );

    while let Some(line) = lines.next() {
        for (i, c) in line.char_indices() {
            // beam gets split
            if c == '^' && beam_indexes.contains_key(&i) {
                let cnt = beam_indexes[&i];
                beam_indexes.remove(&i);
                // could this be out of bounds?
                *beam_indexes.entry(i - 1).or_default() += cnt;
                *beam_indexes.entry(i + 1).or_default() += cnt;
            }
        }
        // dbg!(&beam_indexes);
    }
    beam_indexes.values().sum()
}

#[cfg(test)]
mod test {
    use crate::{process_p1, process_p2};

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_p1() {
        let res = process_p1(TEST_INPUT);
        assert_eq!(21, res);
    }

    #[test]
    fn test_p2() {
        let res = process_p2(TEST_INPUT);
        assert_eq!(40, res);
    }
}
