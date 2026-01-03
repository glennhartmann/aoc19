use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{
    point::{Point2d, Slope},
    prep_io, printwriteln, split_by_char,
};

type P2 = Point2d<i64>;
type Slope64 = Slope<i64>;

#[derive(Clone)]
struct Blocker {
    point: P2,
    slope: Slope64,
    min_dist: f64,
    angle: f64,
}

impl Blocker {
    fn new(x: i64, y: i64, slope: Slope64, min_dist: f64) -> Self {
        Self {
            point: P2::new(x, y),
            slope,
            min_dist,
            angle: slope.get_angle(),
        }
    }

    fn x(&self) -> i64 {
        self.point.x()
    }

    fn y(&self) -> i64 {
        self.point.y()
    }
}

#[derive(PartialEq, Clone)]
struct Asteroid(P2);

impl Asteroid {
    fn new(x: i64, y: i64) -> Self {
        Self(P2::new(x, y))
    }

    fn dist(&self, other: &Asteroid) -> f64 {
        distance((self.0.x(), self.0.y()), (other.0.x(), other.0.y()))
    }

    fn is_blocked(&self, pov: &Asteroid, blockers: &HashMap<Slope64, Blocker>) -> bool {
        let slope = Slope::from_points_2d(&pov.0, &self.0).unwrap();
        match blockers.get(&slope) {
            None => false,
            Some(blocker) => pov.dist(self) > blocker.min_dist,
        }
    }

    fn x(&self) -> i64 {
        self.0.x()
    }

    fn y(&self) -> i64 {
        self.0.y()
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 10).unwrap();
    let split_contents: Vec<_> = contents.iter().map(|line| split_by_char(line)).collect();

    let mut asteroids = Vec::new();
    for (y, line) in split_contents.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell == "#" {
                asteroids.push(Asteroid::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    let (pov, blockers) = part1(&mut writer, &asteroids);
    part2(&mut writer, asteroids, &pov, blockers);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    asteroids: &Vec<Asteroid>,
) -> (Asteroid, HashMap<Slope64, Blocker>) {
    let mut max_count = None;
    let mut max_pov = None;
    let mut max_blockers = None;
    for pov in asteroids {
        let blockers = find_blockers(pov, asteroids);

        let mut count = 0;
        for asteroid in asteroids {
            if asteroid == pov {
                continue;
            }

            if !asteroid.is_blocked(pov, &blockers) {
                count += 1;
            }
        }

        match max_count {
            None => max_count = Some(count),
            Some(mc) => {
                if count > mc {
                    max_count = Some(count);
                }
            }
        }

        match max_pov {
            None => max_pov = Some(pov),
            Some(_) => {
                if count == max_count.expect("must") {
                    max_pov = Some(pov);
                }
            }
        }

        match max_blockers {
            None => max_blockers = Some(blockers),
            Some(_) => {
                if count == max_count.expect("must") {
                    max_blockers = Some(blockers);
                }
            }
        }
    }

    let max_pov_concrete = max_pov.expect("no max pov found");
    println!("pov: ({}, {})", max_pov_concrete.x(), max_pov_concrete.y());
    printwriteln!(writer, "{}", max_count.expect("no max count found")).unwrap();

    (
        max_pov_concrete.clone(),
        max_blockers.expect("no back blockers found"),
    )
}

fn find_blockers(pov: &Asteroid, asteroids: &Vec<Asteroid>) -> HashMap<Slope64, Blocker> {
    let mut blockers: HashMap<Slope64, Blocker> = HashMap::new();
    for asteroid in asteroids {
        if asteroid == pov {
            continue;
        }

        let slope = Slope::from_points_2d(&pov.0, &asteroid.0).unwrap();
        let dist = pov.dist(asteroid);
        blockers
            .entry(slope)
            .and_modify(|e| {
                if dist < e.min_dist {
                    *e = Blocker::new(asteroid.x(), asteroid.y(), slope, dist);
                }
            })
            .or_insert(Blocker::new(asteroid.x(), asteroid.y(), slope, dist));
    }
    blockers
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    mut asteroids: Vec<Asteroid>,
    pov: &Asteroid,
    mut blockers: HashMap<Slope64, Blocker>,
) {
    let mut i = 1;
    let mut two_hundredth = None;
    while asteroids.len() > 1 {
        let mut blockers_by_angle: Vec<_> = blockers.values().cloned().collect();
        blockers_by_angle.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());

        for b in &blockers_by_angle {
            if i == 200 {
                two_hundredth = Some(b.clone());
            }
            println!(
                "the {}th asteroid to be vapourized is at ({}, {}) with a slope of {} / {} and an angle of {} rad",
                i,
                b.x(),
                b.y(),
                b.slope.horizontal(),
                b.slope.vertical(),
                b.angle
            );
            i += 1;
        }

        asteroids = asteroids
            .iter()
            .filter(|e| {
                if *e == pov {
                    return true;
                }
                match blockers.get(&Slope::from_points_2d(&pov.0, &e.0).unwrap()) {
                    None => true,
                    Some(blocker) => e.x() != blocker.x() || e.y() != blocker.y(),
                }
            })
            .cloned()
            .collect();
        blockers = find_blockers(pov, &asteroids);
    }

    let two_hundredth_concrete = two_hundredth.expect("no 200th found");
    printwriteln!(
        writer,
        "200th: ({}, {}): {}",
        two_hundredth_concrete.x(),
        two_hundredth_concrete.y(),
        two_hundredth_concrete.x() * 100 + two_hundredth_concrete.y()
    )
    .unwrap();
}

fn distance(a: (i64, i64), b: (i64, i64)) -> f64 {
    ((b.0 as f64 - a.0 as f64).powi(2) + (b.1 as f64 - a.1 as f64).powi(2)).sqrt()
}
