use std;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;

pub fn write_file(path: String, contents: Vec<u8>) -> anyhow::Result<()> {
    File::create(path)?.write_all(&contents)?;

    Ok(())
}

pub fn read_file(path: String) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut file_contents = Vec::<u8>::new();

    file.read_to_end(&mut file_contents)?;

    Ok(file_contents)
}

pub fn write_stdout(contents: Vec<u8>) -> anyhow::Result<()> {
    std::io::stdout().write(&contents)?;
    
    Ok(())
}

pub fn read_stdin() -> anyhow::Result<Vec<u8>> {
    let mut input = Vec::<u8>::new();

    std::io::stdin().read_to_end(&mut input)?;

    Ok(input)
}

pub fn read_from_listener(listener: TcpListener) -> anyhow::Result<Vec<u8>> {
    let mut contents = listener.accept()?.0;
    let mut file_contents = Vec::<u8>::new();

    contents.read_to_end(&mut file_contents)?;

    Ok(file_contents)
}
