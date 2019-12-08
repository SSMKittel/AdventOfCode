use std::collections::BTreeSet;

fn main() {
    let input_lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    let first = LineSegment::build(&Direction::from_str_csv(input_lines[0]));
    let second = LineSegment::build(&Direction::from_str_csv(input_lines[1]));

    println!("First: {:?}", first);
    println!("Second: {:?}", second);
    println!("Intersection: {:?}", intersect_points(&first, &second));

    let mut intersect = intersect_points(&first, &second);
    intersect.remove(&Point{x: 0, y: 0});
    let closest = intersect
        .iter()
        .map(|x| x.manhatten_distance(&Point{x: 0, y: 0}))
        .min()
        .unwrap();
    println!("Manhatten Distance: {:?}", closest);
}

fn intersect_points(a: &Vec::<LineSegment>, b: &Vec::<LineSegment>) -> BTreeSet<Point> {
    let mut points = BTreeSet::<Point>::new();
    for s1 in a {
        for s2 in b {
            points.extend(s1.intersect(s2));
        }
    }
    points
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
use std::cmp;

impl XAxisLine {
    fn min_x(&self) -> i32 {
        cmp::min(self.x_start, self.x_end)
    }
    fn max_x(&self) -> i32 {
        cmp::max(self.x_start, self.x_end)
    }
}

impl YAxisLine {
    fn min_y(&self) -> i32 {
        cmp::min(self.y_start, self.y_end)
    }
    fn max_y(&self) -> i32 {
        cmp::max(self.y_start, self.y_end)
    }
}

#[derive(Eq, PartialEq, Debug)]
enum LineSegment {
    HLine(XAxisLine),
    VLine(YAxisLine)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn manhatten_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

use crate::LineSegment::*;
impl LineSegment {
    fn end(&self) -> Point {
        match self {
            HLine(ls) => Point{ x: ls.x_end, y: ls.y },
            VLine(ls) => Point{ x: ls.x, y: ls.y_end }
        }
    }
    fn intersect(&self, other: &LineSegment) -> Vec<Point> {
        match self {
            HLine(a) => match other {
                HLine(b) =>
                    if a.y == b.y {
                        let range_low = cmp::max(a.min_x(), b.min_x());
                        let range_high = cmp::min(a.max_x(), b.max_x());
                        let mut rslt = Vec::new();
                        for px in range_low..=range_high {
                            rslt.push(Point{ x: px, y: a.y })
                        }
                        rslt
                    }
                    else {
                        Vec::new()
                    },
                VLine(b) =>
                    if b.x >= a.min_x() && b.x <= a.max_x() && a.y >= b.min_y() && a.y <= b.max_y() {
                        vec![Point{ x: b.x, y: a.y }]
                    }
                    else {
                        Vec::new()
                    }
            },
            VLine(a) => match other {
                HLine(_) => other.intersect(self),
                VLine(b) =>
                    if a.x == b.x {
                        let range_low = cmp::max(a.min_y(), b.min_y());
                        let range_high = cmp::min(a.max_y(), b.max_y());
                        let mut rslt = Vec::new();
                        for py in range_low..=range_high {
                            rslt.push(Point{ x: a.x, y: py })
                        }
                        rslt
                    }
                    else {
                        Vec::new()
                    }
            }
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
    fn test_line_intersection() {
        let line1 = LineSegment::build(&Direction::from_str_csv("R8,U5,L5,D3"));
        let line2 = LineSegment::build(&Direction::from_str_csv("U7,R6,D4,L4"));
        let points = intersect_points(&line1, &line2);
        let mut expected = BTreeSet::<Point>::new();
        expected.extend(vec![Point{x: 0, y: 0}, Point{x: 3, y: 3}, Point{x: 6, y: 5}]);
        assert_eq!(expected, points);
    }

    #[test]
    fn test_miss_parallel_vline() {
        let a = VLine(YAxisLine{ x: 5, y_start: 6, y_end: 3 });
        let b = VLine(YAxisLine{ x: 4, y_start: 5, y_end: 2 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = VLine(YAxisLine{ x: 5, y_start: 3, y_end: 6 });
        let b = VLine(YAxisLine{ x: 4, y_start: 2, y_end: 5 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = VLine(YAxisLine{ x: 5, y_start: 3, y_end: 6 });
        let b = VLine(YAxisLine{ x: 5, y_start: 0, y_end: 2 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = VLine(YAxisLine{ x: 5, y_start: 6, y_end: 3 });
        let b = VLine(YAxisLine{ x: 5, y_start: 2, y_end: 0 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));
    }

    #[test]
    fn test_intersect_parallel_vline() {
        let a = VLine(YAxisLine{ x: 5, y_start: 6, y_end: 3 });
        let b = VLine(YAxisLine{ x: 5, y_start: 5, y_end: 2 });
        assert_eq!(vec![Point{x: 5, y: 3}, Point{x: 5, y: 4}, Point{x: 5, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 5, y: 3}, Point{x: 5, y: 4}, Point{x: 5, y: 5}], b.intersect(&a));

        let a = VLine(YAxisLine{ x: 5, y_start: 3, y_end: 6 });
        let b = VLine(YAxisLine{ x: 5, y_start: 2, y_end: 5 });
        assert_eq!(vec![Point{x: 5, y: 3}, Point{x: 5, y: 4}, Point{x: 5, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 5, y: 3}, Point{x: 5, y: 4}, Point{x: 5, y: 5}], b.intersect(&a));
    }

    #[test]
    fn test_miss_parallel_hline() {
        let a = HLine(XAxisLine{ y: 5, x_start: 6, x_end: 3 });
        let b = HLine(XAxisLine{ y: 4, x_start: 5, x_end: 2 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 3, x_end: 6 });
        let b = HLine(XAxisLine{ y: 4, x_start: 2, x_end: 5 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 3, x_end: 6 });
        let b = HLine(XAxisLine{ y: 5, x_start: 0, x_end: 2 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 6, x_end: 3 });
        let b = HLine(XAxisLine{ y: 5, x_start: 2, x_end: 0 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));
    }

    #[test]
    fn test_intersect_parallel_hline() {
        let a = HLine(XAxisLine{ y: 5, x_start: 6, x_end: 3 });
        let b = HLine(XAxisLine{ y: 5, x_start: 5, x_end: 2 });
        assert_eq!(vec![Point{x: 3, y: 5}, Point{x: 4, y: 5}, Point{x: 5, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 3, y: 5}, Point{x: 4, y: 5}, Point{x: 5, y: 5}], b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 3, x_end: 6 });
        let b = HLine(XAxisLine{ y: 5, x_start: 2, x_end: 5 });
        assert_eq!(vec![Point{x: 3, y: 5}, Point{x: 4, y: 5}, Point{x: 5, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 3, y: 5}, Point{x: 4, y: 5}, Point{x: 5, y: 5}], b.intersect(&a));
    }

    #[test]
    fn test_intersect_perpendicular() {
        let a = HLine(XAxisLine{ y: 5, x_start: 8, x_end: 3 });
        let b = VLine(YAxisLine{ x: 6, y_start: 7, y_end: 3 });
        assert_eq!(vec![Point{x: 6, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 6, y: 5}], b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 3, x_end: 8 });
        let b = VLine(YAxisLine{ x: 6, y_start: 3, y_end: 7 });
        assert_eq!(vec![Point{x: 6, y: 5}], a.intersect(&b));
        assert_eq!(vec![Point{x: 6, y: 5}], b.intersect(&a));
    }

    #[test]
    fn test_miss_perpendicular() {
        let a = HLine(XAxisLine{ y: 5, x_start: 8, x_end: 3 });
        let b = VLine(YAxisLine{ x: 0, y_start: 0, y_end: 7 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));

        let a = HLine(XAxisLine{ y: 5, x_start: 3, x_end: 8 });
        let b = VLine(YAxisLine{ x: 0, y_start: 7, y_end: 0 });
        assert_eq!(Vec::<Point>::new(), a.intersect(&b));
        assert_eq!(Vec::<Point>::new(), b.intersect(&a));
    }

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