use std::collections::HashMap;
extern crate int_code;
use int_code::*;

fn main() {
    let mem = parse_csv(include_str!("input.txt")).unwrap();

    let painted = paint(&mem);

    println!("Painted: {}", painted.len());
}

fn paint(init_mem: &Vec<Word>) -> HashMap<Point, Colour> {
    let (mut machine, input, output) = Machine::new(init_mem);
    let mut direction = Direction::Up;
    let mut paints = HashMap::<Point, Colour>::new();
    let mut position = Point{x: 0, y: 0};
    loop {
        let mut halt = false;

        let cur_colour = paints.get(&position).unwrap_or(&Colour::Black);

        input.send(cur_colour.to_word()).unwrap();
        match machine.execute(10000) {
            Ok(_) => halt = true,
            Err(ExecuteError::InputRequired) => {},
            Err(e) => panic!("{}", e),
        }

        let new_colour = Colour::from_word(output.recv().unwrap());
        paints.insert(position, new_colour);

        match output.recv().unwrap() {
            0 => direction = direction.rotate_left(),
            1 => direction = direction.rotate_right(),
            x => panic!("direction: {}", x)
        }

        position.move_dir(direction);

        if halt {
            break;
        }
    }
    paints
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Colour {
    Black,
    White
}

impl Colour {
    fn to_word(self) -> Word {
        match self {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
    fn from_word(w: Word) -> Colour {
        match w {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!("{}", w)
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Point{x: i32, y: i32}

impl Point {
    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

impl Direction {
    fn rotate_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}