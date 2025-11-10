use std::{
    fmt,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

use {once_cell::sync::Lazy, regex::Regex};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Moon {
    position: ThreeDVal,
    velocity: ThreeDVal,
}

impl Moon {
    fn new(position: ThreeDVal) -> Self {
        Self {
            position,
            velocity: ThreeDVal::zero(),
        }
    }

    fn apply_velocity(&mut self) {
        self.position = ThreeDVal::new(
            self.position.x + self.velocity.x,
            self.position.y + self.velocity.y,
            self.position.z + self.velocity.z,
        );
    }

    fn total_energy(&self) -> i32 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

impl From<&str> for Moon {
    fn from(s: &str) -> Moon {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap());
        let Some(caps) = RE.captures(s) else {
            panic!("bad input");
        };

        Moon::new(ThreeDVal::new(
            caps[1].parse::<i32>().unwrap(),
            caps[2].parse::<i32>().unwrap(),
            caps[3].parse::<i32>().unwrap(),
        ))
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ThreeDVal {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for ThreeDVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl ThreeDVal {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 12).unwrap();

    let mut moons: Vec<Moon> = Vec::with_capacity(4);
    for line in contents {
        moons.push(Moon::from(line));
    }

    part1(&mut writer, moons.clone());
}

fn part1<W: Write>(writer: &mut BufWriter<W>, mut moons: Vec<Moon>) {
    print_moons(&moons, 0);
    for i in 0..1000 {
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                (moons[i].velocity.x, moons[j].velocity.x) = apply_gravity_to_dimension(
                    moons[i].position.x,
                    moons[j].position.x,
                    moons[i].velocity.x,
                    moons[j].velocity.x,
                );
                (moons[i].velocity.y, moons[j].velocity.y) = apply_gravity_to_dimension(
                    moons[i].position.y,
                    moons[j].position.y,
                    moons[i].velocity.y,
                    moons[j].velocity.y,
                );
                (moons[i].velocity.z, moons[j].velocity.z) = apply_gravity_to_dimension(
                    moons[i].position.z,
                    moons[j].position.z,
                    moons[i].velocity.z,
                    moons[j].velocity.z,
                );
            }
        }

        for moon in &mut moons {
            moon.apply_velocity();
        }

        if (i + 1) % 100 == 0 {
            print_moons(&moons, i + 1);
        }
    }

    let mut total = 0;
    for moon in moons {
        total += moon.total_energy();
    }

    printwriteln!(writer, "{}", total).unwrap();
}

fn apply_gravity_to_dimension(
    pos_a: i32,
    pos_b: i32,
    mut vel_a: i32,
    mut vel_b: i32,
) -> (i32, i32) {
    match pos_a - pos_b {
        0 => {}
        i32::MIN..0 => {
            vel_a += 1;
            vel_b -= 1;
        }
        1..=i32::MAX => {
            vel_a -= 1;
            vel_b += 1;
        }
    }
    (vel_a, vel_b)
}

fn print_moons(moons: &Vec<Moon>, step: usize) {
    println!("After {} steps:", step);
    for moon in moons {
        println!("{}", moon);
    }
    println!();
}
