use std::{
    io::{BufWriter, Write},
    str::FromStr,
};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 1).unwrap();
    let masses: Vec<u32> = contents.iter().map(|s| u32::from_str(s).unwrap()).collect();

    part1(&mut writer, &masses);
    part2(&mut writer, &masses);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, masses: &[u32]) {
    printwriteln!(writer, "{}", masses.iter().map(|m| m / 3 - 2).sum::<u32>()).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, masses: &[u32]) {
    let mut total: u32 = 0;
    for mass in masses {
        let mut total_fuel = 0;
        let mut prev_mass = *mass;
        loop {
            let fuel = (prev_mass / 3).saturating_sub(2);
            if fuel == 0 {
                break;
            }
            total_fuel += fuel;
            prev_mass = fuel;
        }
        total += total_fuel;
    }
    printwriteln!(writer, "{}", total).unwrap();
}
