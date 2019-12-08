use std::collections::HashMap;
use std::ops::Index;
use std::borrow::Borrow;

fn main() {
    let orbits = include_str!("input.txt")
        .lines()
        .map(build_orbit)
        .collect::<Vec<_>>();

    let system = build_system(&orbits);

    println!("Direct and Indirect: {}", system.count_all_direct_indirect());
}

fn build_system<'a>(orbits: &[Orbit<'a>]) -> SolarSystem<'a> {
    let mut ss = SolarSystem {
        entries: vec![],
        names: HashMap::new(),
        roots: vec![]
    };

    let mut names = orbits.iter()
        .flat_map(|x| vec![x.parent, x.child])
        .collect::<Vec<&str>>();
    names.sort();
    names.dedup();
    for name in names {
        ss.add(name);
    }
    for orbit in orbits {
        ss.make_orbit(ss[orbit.parent].id, ss[orbit.child].id);
    }
    ss
}

#[derive(Eq, PartialEq, Debug)]
struct Orbit<'a> {
    parent: &'a str,
    child: &'a str
}
fn build_orbit(line: &str) -> Orbit {
    let names = line.split(')').collect::<Vec<_>>();
    Orbit{ parent: names[0], child: names[1] }
}

type SoiId = usize;
#[derive(Eq, PartialEq, Debug)]
struct SolarSystem<'a> {
    entries: Vec<SphereOfInfluence<'a>>,
    names: HashMap<&'a str, SoiId>,
    roots: Vec<SoiId>
}

impl<'a> SolarSystem<'a> {
    fn add(&mut self, name: &'a str) -> SoiId {
        let id = self.entries.len();
        self.entries.push(SphereOfInfluence{ id, name, children: vec![] });
        if let Some(_) = self.names.insert(name, id) {
            panic!("Duplicate entry {}", name)
        }

        self.roots.push(id);
        id
    }
    fn make_orbit(&mut self, parent: SoiId, child: SoiId) {
        if let Some(index) = self.roots.iter().position(|x| *x == child) {
            self.roots.remove(index);
        }
        self.entries[parent].children.push(child);
    }

    fn count_all_direct_indirect(&self) -> usize {
        let mut total: usize = 0;
        for root in &self.roots {
            total += self.count_direct_indirect(&self[*root], 0)
        }
        total
    }
    fn count_direct_indirect(&self, soi: &SphereOfInfluence<'a>, depth: usize) -> usize {
        let mut total: usize = depth;
        for root in &soi.children {
            total += self.count_direct_indirect(&self[*root], depth + 1)
        }
        total
    }
}
impl<'a> Index<SoiId> for SolarSystem<'a> {
    type Output = SphereOfInfluence<'a>;

    fn index(&self, index: SoiId) -> &Self::Output {
        self.entries[index].borrow()
    }
}
impl<'a> Index<&str> for SolarSystem<'a> {
    type Output = SphereOfInfluence<'a>;

    fn index(&self, index: &str) -> &Self::Output {
        self.entries[self.names[index]].borrow()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct SphereOfInfluence<'a> {
    id: SoiId,
    name: &'a str,
    children: Vec<SoiId>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let orbits = include_str!("input-test.txt")
            .lines()
            .map(build_orbit)
            .collect::<Vec<_>>();

        let system = build_system(&orbits);
        assert_eq!(system.count_all_direct_indirect(), 42);
    }
}