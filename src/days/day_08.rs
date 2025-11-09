use std::{
    io::{BufWriter, Write},
    str::FromStr,
};

use aoclib_rs::{prep_io, printwriteln, split_by_char};

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

struct Image {
    layers: Vec<Layer>,
}

struct Layer {
    pixels: Vec<Vec<u8>>,
}

impl Image {
    fn render(&self) -> Vec<Vec<char>> {
        let mut r = vec![vec!['3'; WIDTH]; HEIGHT];
        for (row_index, row) in r.iter_mut().enumerate().take(HEIGHT) {
            for (col_index, p) in row.iter_mut().enumerate().take(WIDTH) {
                for layer in &self.layers {
                    let colour = Colour::try_from(layer.pixels[row_index][col_index]).unwrap();
                    if colour != Colour::Transparent {
                        *p = colour.into();
                        break;
                    }
                }
            }
        }
        r
    }
}

impl From<Vec<&str>> for Image {
    fn from(contents: Vec<&str>) -> Self {
        let pixels1d: Vec<u8> = split_by_char(contents[0])
            .iter()
            .map(|c| u8::from_str(c).unwrap())
            .collect();

        let mut img = Image { layers: Vec::new() };

        let mut layer = 0;
        let mut row = 0;
        let mut column = 0;
        for p in pixels1d {
            if column == 0 {
                if row == 0 {
                    img.layers.push(Layer { pixels: Vec::new() });
                }
                img.layers[layer].pixels.push(Vec::new());
            }

            img.layers[layer].pixels[row].push(p);

            column = (column + 1) % WIDTH;
            if column == 0 {
                row = (row + 1) % HEIGHT;
                if row == 0 {
                    layer += 1;
                }
            }
        }

        img
    }
}

impl Layer {
    fn count_digits(&self, digit: u8) -> u32 {
        let mut total = 0;
        for row in &self.pixels {
            for p in row {
                if *p == digit {
                    total += 1;
                }
            }
        }
        total
    }
}

#[derive(PartialEq)]
enum Colour {
    Black,
    White,
    Transparent,
}

impl TryFrom<u8> for Colour {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Colour::Black),
            1 => Ok(Colour::White),
            2 => Ok(Colour::Transparent),
            _ => Err(format!("invalid colour: {}", value)),
        }
    }
}

impl From<Colour> for char {
    fn from(val: Colour) -> Self {
        match val {
            Colour::Black => ' ',
            Colour::White => '█',
            Colour::Transparent => '4',
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 8).unwrap();
    let img = Image::from(contents);

    part1(&mut writer, &img);
    part2(&mut writer, &img);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, img: &Image) {
    for layer in &img.layers {
        for row in &layer.pixels {
            for p in row {
                print!("{p}");
            }
            println!();
        }
        println!();
    }

    let mut least_0s = None;
    let mut least_1s = None;
    let mut least_2s = None;
    for layer in &img.layers {
        match least_0s {
            None => {
                least_0s = Some(layer.count_digits(0));
                least_1s = Some(layer.count_digits(1));
                least_2s = Some(layer.count_digits(2));
            }
            Some(prev_least_0s) => {
                let curr_least_0s = layer.count_digits(0);
                if curr_least_0s < prev_least_0s {
                    least_0s = Some(curr_least_0s);
                    least_1s = Some(layer.count_digits(1));
                    least_2s = Some(layer.count_digits(2));
                }
            }
        }
    }

    printwriteln!(
        writer,
        "{}",
        least_1s.expect("no 1s") * least_2s.expect("no 2s")
    )
    .unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, img: &Image) {
    let r = img.render();
    for row in r {
        for p in row {
            print!("{p}");
        }
        println!();
    }
    printwriteln!(writer, "CYUAH").unwrap();
}
