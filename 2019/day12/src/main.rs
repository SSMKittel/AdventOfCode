use std::cmp::Ordering;
use std::borrow::BorrowMut;

fn main() {
    let mut moons = Moons::new();
    moons.add(-19, -4, 2);
    moons.add(-9, 8, -16);
    moons.add(-4, 5, -11);
    moons.add(1, 9, -13);

    for _ in 0..1000 {
        moons.step();
    }

    println!("{}", moons.energy());
}

struct Moon {
    p_x: i64,
    p_y: i64,
    p_z: i64,
    v_x: i64,
    v_y: i64,
    v_z: i64,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.p_x += self.v_x;
        self.p_y += self.v_y;
        self.p_z += self.v_z;
    }
}

struct Moons {
    moons: Vec<Moon>
}

fn adjust_velocity(m1: &mut Moon, m2: &mut Moon) {
    match m1.p_x.cmp(&m2.p_x) {
        Ordering::Less => {
            m1.v_x += 1;
            m2.v_x -= 1;
        },
        Ordering::Equal => {},
        Ordering::Greater => {
            m1.v_x -= 1;
            m2.v_x += 1;
        },
    }

    match m1.p_y.cmp(&m2.p_y) {
        Ordering::Less => {
            m1.v_y += 1;
            m2.v_y -= 1;
        },
        Ordering::Equal => {},
        Ordering::Greater => {
            m1.v_y -= 1;
            m2.v_y += 1;
        },
    }

    match m1.p_z.cmp(&m2.p_z) {
        Ordering::Less => {
            m1.v_z += 1;
            m2.v_z -= 1;
        },
        Ordering::Equal => {},
        Ordering::Greater => {
            m1.v_z -= 1;
            m2.v_z += 1;
        },
    }
}

impl Moons {
    fn new() -> Moons {
        Moons{
            moons: vec![]
        }
    }

    fn step(&mut self) {
        for i in 0..self.moons.len() {
            let (left, right) = self.moons.split_at_mut(i + 1);
            let m1 = left[i].borrow_mut();
            for j in 0..right.len() {
                let m2 = right[j].borrow_mut();
                adjust_velocity(m1, m2);
            }
        }
        for moon in self.moons.iter_mut() {
            moon.apply_velocity()
        }
    }

    fn add(&mut self, x: i64, y: i64, z: i64) {
        self.moons.push(Moon{
            p_x: x,
            p_y: y,
            p_z: z,
            v_x: 0,
            v_y: 0,
            v_z: 0
        });
    }

    fn energy(&self) -> i64 {
        let mut rslt = 0;

        for moon in self.moons.iter() {
            let pot = moon.p_x.abs() + moon.p_y.abs() + moon.p_z.abs();
            let kin = moon.v_x.abs() + moon.v_y.abs() + moon.v_z.abs();
            rslt += pot * kin;
        }

        rslt
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_179() {
        let mut moons = Moons::new();
        moons.add(-1, 0, 2);
        moons.add(2, -10, -7);
        moons.add(4, -8, 8);
        moons.add(3, 5, -1);

        for _ in 0..10 {
            moons.step();
        }

        assert_eq!(moons.energy(), 179);
    }

    #[test]
    fn test_energy_1940() {
        let mut moons = Moons::new();
        moons.add(-8, -10, 0);
        moons.add(5, 5, 10);
        moons.add(2, -7, 3);
        moons.add(9, -8, -3);

        for _ in 0..100 {
            moons.step();
        }

        assert_eq!(moons.energy(), 1940);
    }
}