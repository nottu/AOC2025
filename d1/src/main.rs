fn main() {
    let num_vals = 100;

    let content = include_str!("input1.txt");
    let password = decode_password(&content, 50, num_vals);
    println!("p1: {password}");

    let password = decode_password2(&content, 50, num_vals);
    println!("p2: {password}")
}

fn decode_password(content: &str, start: i64, num_vals: i64) -> i64 {
    content
        .lines()
        .map(|line| {
            let (direction_char, ammount_str) = line.trim().split_at(1);
            let direction = match direction_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction {direction_char}."),
            };
            let ammount = ammount_str.parse::<i64>().expect("Invalid ammount");
            Rotation { direction, ammount }
        })
        .fold((start, 0), |acc, x| {
            // println!("acc: {acc}");
            let pos = match x.direction {
                Direction::Left => (acc.0 - x.ammount).rem_euclid(num_vals),
                Direction::Right => (acc.0 + x.ammount).rem_euclid(num_vals),
            };
            if pos == 0 {
                (pos, acc.1 + 1)
            } else {
                (pos, acc.1)
            }
        })
        .1
}

fn decode_password2(content: &str, start: i64, num_vals: i64) -> i64 {
    let rotations = content.lines().map(|line| {
        let (direction_char, ammount_str) = line.trim().split_at(1);
        let direction = match direction_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction {direction_char}."),
        };
        let ammount = ammount_str.parse::<i64>().expect("Invalid ammount");
        Rotation { direction, ammount }
    });

    let mut pass = 0;
    let mut pos = start;
    for Rotation { direction, ammount } in rotations {
        pass += (ammount / num_vals).abs();
        let ammount = ammount % num_vals;
        if ammount == 0 {
            continue;
        }
        let new_pos = match direction {
            Direction::Left => pos - ammount,
            Direction::Right => pos + ammount,
        };
        if (new_pos <= 0 && pos != 0) || new_pos >= num_vals {
            pass += 1;
        }
        pos = new_pos.rem_euclid(num_vals);
    }
    pass
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    ammount: i64,
}

#[cfg(test)]
mod test {
    use crate::{decode_password, decode_password2};

    #[test]
    fn test_decode() {
        let input = "L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";
        let password = decode_password(input, 50, 100);
        assert_eq!(password, 3);
    }

    #[test]
    fn test_decode2() {
        let input = "L68
        L130
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";
        let password = decode_password2(input, 50, 100);
        assert_eq!(password, 7);
    }
}
