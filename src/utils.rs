use io::{BufRead, Read, Write};
use std::io;

pub fn read<R: io::Read>(reader: &mut io::BufReader<R>) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_line<R: io::Read>(reader: &mut io::BufReader<R>) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf)
}

pub fn write<W: io::Write>(writer: &mut io::BufWriter<W>, content: &[u8]) -> io::Result<()> {
    writer.write_all(content)?;
    Ok(())
}
