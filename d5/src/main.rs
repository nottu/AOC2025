fn main() {
    let content = include_str!("input.txt");

    let num_fresh = part1(content);
    println!("{num_fresh}");
}

fn part1(content: &str) -> usize {
    let mut lines = content.lines();
    let ranges = {
        let mut ranges = Vec::new();
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            ranges.push(Range::new_from_str(l));
        }
        ranges.sort_unstable_by_key(|r| r.start);

        let mut collapsed_ranges = Vec::with_capacity(ranges.len());

        let mut tmp_range = ranges[0];

        for r in ranges.into_iter().skip(1) {
            if tmp_range.in_range(r.start) {
                tmp_range.end = tmp_range.end.max(r.end);
            } else {
                collapsed_ranges.push(tmp_range);
                tmp_range = r;
            }
        }
        collapsed_ranges.push(tmp_range);
        collapsed_ranges
    };

    let fresh_size = ranges.iter().map(|r| r.size()).sum::<usize>();
    println!("Ranges Size: {fresh_size}");

    lines
        .map(|l| l.parse::<usize>().expect("expected id"))
        .filter(|&id| ranges.iter().any(|r| r.in_range(id)))
        .count()
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn in_range(&self, val: usize) -> bool {
        val >= self.start && val <= self.end
    }
    fn new_from_str(range: &str) -> Self {
        let mut range = range.split('-');
        let start = range
            .next()
            .map(|v| v.parse().expect("expected start val"))
            .expect("expected start");

        let end = range
            .next()
            .map(|v| v.parse().expect("expected end val"))
            .expect("expected end");

        Range { start, end }
    }
    fn size(&self) -> usize {
        1 + self.end - self.start
    }
}

#[cfg(test)]
mod test {
    use crate::{Range, part1};

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_range() {
        let range = Range::new_from_str("3-5");
        assert!(range.in_range(3));
        assert!(range.in_range(4));
        assert!(range.in_range(5));

        assert!(!range.in_range(2));
        assert!(!range.in_range(6));
    }

    #[test]
    fn test_part1() {
        let num_fresh = part1(TEST_INPUT);
        assert_eq!(3, num_fresh)
    }
}
