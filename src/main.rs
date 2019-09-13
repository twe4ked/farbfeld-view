//! http://tools.suckless.org/farbfeld/
//!
//! Bytes  | Description
//! -------+-----------------------------------------------------------------
//! 8      | "farbfeld" magic value
//! 4      | 32-Bit BE unsigned integer (width)
//! 4      | 32-Bit BE unsigned integer (height)
//! [2222] | 4⋅16-Bit BE unsigned integers [RGBA] / pixel, row-major

use minifb::{Window, WindowOptions};
use std::io::{self, BufReader, Error, Read, Stdin};
use std::process;

fn main() {
    draw().unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });
}

fn draw() -> Result<(), Error> {
    let mut reader = BufReader::new(io::stdin());

    read_header(&mut reader)?;
    let width = read_u32(&mut reader)? as usize;
    let height = read_u32(&mut reader)? as usize;
    let image_data = read_image_data(&mut reader, width * height)?;

    // TODO: Handle alpha channel
    let buffer: Vec<u32> = image_data
        .iter()
        .map(|(r, g, b, _a)| (*r as u32) << 16 | (*g as u32) << 8 | (*b as u32))
        .collect();

    let mut window = Window::new("Farbfeld", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        window.update_with_buffer(&buffer).unwrap();
    }

    Ok(())
}

fn read_header(reader: &mut BufReader<Stdin>) -> Result<(), Error> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    assert_eq!(buffer, "farbfeld".as_bytes(), "incorrect magic value");
    Ok(())
}

fn read_u32(reader: &mut BufReader<Stdin>) -> Result<u32, Error> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32_be(&buffer))
}

fn read_image_data(
    reader: &mut BufReader<Stdin>,
    length: usize,
) -> Result<Vec<(u16, u16, u16, u16)>, Error> {
    let mut buffer = Vec::new();
    for _ in 0..length {
        let mut rgba = [0; 8];
        reader.read_exact(&mut rgba)?;
        buffer.push((
            u16_be(&[rgba[0], rgba[1]]), // R
            u16_be(&[rgba[2], rgba[3]]), // G
            u16_be(&[rgba[4], rgba[5]]), // B
            u16_be(&[rgba[6], rgba[7]]), // A
        ));
    }
    Ok(buffer)
}

fn u16_be(data: &[u8]) -> u16 {
    assert_eq!(data.len(), 2);
    data[1] as u16 | (data[0] as u16) << 8
}

fn u32_be(data: &[u8]) -> u32 {
    assert_eq!(data.len(), 4);
    data[3] as u32 | (data[2] as u32) << 8 | (data[1] as u32) << 16 | (data[0] as u32) << 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_be() {
        assert_eq!(u16_be(&vec![0xaa, 0xbb]), 0xaabb);
    }

    #[test]
    fn test_u32_be() {
        assert_eq!(u32_be(&vec![0xaa, 0xbb, 0xcc, 0xdd]), 0xaabbccdd);
    }
}
