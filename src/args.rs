#[derive(clap::Parser)]
pub struct SendArgs {
    pub address: String,
    pub path: String,
}

#[derive(clap::Parser)]
pub struct ReceiveArgs {
    pub address: String,
    pub path: String,
}

#[derive(clap::Parser)]
pub enum Args {
    Send(SendArgs),
    Receive(ReceiveArgs),
}
