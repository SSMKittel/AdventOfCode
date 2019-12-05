
fn main() {
    let input_lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    let first = LineSegment::build(&Direction::from_str_csv(input_lines[0]));
    let second = LineSegment::build(&Direction::from_str_csv(input_lines[1]));

    println!("First: {:?}", first);
    println!("Second: {:?}", second);
}

#[derive(Eq, PartialEq, Debug)]
struct YAxisLine {
    x: i32,
    y_start: i32,
    y_end: i32
}

#[derive(Eq, PartialEq, Debug)]
struct XAxisLine {
    y: i32,
    x_start: i32,
    x_end: i32
}

#[derive(Eq, PartialEq, Debug)]
enum LineSegment {
    HLine(XAxisLine),
    VLine(YAxisLine)
}

#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

use crate::LineSegment::*;
impl LineSegment {
    fn end(&self) -> Point {
        match self {
            HLine(ls) => Point{ x: ls.x_end, y: ls.y },
            VLine(ls) => Point{ x: ls.x, y: ls.y_end }
        }
    }
    fn build(directions: &[Direction]) -> Vec<LineSegment> {
        let mut point = Point{ x: 0, y: 0 };
        let mut line: Vec<LineSegment> = Vec::new();
        for dir in directions {
            let current = match dir {
                Up(mag) => VLine(YAxisLine { x: point.x, y_start: point.y, y_end: point.y + *mag }),
                Down(mag) => VLine(YAxisLine { x: point.x, y_start: point.y, y_end: point.y - *mag }),
                Left(mag) => HLine(XAxisLine { y: point.y, x_start: point.x, x_end: point.x - *mag }),
                Right(mag) => HLine(XAxisLine { y: point.y, x_start: point.x, x_end: point.x + *mag }),
            };
            point = current.end();
            line.push(current);
        }
        line
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

use crate::Direction::*;
impl Direction {
    fn from_str(val: &str) -> Direction {
        let (dir, magnitude_str) = val.split_at(1);
        let magnitude = magnitude_str.parse::<i32>().unwrap();
        match dir {
            "U" => Up(magnitude),
            "D" => Down(magnitude),
            "L" => Left(magnitude),
            "R" => Right(magnitude),
            _ => panic!(String::from(val))
        }
    }

    fn from_str_csv(val: &str) -> Vec<Direction> {
        val.split(",")
            .map(Direction::from_str)
            .collect::<Vec<Direction>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![
            Right(8), Up(5), Left(5), Down(3)
        ];
        let parsed = Direction::from_str_csv("R8,U5,L5,D3");
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_build() {
        let expected = vec![
            HLine(XAxisLine{ y: 0, x_start: 0, x_end: 8 }),
            VLine(YAxisLine{ x: 8, y_start: 0, y_end: 5 }),
            HLine(XAxisLine{ y: 5, x_start: 8, x_end: 3 }),
            VLine(YAxisLine{ x: 3, y_start: 5, y_end: 2 })
        ];
        let line = LineSegment::build(&Direction::from_str_csv("R8,U5,L5,D3"));
        assert_eq!(expected, line);
    }

    #[test]
    fn test_end() {
        let expected = vec![
            Point{ x: 8, y: 0 },
            Point{ x: 8, y: 5 },
            Point{ x: 3, y: 5 },
            Point{ x: 3, y: 2 }
        ];

        let ends = LineSegment::build(&Direction::from_str_csv("R8,U5,L5,D3"))
            .iter()
            .map(LineSegment::end)
            .collect::<Vec<_>>();

        assert_eq!(expected, ends);
    }
}