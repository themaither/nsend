use clap::Parser;
use std::{fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}};
mod args;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    
    match args {
        args::Args::Send(input) => send(input.path, input.address)?,
        args::Args::Receive(input) => receive(input.path, input.address)?,
    }

    Ok(())
}

fn send(path: String, address: String) -> anyhow::Result<()>{
    
    let mut file = File::open(path)?;
    let mut connection = TcpStream::connect(address)?;

    let mut file_contents = Vec::<u8>::new();
    file.read_to_end(&mut file_contents)?;
    
    connection.write(&file_contents)?;
    
    Ok(())
}

fn receive(path: String, address: String) -> anyhow::Result<()> {
    
    let listener = TcpListener::bind(address)?;
    
    let mut contents = listener.accept()?.0;
    let mut file_contents = Vec::<u8>::new();
    contents.read_to_end(&mut file_contents)?;

    File::create(path)?.write_all(&file_contents)?;

    Ok(())
}