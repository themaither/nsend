use clap::Parser;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
mod args;
mod io;
fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    
    match args {
        args::Args::Send(input) => send(input.path, input.address)?,
        args::Args::Receive(input) => receive(input.path, input.address)?,
    }

    Ok(())
}

fn send(path: Option<String>, address: String) -> anyhow::Result<()>{
    let mut connection = TcpStream::connect(address)?;
    
    let file_contents;

    match path {
        Some(a) => file_contents = io::read_file(a)?,
        None => file_contents = io::read_stdin()?,
    }
    
    connection.write(&file_contents)?;
    
    Ok(())
}

fn receive(path: String, address: String) -> anyhow::Result<()> { 
    let listener = TcpListener::bind(address)?;
    let contents = io::read_from_listener(listener)?; 
    
    io::write_file(path, contents)?;
    
    Ok(())
}

