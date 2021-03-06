use std::ops::Index;
use std::convert::TryInto;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let asteroid_field = input_to_field(input);
    let g = AsteroidField::new(&asteroid_field);
    let station = g.locate_station();
    println!("{:?}", station);
    let hitscan = g.scan_asteroids(station.0);
    // println!("{:?}", hitscan);
    let a200 = hitscan[200 - 1];
    println!("200th: {:?}", a200);
    println!("200th (value): {}", a200.x * 100 + a200.y);
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

struct AsteroidField {
    width: i32,
    height: i32,
    field: Vec<Space>,
    scan_steps: Vec<ScanStep>
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct ScanStep{x: i32, y: i32}

impl Ord for ScanStep {
    // Order all scan_steps in laser-order so we can just scan each scan_step in a loop
    fn cmp(&self, other: &Self) -> Ordering {
        let quadrant = self.quadrant();
        match quadrant.cmp(&other.quadrant()) {
            Ordering::Equal => {},
            a => return a,
        }
        let c = self.abs().slope_ord().cmp(&other.abs().slope_ord());
        if c == Ordering::Equal {
            return self.sq_len().cmp(&other.sq_len());
        }

        if quadrant == 2 || quadrant == 4 {
            c.reverse()
        }
        else {
            c
        }
    }
}
impl PartialOrd for ScanStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ScanStep {
    fn canonical(self) -> ScanStep {
        let d = gcd(self.x, self.y);
        if d <= 1 {
            self
        }
        else {
            ScanStep{x: self.x / d, y: self.y / d}
        }
    }

    fn sq_len(self) -> usize {
        let xx = (self.x as i64 * self.x as i64) as usize;
        let yy = (self.y as i64 * self.y as i64) as usize;
        xx + yy
    }

    // Only valid with non-negative ScanStep
    fn slope_ord(self) -> i64 {
        if self.x == 0 {
            std::i64::MIN
        }
        else {
            let x: i64 = self.x as i64;
            let y: i64 = self.y as i64;
            -1000000 * y / x
        }
    }

    fn abs(self) -> ScanStep {
        ScanStep{x: self.x.abs(), y: self.y.abs()}
    }

    fn quadrant(self) -> i32 {
        // The field has positive y going down, so moving "up" is negative
        let y = -1 * self.y;
        if self.x >= 0 {
            if y >= 0 {
                1
            }
            else {
                2
            }
        }
        else {
            if y >= 0 {
                4
            }
            else {
                3
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Point{x: i32, y: i32}

impl AsteroidField {
    fn new(world: &Vec<Vec<Space>>) -> AsteroidField {
        let mut g = AsteroidField{
            width: world[0].len() as i32,
            height: world.len() as i32,
            field: world.clone().into_iter().flatten().collect(),
            scan_steps: vec![]
        };
        g.build_scan_steps();
        g
    }

    fn build_scan_steps(&mut self) {
        self.scan_steps.clear();
        self.scan_steps.reserve_exact((self.width * self.height * 4 - 4) as usize);
        for x in 0..self.width {
            for y in 0..self.height {
                if x == 0 && y == 0 {
                    continue;
                }
                self.scan_steps.push(ScanStep{x, y}.canonical());
                self.scan_steps.push(ScanStep{x: -x, y}.canonical());
                self.scan_steps.push(ScanStep{x, y: -y}.canonical());
                self.scan_steps.push(ScanStep{x: -x, y: -y}.canonical());
            }
        }
        self.scan_steps.sort();
        self.scan_steps.dedup();
        self.scan_steps.shrink_to_fit();
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && p.y >= 0
        && p.x < self.width && p.y < self.height
    }

    fn locate_station(&self) -> (Point, usize) {
        let mut stations = Vec::<(Point, usize)>::with_capacity(self.width as usize * self.height as usize);
        for x in 0..self.width {
            for y in 0..self.height {
                let station = Point{x, y};
                if self[station] != Space::Empty {
                    stations.push((station, self.view_count(station)))
                }
            }
        }
        stations.into_iter().max_by_key(|s| s.1).unwrap()
    }

    fn scan_asteroids(&self, station: Point) -> Vec<Point> {
        let mut field = self.field.clone();
        let mut asteroids = Vec::new();

        loop {
            let mut found = false;
            for &ss in &self.scan_steps {
                let mut look = station;
                loop {
                    look = Point { x: look.x + ss.x, y: look.y + ss.y };
                    match self.to_index(&look) {
                        Some(idx) => {
                            if field[idx] == Space::Asteroid {
                                asteroids.push(look);
                                field[idx] = Space::Empty;
                                found = true;
                                break;
                            }
                        }
                        None => break
                    }
                }
            }
            if !found {
                break;
            }
        }

        asteroids
    }

    fn view_count(&self, station: Point) -> usize {
        let mut seen = 0;
        for &ss in &self.scan_steps {
            let mut look = station;
            loop {
                look = Point{x: look.x + ss.x, y: look.y + ss.y};
                match self.to_index(&look) {
                    Some(idx) => {
                        if self.field[idx] == Space::Asteroid {
                            seen += 1;
                            break;
                        }
                    }
                    None => break
                }
            }
        }

        seen
    }

    fn to_index(&self, p: &Point) -> Option<usize> {
        if self.contains(p) {
            let pos = p.y * self.width + p.x;
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

fn gcd(a: i32, b: i32) -> i32 {
    fn gcd_inner(a: i32, b: i32) -> i32 {
        match a.cmp(&b) {
            Ordering::Equal => a,
            Ordering::Greater => gcd(a - b, b),
            Ordering::Less => gcd(a, b - a),
        }
    }
    if a == 0 {
        b.abs()
    }
    else if b == 0 {
        a.abs()
    }
    else {
        gcd_inner(a.abs(), b.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_gcd_107_79() { assert_eq!(1, gcd(107, 79)); }
    #[test] fn test_gcd_n107_79() { assert_eq!(1, gcd(-107, 79)); }
    #[test] fn test_gcd_107_n79() { assert_eq!(1, gcd(107, -79)); }
    #[test] fn test_gcd_n107_n79() { assert_eq!(1, gcd(-107, -79)); }
    #[test] fn test_gcd_79_107() { assert_eq!(1, gcd(79, 107)); }
    #[test] fn test_gcd_n79_107() { assert_eq!(1, gcd(-79, 107)); }
    #[test] fn test_gcd_79_n107() { assert_eq!(1, gcd(79, -107)); }
    #[test] fn test_gcd_n79_n107() { assert_eq!(1, gcd(-79, -107)); }
    #[test] fn test_gcd_32_48() { assert_eq!(16, gcd(32, 48)); }
    #[test] fn test_gcd_32_n48() { assert_eq!(16, gcd(32, -48)); }
    #[test] fn test_gcd_n32_48() { assert_eq!(16, gcd(-32, 48)); }
    #[test] fn test_gcd_n32_n48() { assert_eq!(16, gcd(-32, -48)); }
    #[test] fn test_gcd_48_32() { assert_eq!(16, gcd(48, 32)); }
    #[test] fn test_gcd_n48_32() { assert_eq!(16, gcd(-48, 32)); }
    #[test] fn test_gcd_48_n32() { assert_eq!(16, gcd(48, -32)); }
    #[test] fn test_gcd_n48_n32() { assert_eq!(16, gcd(-48, -32)); }
    #[test] fn test_gcd_0_48() { assert_eq!(48, gcd(0, 48)); }
    #[test] fn test_gcd_32_0() { assert_eq!(32, gcd(32, 0)); }
    #[test] fn test_gcd_0_n48() { assert_eq!(48, gcd(0, -48)); }
    #[test] fn test_gcd_n32_0() { assert_eq!(32, gcd(-32, 0)); }
    #[test] fn test_gcd_0_0() { assert_eq!(0, gcd(0, 0)); }
    #[test] fn test_gcd_0_1() { assert_eq!(1, gcd(0, 1)); }
    #[test] fn test_gcd_1_0() { assert_eq!(1, gcd(1, 0)); }
    #[test] fn test_gcd_1_1() { assert_eq!(1, gcd(1, 1)); }
    #[test] fn test_gcd_0_n1() { assert_eq!(1, gcd(0, -1)); }
    #[test] fn test_gcd_n1_0() { assert_eq!(1, gcd(-1, 0)); }
    #[test] fn test_gcd_n1_n1() { assert_eq!(1, gcd(-1, -1)); }

    fn run_locate(expected: (Point, usize), input: &str) {
        let asteroid_field = input_to_field(input);
        let g = AsteroidField::new(&asteroid_field);
        assert_eq!(expected, g.locate_station());
    }

    #[test]
    fn test_1() {
        run_locate(
            (Point{x: 3, y: 4}, 8),
".#..#
.....
#####
....#
...##");
    }

    #[test]
    fn test_2() {
        run_locate(
            (Point{x: 5, y: 8}, 33),
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
            (Point{x: 1, y: 2}, 35),
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
            (Point{x: 6, y: 3}, 41),
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
            (Point{x: 11, y: 13}, 210),
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

    #[test]
    fn test_200th() {
        let asteroid_field = input_to_field(".#..##.###...#######
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

        let g = AsteroidField::new(&asteroid_field);
        let hitscan = g.scan_asteroids(Point{x: 11, y: 13});
        assert_eq!(Point{x: 8, y: 2}, hitscan[200 - 1]);
    }
}