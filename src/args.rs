#[derive(clap::Parser)]
pub struct SendArgs {
    pub address: String,
    pub path: Option<String>,
}

#[derive(clap::Parser)]
pub struct ReceiveArgs {
    pub address: String,
    pub path: Option<String>,
}

#[derive(clap::Parser)]
pub enum Args {
    Send(SendArgs),
    Receive(ReceiveArgs),
}
