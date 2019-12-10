use std::ops::Index;
use std::convert::TryInto;

fn main() {
    let input = include_str!("input.txt");
    let asteroid_field = input_to_field(input);
    let g = AsteroidField::new(&asteroid_field);
    let station = g.locate_station();
    println!("{:?}", station);
}

fn input_to_field(input: &str) -> Vec<Vec<Space>> {
    input.lines().map(line_to_field_row).collect()
}

fn line_to_field_row(line: &str) -> Vec<Space> {
    line.chars().map(char_to_space).collect()
}

fn char_to_space(c: char) -> Space {
    match c {
        '.' => Space::Empty,
        '#' => Space::Asteroid,
        _ => panic!("Unrecognised {}", c)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Space {
    Asteroid,
    Empty
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Visibility {
    Blocked,
    Visible,
    Empty
}

struct AsteroidField {
    width: i32,
    height: i32,
    field: Vec<Space>,
    moves: Vec<Move>
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Move(i32, i32);
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

impl AsteroidField {
    fn new(world: &Vec<Vec<Space>>) -> AsteroidField {
        let mut g = AsteroidField{
            width: world[0].len() as i32,
            height: world.len() as i32,
            field: world.clone().into_iter().flatten().collect(),
            moves: vec![]
        };
        g.build_moves();
        g
    }

    fn build_moves(&mut self) {
        self.moves.clear();
        for x in 0..=(self.width / 2) {
            for y in 0..=(self.height / 2) {
                if x == 0 && y == 0 {
                    continue;
                }
                // We could simplify this by dividing both params by prime p when x & y are wholly divisible by p
                self.moves.push(Move(x, y));
                self.moves.push(Move(-x, y));
                self.moves.push(Move(x, -y));
                self.moves.push(Move(-x, -y));
            }
        }
        self.moves.sort();
        self.moves.dedup();
    }

    fn contains(&self, p: &Point) -> bool {
        p.0 >= 0 && p.1 >= 0
        && p.0 < self.width && p.1 < self.height
    }

    fn locate_station(&self) -> (Point, usize) {
        let mut stations = Vec::<(Point, usize)>::with_capacity(self.width as usize * self.height as usize);
        for x in 0..self.width {
            for y in 0..self.height {
                let station = Point(x, y);
                stations.push((station, self.view_count(station)))
            }
        }
        stations.into_iter().max_by_key(|s| s.1).unwrap()
    }

    fn view_count(&self, station: Point) -> usize {
        if let Space::Empty = self[station] {
            return 0;
        }

        let mut vis = self.field.iter().map(|&s| match s {
            Space::Asteroid => Visibility::Visible,
            Space::Empty => Visibility::Empty,
        }).collect::<Vec<_>>();

        vis[self.to_index(&station).unwrap()] = Visibility::Blocked;

        for &m in &self.moves {
            let mut look = station;
            let mut blocking = false;
            loop {
                look = Point(look.0 + m.0, look.1 + m.1);
                match self.to_index(&look) {
                    Some(idx) => {
                        if self.field[idx] == Space::Asteroid {
                            if blocking {
                                vis[idx] = Visibility::Blocked;
                            }
                            else {
                                blocking = true;
                            }
                        }
                    }
                    None => break
                }
            }
        }
        vis.into_iter()
            .filter(|&x| x == Visibility::Visible)
            .count()
    }

    fn to_index(&self, p: &Point) -> Option<usize> {
        if self.contains(p) {
            let pos = p.1 * self.width + p.0;
            let pos: usize = pos.try_into().unwrap();
            Some(pos)
        }
        else {
            None
        }
    }
}

impl Index<Point> for AsteroidField {
    type Output = Space;

    fn index(&self, index: Point) -> &Self::Output {
        let idx = self.to_index(&index).unwrap();
        &self.field[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_locate(expected: (Point, usize), input: &str) {
        let asteroid_field = input_to_field(input);
        let g = AsteroidField::new(&asteroid_field);
        assert_eq!(expected, g.locate_station());
    }

    #[test]
    fn test_1() {
        run_locate(
            (Point(3, 4), 8),
".#..#
.....
#####
....#
...##");
    }

    #[test]
    fn test_2() {
        run_locate(
            (Point(5, 8), 33),
"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####");
    }

    #[test]
    fn test_3() {
        run_locate(
            (Point(1, 2), 35),
"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.");
    }

    #[test]
    fn test_4() {
        run_locate(
            (Point(6, 3), 41),
".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..");
    }

    #[test]
    fn test_5() {
        run_locate(
            (Point(11, 13), 210),
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##");
    }
}