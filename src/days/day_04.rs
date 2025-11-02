use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 4).unwrap();
    let input: Vec<u32> = split_and_parse(contents[0], "-").unwrap();

    part1(&mut writer, &input);
    part2(&mut writer, &input);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, input: &[u32]) {
    let mut total = 0;
    for i in input[0]..=input[1] {
        let digs = digits(i);
        if !has_repeated_digits(&digs) {
            continue;
        }
        if !is_non_decreasing(&digs) {
            continue;
        }
        total += 1;
    }
    printwriteln!(writer, "{}", total).unwrap();
}

fn has_repeated_digits(digs: &[u8]) -> bool {
    for i in 0..(digs.len() - 1) {
        if digs[i] == digs[i + 1] {
            return true;
        }
    }
    false
}

fn part2<W: Write>(writer: &mut BufWriter<W>, input: &[u32]) {
    let mut total = 0;
    for i in input[0]..=input[1] {
        let digs = digits(i);
        if !has_repeated_digits_part_2(&digs) {
            continue;
        }
        if !is_non_decreasing(&digs) {
            continue;
        }
        total += 1;
    }
    printwriteln!(writer, "{}", total).unwrap();
}

fn has_repeated_digits_part_2(digs: &[u8]) -> bool {
    let mut i = 0;
    while i < digs.len() {
        let mut j = i + 1;
        while j <= digs.len() {
            if j == digs.len() || digs[j] != digs[i] {
                if j - i == 2 {
                    return true;
                }
                i = j - 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }
    false
}

fn is_non_decreasing(digs: &[u8]) -> bool {
    for i in 0..(digs.len() - 1) {
        if digs[i + 1] < digs[i] {
            return false;
        }
    }
    true
}

fn digits(mut i: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(6);
    while i > 0 {
        v.push((i % 10).try_into().unwrap());
        i /= 10;
    }
    v.reverse();
    v
}
