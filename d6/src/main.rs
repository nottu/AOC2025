fn main() {
    let content = include_str!("input.txt");

    let res = process_p1(&content);
    println!("p1: {res}");

    let res = process_p2(&content);
    println!("p2: {res}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Mult,
    Sum,
}

fn process_p1(input: &str) -> usize {
    let last_line = input.lines().last().expect("Expected Ops").trim();
    let ops = last_line
        .split_whitespace()
        .map(|op| match op {
            "*" => Op::Mult,
            "+" => Op::Sum,
            _ => unreachable!("Got {op}"),
        })
        .collect::<Vec<_>>();

    input
        .lines()
        .into_iter()
        .rev()
        .skip(1)
        .map(|v| {
            v.trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().expect("expected val"))
                .collect::<Vec<usize>>()
        })
        .reduce(|mut acc, x| {
            for i in 0..ops.len() {
                match ops[i] {
                    Op::Mult => acc[i] *= x[i],
                    Op::Sum => acc[i] += x[i],
                }
            }
            acc
        })
        .expect("epxected reduce")
        .iter()
        .sum()
}

fn process_p2(input: &str) -> usize {
    let last_line = input.lines().last().expect("Expected Ops").trim();
    let ops = last_line
        .split_whitespace()
        .map(|op| match op {
            "*" => Op::Mult,
            "+" => Op::Sum,
            _ => unreachable!("Got {op}"),
        })
        .collect::<Vec<_>>();

    let lines: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut problems = vec![];
    let mut problem = vec![];
    for j in 0..max_len {
        let mut tmp = String::new();
        for i in 0..(lines.len() - 1) {
            if lines[i].len() <= j {
                continue;
            }
            tmp.push(lines[i][j]);
        }
        let tmp = tmp.trim();
        if tmp.is_empty() {
            problems.push(problem);
            problem = vec![];
            continue;
        }
        // println!("tpm: {tmp}");
        problem.push(tmp.parse::<usize>().expect("expected val"));
    }
    problems.push(problem);

    ops.into_iter()
        .zip(problems)
        .map(|(op, problem)| {
            // dbg!(op, &problem);
            problem
                .into_iter()
                .reduce(|acc, v| match op {
                    Op::Mult => acc * v,
                    Op::Sum => acc + v,
                })
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{process_p1, process_p2};

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    #[test]
    fn test_p1() {
        let res = process_p1(TEST_INPUT);
        assert_eq!(4277556, res);
    }

    #[test]
    fn test_p2() {
        let res = process_p2(TEST_INPUT);
        assert_eq!(3263827, res);
    }
}
