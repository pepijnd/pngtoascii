use std::io::prelude::*;
use std::fs::File;

static CHARACTERS: &str = " .&%############";

fn calc_l(c: &[u8]) -> u8 {
    (0.2*(c[0] as f32) + 0.7*(c[1] as f32) + 0.1*(c[2] as f32)) as u8
}

fn calc_ln(c: &[u8], min: u8, max: u8) -> u8 {
    (((0.2*(c[0] as f32) + 0.7*(c[1] as f32) + 0.1*(c[2] as f32)) as u8 - min) as f32 * ((255f32 - min as f32)/(max as f32 - min as f32))) as u8
}

fn main() -> std::result::Result<(), Box<std::error::Error>> {
    let decoder = png::Decoder::new(File::open("input.png")?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;
    let data = buf.into_boxed_slice();
/*
    let mut max = 0;
    let mut min = 255;

    for i in (0..info.width*info.height).step_by(3) {
        let n = i as usize;
        let c = calc_l(&data[n..n+3]);
        if c < min {
            min = c;
        }
        if c > max {
            max = c;
        }
    }*/

    let mut file = File::create("output.txt")?;

    for h in (0..info.height).step_by(1) {
        let mut row = String::new();
        for w in (0..info.width).step_by(1) {
            let i = (w*h*3) as usize;
            let c = (calc_l(&data[i..i+3])/32) as usize;
            row.push(CHARACTERS.chars().nth(c).unwrap());
        }
        row.push_str("\n");
        file.write(row.as_bytes())?;
    }
    file.flush()?;
    //println!("{} {}", min, max);
    Ok(())
}
