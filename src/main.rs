use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::io::Read;
use std::{collections::BTreeMap, fs};

fn main() {
    let b = entry_map(read_from_dir());
    let mut s = String::new();
    for (k, v) in &b {
        s.push_str(&format!("{}: {}\n", k, v));
    }
    let f = open_file(get_file_name(b, &s));
    if f.contains("P3") {
        create_image(f).save("output.png").unwrap();
        println!("Finished and saved as output.png!");
    } else {
        println!("Not formatted correctly! Only works with P3!")
    }
    pause();
}
fn read_from_dir() -> Vec<String> {
    let b = fs::read_dir("./").unwrap();
    let mut h = vec![];
    for i in b {
        let q = i.unwrap();
        let s = q.file_name().to_str().unwrap().to_string();

        if s.contains(".ppm") {
            h.push(s);
        }
    }
    h
}

fn entry_map(v: Vec<String>) -> BTreeMap<usize, String> {
    (1..=v.len()).zip(v.into_iter()).collect()
}

fn input(m: &str) -> Result<String, std::io::Error> {
    println!("{}", m);
    let mut b = String::new();
    std::io::stdin().read_line(&mut b)?;
    Ok(b)
}
fn input_safe(p: &str) -> String {
    match input(p) {
        Err(_) => input_safe("An error occurred, please enter again"),
        Ok(s) => s,
    }
}
fn get_file_name(map: BTreeMap<usize, String>, s: &str) -> String {
    match map.get(&input_safe(&s).trim().parse::<usize>().unwrap()) {
        Some(s) => s.to_string(),
        None => get_file_name(map, "That is not a valid entry!"),
    }
}

fn open_file(name: String) -> String {
    let mut f = File::open(name).unwrap();
    let mut b = vec![];
    f.read_to_end(&mut b).unwrap();
    read_string(&b).unwrap().replace('\u{feff}', "")
}
fn read_string(bytes: &[u8]) -> Option<String> {
    let (front, slice, back) = unsafe { bytes.align_to::<u16>() };
    if front.is_empty() && back.is_empty() {
        String::from_utf16(slice).ok()
    } else {
        None
    }
}
#[allow(clippy::needless_range_loop)]
fn create_image(s: String) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut v = s.lines().collect::<Vec<_>>();
    let dims = v[1]
        .split_ascii_whitespace()
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let img_width = dims[1];
    let mut image = RgbImage::new(img_width, dims[1]);
    let colour_shift = v[2].trim().parse::<u8>().unwrap() as f64;
    v = v[3..].to_vec();
    for i in 0..v.len() {
        let colours = v[i]
            .split_ascii_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        image.put_pixel(
            (i % img_width as usize) as u32,
            (i / img_width as usize) as u32,
            Rgb([
                (colours[0] as f64 / colour_shift * 255.0) as u8,
                (colours[1] as f64 / colour_shift * 255.0) as u8,
                (colours[2] as f64 / colour_shift * 255.0) as u8,
            ]),
        );
    }
    image
}
fn pause() {
    use std::io::{stdin, stdout, Write};
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
