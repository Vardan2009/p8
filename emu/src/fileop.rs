use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub fn read_hex_file(path: &str, data: &mut [u16; 256]) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line_num == 0 && line.starts_with("v2.0") {
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if index >= data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "file has more entries than the array can hold",
            ));
        }

        let value = u16::from_str_radix(line, 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("line {}: {}", line_num + 1, e))
        })?;

        data[index] = value;
        index += 1;
    }

    Ok(())
}

pub fn read_hex_file_u8(path: &str, data: &mut [u8; 128]) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line_num == 0 && line.starts_with("v2.0") {
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if index >= data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "file has more entries than the array can hold",
            ));
        }

        let value = u8::from_str_radix(line, 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("line {}: {}", line_num + 1, e))
        })?;

        data[index] = value;
        index += 1;
    }

    Ok(())
}


