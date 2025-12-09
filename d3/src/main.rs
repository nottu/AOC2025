fn main() {
    let content = include_str!("input.txt");

    let joltage = part1(&content);
    println!("p1: {joltage}");

    let joltage = part2(&content);
    println!("p2: {joltage}");
}

fn part1(content: &str) -> usize {
    content
        .lines()
        .map(|l| BatteryBank::from(l).max_joltage(2))
        .sum()
}

fn part2(content: &str) -> usize {
    content
        .lines()
        .map(|l| BatteryBank::from(l).max_joltage(12))
        .sum()
}

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    fn max_joltage(&self, num_on: usize) -> usize {
        let mut start_idx = 0;
        let mut joltage = 0;
        let mut num_battery = 0;

        while num_battery < num_on {
            joltage *= 10;
            let Some((idx, j)) = self.batteries
                [start_idx..=self.batteries.len() + num_battery - num_on]
                .iter()
                .map(|v| v.to_owned())
                .enumerate()
                .max_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0)))
            else {
                unreachable!("Should not reach here while searching for {num_battery} digit")
            };
            start_idx += idx + 1;
            joltage += j as usize;
            num_battery += 1;
        }
        joltage
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries = value
            .chars()
            .map(|v| v.to_digit(10).expect("Expected decimal digit") as u8)
            .collect();
        Self { batteries }
    }
}

#[cfg(test)]
mod test {
    use crate::{BatteryBank, part1, part2};

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_max_joltage() {
        let bank = BatteryBank::from("1234");
        assert_eq!(34, bank.max_joltage(2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(357, part1(TEST_INPUT))
    }

    #[test]
    fn test_part2() {
        assert_eq!(3121910778619, part2(TEST_INPUT))
    }
}
