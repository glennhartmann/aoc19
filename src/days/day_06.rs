use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

struct Object {
    direct_orbit: Option<String>,
}

impl Object {
    fn new(direct_orbit: Option<String>) -> Self {
        Self { direct_orbit }
    }

    fn set_direct_orbit(&mut self, direct_orbit: String) {
        self.direct_orbit = Some(direct_orbit);
    }
}

struct System {
    objects: HashMap<String, Object>,
}

impl System {
    fn add_direct_orbit(&mut self, orbitee_name: &str, orbiter_name: &str) {
        self.objects
            .entry(orbitee_name.into())
            .or_insert(Object::new(None));
        self.objects
            .entry(orbiter_name.into())
            .and_modify(|e| e.set_direct_orbit(orbitee_name.into()))
            .or_insert(Object::new(Some(orbitee_name.into())));
    }

    fn get_all_direct_and_indirect_orbits(&self) -> u32 {
        let mut indirect_orbits: HashMap<String, u32> = HashMap::new();
        let (mut total, mut cache_hits, mut cache_misses) = (0, 0, 0);
        for name in self.objects.keys() {
            total += self.get_indirect_orbits(
                name,
                &mut indirect_orbits,
                &mut cache_hits,
                &mut cache_misses,
            );
        }
        println!("cache hits: {cache_hits}, cache misses: {cache_misses}");
        total
    }

    fn get_indirect_orbits(
        &self,
        name: &str,
        indirect_orbits: &mut HashMap<String, u32>,
        cache_hits: &mut u32,
        cache_misses: &mut u32,
    ) -> u32 {
        if let Some(curr_indirects) = indirect_orbits.get(name) {
            *cache_hits += 1;
            return *curr_indirects;
        }
        *cache_misses += 1;

        let curr_indirects = match &self.objects.get(name).expect("missing object").direct_orbit {
            None => 0,
            Some(direct_orbit) => {
                self.get_indirect_orbits(direct_orbit, indirect_orbits, cache_hits, cache_misses)
                    + 1
            }
        };
        indirect_orbits.insert(name.to_string(), curr_indirects);
        curr_indirects
    }

    fn get_direct_orbit(&self, name: &str) -> String {
        self.objects
            .get(name)
            .expect("missing object")
            .direct_orbit
            .clone()
            .expect("object has no direct orbit")
    }

    fn get_dist(&self, src: String, dst: String) -> u32 {
        let mut dst_dists: HashMap<String, u32> = HashMap::new();
        let mut curr = &dst;
        let mut dist = 0;
        loop {
            dst_dists.insert(curr.clone(), dist);
            match &self.objects.get(curr).expect("missing object").direct_orbit {
                None => break,
                Some(next) => curr = next,
            }
            dist += 1;
        }

        curr = &src;
        dist = 0;
        loop {
            if let Some(dst_dist) = dst_dists.get(curr) {
                return dist + dst_dist;
            }

            match &self.objects.get(curr).expect("missing object").direct_orbit {
                None => panic!("didn't intersect paths"),
                Some(next) => curr = next,
            }

            dist += 1;
        }
    }
}

impl TryFrom<Vec<&str>> for System {
    type Error = String;

    fn try_from(contents: Vec<&str>) -> Result<Self, Self::Error> {
        let mut system = System {
            objects: HashMap::new(),
        };
        for line in contents {
            let mut sp = line.split(")");
            let orbitee = sp.next().ok_or("no orbitee")?;
            let orbiter = sp.next().ok_or("no orbiter")?;
            system.add_direct_orbit(orbitee, orbiter);
        }
        Ok(system)
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 6).unwrap();
    let system = System::try_from(contents).unwrap();

    part1(&mut writer, &system);
    part2(&mut writer, &system);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, system: &System) {
    let indirect_orbits = system.get_all_direct_and_indirect_orbits();
    printwriteln!(writer, "{}", indirect_orbits).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, system: &System) {
    let you_san_dist = system.get_dist(
        system.get_direct_orbit("YOU"),
        system.get_direct_orbit("SAN"),
    );
    printwriteln!(writer, "{}", you_san_dist).unwrap();
}
