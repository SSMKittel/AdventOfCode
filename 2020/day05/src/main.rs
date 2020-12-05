use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let mut seats = input.split('\n')
        .filter(|&x| x.len() > 0)
        .filter_map(|x| Seat::from_str(x).ok())
        .map(|x| x.id())
        .collect::<Vec<_>>();

    seats.sort();

    let max_id = seats.iter().max().unwrap();

    println!("{:#?}", seats);
    println!("min {}, max {}", seats[0], seats[seats.len() - 1]);

    let mut previous = seats[0];
    for seat in seats.into_iter().skip(1) {
        if previous + 1 != seat {
            println!("seat {}", previous + 1);
            break;
        }
        previous = seat;
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Seat {
    row: u32,
    column: u32
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err("Invalid length")
        }

        let mut rc = 0;
        let mut cc = 0;

        let mut row: u32 = 0;
        let mut column: u32 = 0;
        for c in s.chars() {
            match c {
                'F' => {row = row << 1; rc += 1},
                'B' => {row = (row << 1) | 1; rc += 1},
                'L' => {column = column << 1; cc += 1},
                'R' => {column = (column << 1) | 1; cc += 1},
                _ => return Err("Unrecognised character")
            };
        }

        if rc == 7 && cc == 3 {
            Ok(Seat {
                row,
                column
            })
        }
        else {
            Err("invalid pattern")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rslt = Seat::from_str("FBFBBFFRLR").ok();

        assert_eq!(rslt, Some(Seat {
            row: 44,
            column: 5
        }));
    }

    #[test]
    fn test1_id() {
        assert_eq!(Seat {
            row: 44,
            column: 5
        }.id(), 357);
    }

    #[test]
    fn test2() {
        let rslt = Seat::from_str("BFFFBBFRRR").ok();

        assert_eq!(rslt, Some(Seat {
            row: 70,
            column: 7
        }));
    }

    #[test]
    fn test2_id() {
        assert_eq!(Seat {
            row: 70,
            column: 7
        }.id(), 567);
    }

    #[test]
    fn test3() {
        let rslt = Seat::from_str("FFFBBBFRRR").ok();

        assert_eq!(rslt, Some(Seat {
            row: 14,
            column: 7
        }));
    }

    #[test]
    fn test3_id() {
        assert_eq!(Seat {
            row: 14,
            column: 7
        }.id(), 119);
    }

    #[test]
    fn test4() {
        let rslt = Seat::from_str("BBFFBBFRLL").ok();

        assert_eq!(rslt, Some(Seat {
            row: 102,
            column: 4
        }));
    }

    #[test]
    fn test4_id() {
        assert_eq!(Seat {
            row: 102,
            column: 4
        }.id(), 820);
    }
}