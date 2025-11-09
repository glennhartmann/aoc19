use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln, split_by_char};

#[derive(Copy, Clone)]
struct Blocker {
    x: i32,
    y: i32,
    slope: Slope,
    min_dist: f32,
    angle: f32,
}

impl Blocker {
    fn new(x: i32, y: i32, slope: Slope, min_dist: f32) -> Self {
        Self {
            x,
            y,
            slope,
            min_dist,
            angle: match slope {
                Slope {
                    horizontal: 0,
                    vertical: 0,
                } => panic!("0 slope"),

                // up
                Slope {
                    horizontal: 0,
                    vertical: i32::MIN..0,
                } => 0.0,

                // right
                Slope {
                    horizontal: 0..=i32::MAX,
                    vertical: 0,
                } => FRAC_PI_2,

                // down
                Slope {
                    horizontal: 0,
                    vertical: 0..=i32::MAX,
                } => PI,

                // left
                Slope {
                    horizontal: i32::MIN..0,
                    vertical: 0,
                } => PI + FRAC_PI_2,

                // top-right
                Slope {
                    horizontal: 0..=i32::MAX,
                    vertical: i32::MIN..0,
                } => (slope.horizontal as f32 / -slope.vertical as f32).atan(),

                // bottom-right
                Slope {
                    horizontal: 0..=i32::MAX,
                    vertical: 0..=i32::MAX,
                } => (slope.vertical as f32 / slope.horizontal as f32).atan() + FRAC_PI_2,

                // bottom-left
                Slope {
                    horizontal: i32::MIN..0,
                    vertical: 0..=i32::MAX,
                } => (-slope.horizontal as f32 / slope.vertical as f32).atan() + PI,

                // top-left
                Slope {
                    horizontal: i32::MIN..0,
                    vertical: i32::MIN..0,
                } => (-slope.vertical as f32 / -slope.horizontal as f32).atan() + PI + FRAC_PI_2,
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Slope {
    horizontal: i32,
    vertical: i32,
}

impl Slope {
    fn from(pov: &Asteroid, asteroid: &Asteroid) -> Self {
        let mut s = Self {
            horizontal: asteroid.x - pov.x,
            vertical: asteroid.y - pov.y,
        };
        s.simplify();
        s
    }

    fn simplify(&mut self) {
        let gcd = Self::euclidean_algorithm(self.horizontal.abs(), self.vertical.abs());
        self.horizontal /= gcd;
        self.vertical /= gcd;
    }

    fn euclidean_algorithm(a: i32, b: i32) -> i32 {
        if a < 0 || b < 0 {
            panic!("should only be used on positive numbers");
        }

        if b == 0 {
            return a;
        }

        Self::euclidean_algorithm(b, a % b)
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Asteroid) -> f32 {
        distance((self.x, self.y), (other.x, other.y))
    }

    fn is_blocked(&self, pov: &Asteroid, blockers: &HashMap<Slope, Blocker>) -> bool {
        let slope = Slope::from(pov, self);
        match blockers.get(&slope) {
            None => false,
            Some(blocker) => pov.dist(self) > blocker.min_dist,
        }
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
) -> (Asteroid, HashMap<Slope, Blocker>) {
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
    println!("pov: ({}, {})", max_pov_concrete.x, max_pov_concrete.y);
    printwriteln!(writer, "{}", max_count.expect("no max count found")).unwrap();

    (
        *max_pov_concrete,
        max_blockers.expect("no back blockers found"),
    )
}

fn find_blockers(pov: &Asteroid, asteroids: &Vec<Asteroid>) -> HashMap<Slope, Blocker> {
    let mut blockers: HashMap<Slope, Blocker> = HashMap::new();
    for asteroid in asteroids {
        if asteroid == pov {
            continue;
        }

        let slope = Slope::from(pov, asteroid);
        let dist = pov.dist(asteroid);
        blockers
            .entry(slope)
            .and_modify(|e| {
                if dist < e.min_dist {
                    *e = Blocker::new(asteroid.x, asteroid.y, slope, dist);
                }
            })
            .or_insert(Blocker::new(asteroid.x, asteroid.y, slope, dist));
    }
    blockers
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    mut asteroids: Vec<Asteroid>,
    pov: &Asteroid,
    mut blockers: HashMap<Slope, Blocker>,
) {
    let mut i = 1;
    let mut two_hundredth = None;
    while asteroids.len() > 1 {
        let mut blockers_by_angle: Vec<_> = blockers.values().collect();
        blockers_by_angle.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());

        for b in &blockers_by_angle {
            if i == 200 {
                two_hundredth = Some(**b);
            }
            println!(
                "the {}th asteroid to be vapourized is at ({}, {}) with a slope of {} / {} and an angle of {} rad",
                i, b.x, b.y, b.slope.horizontal, b.slope.vertical, b.angle
            );
            i += 1;
        }

        asteroids = asteroids
            .iter()
            .filter(|e| {
                if *e == pov {
                    return true;
                }
                match blockers.get(&Slope::from(pov, e)) {
                    None => true,
                    Some(blocker) => e.x != blocker.x || e.y != blocker.y,
                }
            })
            .copied()
            .collect();
        blockers = find_blockers(pov, &asteroids);
    }

    let two_hundredth_concrete = two_hundredth.expect("no 200th found");
    printwriteln!(
        writer,
        "200th: ({}, {}): {}",
        two_hundredth_concrete.x,
        two_hundredth_concrete.y,
        two_hundredth_concrete.x * 100 + two_hundredth_concrete.y
    )
    .unwrap();
}

fn distance(a: (i32, i32), b: (i32, i32)) -> f32 {
    ((b.0 as f32 - a.0 as f32).powi(2) + (b.1 as f32 - a.1 as f32).powi(2)).sqrt()
}
