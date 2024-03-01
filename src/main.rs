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
    let mut connection = TcpStream::connect(address)?;
    let file_contents = read_file(path)?;

    connection.write(&file_contents)?;
    
    Ok(())
}

fn receive(path: String, address: String) -> anyhow::Result<()> { 
    let listener = TcpListener::bind(address)?;
    let file_contents = read_from_listener(listener)?; 
    
    File::create(path)?.write_all(&file_contents)?;
    
    Ok(())
}

fn read_file(path: String) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut file_contents = Vec::<u8>::new();

    file.read_to_end(&mut file_contents)?;

    Ok(file_contents)
}

fn read_from_listener(listener: TcpListener) -> anyhow::Result<Vec<u8>> {
    let mut contents = listener.accept()?.0;
    let mut file_contents = Vec::<u8>::new();

    contents.read_to_end(&mut file_contents)?;

    Ok(file_contents)
}