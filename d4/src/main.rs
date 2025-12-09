use std::{
    fmt::{Debug, Write},
    iter,
};

fn main() {
    let content = include_str!("input.txt");

    let p1 = part1(content);
    println!("{p1}");

    let p2 = part2(content);
    println!("{p2}");
}

fn part1(content: &str) -> usize {
    let mut map = Map::new_from_str(content);
    let r = map.num_accesible();
    r
}
fn part2(content: &str) -> usize {
    let mut map = Map::new_from_str(content);
    let mut total_removed = 0;
    loop {
        let r = map.remove_accesible();
        if r == 0 {
            break;
        }
        total_removed += r;
    }
    total_removed
}

struct Map {
    inner: Vec<Vec<Tile>>,
}

impl Map {
    fn new_from_str(map: &str) -> Self {
        let len_line = map.lines().next().map(|l| l.len() + 2).unwrap_or_default();
        let inner = iter::once(vec![Tile::Empty; len_line])
            .chain(map.lines().map(|line| {
                iter::once(Tile::Empty)
                    .chain(
                        line.chars()
                            .map(Tile::from_char)
                            .chain(iter::once(Tile::Empty)),
                    )
                    .collect()
            }))
            .chain(iter::once(vec![Tile::Empty; len_line]))
            .collect();
        Self { inner }
    }

    fn num_accesible(&mut self) -> usize {
        let mut num_acc = 0;
        for i in 1..self.inner.len() - 1 {
            for j in 1..self.inner[i].len() - 1 {
                let tile = self.inner[i][j];
                if tile == Tile::Empty {
                    continue;
                }
                // check all 4 sides
                let mut num_empty = 0;

                if self.inner[i - 1][j] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i][j + 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i - 1][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i - 1][j + 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j + 1] == Tile::Empty {
                    num_empty += 1
                };

                if num_empty > 4 {
                    num_acc += 1;
                    self.inner[i][j] = Tile::AccRoll;
                }
            }
        }

        num_acc
    }

    fn remove_accesible(&mut self) -> usize {
        let mut num_acc = 0;
        for i in 1..self.inner.len() - 1 {
            for j in 1..self.inner[i].len() - 1 {
                let tile = self.inner[i][j];
                if tile == Tile::Empty {
                    continue;
                }
                // check all 4 sides
                let mut num_empty = 0;

                if self.inner[i - 1][j] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i][j + 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i - 1][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i - 1][j + 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j - 1] == Tile::Empty {
                    num_empty += 1
                };
                if self.inner[i + 1][j + 1] == Tile::Empty {
                    num_empty += 1
                };

                if num_empty > 4 {
                    num_acc += 1;
                    self.inner[i][j] = Tile::Empty;
                }
            }
        }

        num_acc
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for l in self.inner[..self.inner.len() - 1].iter().skip(1) {
            for tile in l[..l.len() - 1].iter().skip(1) {
                f.write_fmt(format_args!("{:?}", tile))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
    AccRoll,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('.'),
            Self::Roll => f.write_char('@'),
            Self::AccRoll => f.write_char('x'),
        }
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Roll,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    const TEST_CONTENT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(TEST_CONTENT))
    }

    #[test]
    fn test_part2() {
        assert_eq!(43, part2(TEST_CONTENT))
    }
}
