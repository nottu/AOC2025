fn main() {
    let content = include_str!("input.txt");

    let total_cnt = part1(content);
    println!("{total_cnt}");

    let total_cnt = part2(content);
    println!("{total_cnt}");
}

fn part1(content: &str) -> usize {
    let mut total_sum: usize = 0;
    for range in content.split(',').map(|range| -> Range { range.into() }) {
        // dbg!(range);
        for val in range.start..=range.end {
            if is_special(val) {
                // dbg!(val);
                total_sum += val
            }
        }
    }
    total_sum
}

fn part2(content: &str) -> usize {
    let mut total_sum: usize = 0;
    for range in content.split(',').map(|range| -> Range { range.into() }) {
        dbg!(range);
        for val in range.start..=range.end {
            if is_super_special(val) {
                total_sum += val
            }
        }
    }
    total_sum
}

fn is_special(val: usize) -> bool {
    let log10 = val.ilog10();
    if log10 % 2 == 0 {
        return false;
    }
    let factor = 10usize.pow(1 + log10 / 2);
    val / factor == val % factor
}

fn is_super_special(val: usize) -> bool {
    let mut factor = 1;
    while factor < val {
        factor *= 10;
        let repeated = val % factor;
        if repeated < factor / 10 {
            continue;
        }
        let mut val = val / factor;
        if val == 0 {
            break;
        }
        // some loop
        while val > 0 && val % factor == repeated {
            val /= factor
        }
        if val == 0 {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        eprintln!("val: {value}");
        let mut value = value.split('-');

        let start = value
            .next()
            .map(|v| v.trim().parse::<usize>().expect("expected usize"))
            .unwrap();
        let end = value
            .next()
            .map(|v| {
                v.trim()
                    .parse::<usize>()
                    .expect(&format!("expected usize, found: {v}"))
            })
            .unwrap();

        Range { start, end }
    }
}

#[cfg(test)]
mod test {
    use crate::{Range, is_special, is_super_special, part1, part2};

    #[test]
    fn test_range_from_str() {
        let test_range: Range = "1-100".into();
        assert_eq!(test_range, Range { start: 1, end: 100 })
    }

    #[test]
    fn test_is_special() {
        assert!(is_special(1010));
        assert!(!is_special(1011));
    }

    #[test]
    fn test_is_super_special() {
        assert!(is_super_special(1010));
        assert!(!is_super_special(101));

        assert!(is_super_special(101010));
    }

    #[test]
    fn test_part1() {
        let content = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(1227775554, part1(content))
    }

    #[test]
    fn test_part2() {
        let content = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(4174379265, part2(content))
    }
}
