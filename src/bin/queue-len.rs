use clap::Parser;
use mpd::{Client, Idle, Subsystem};
use std::io::{Read, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify a hostname or IP address to connect to. Defaults to `localhost`.
    #[arg(long)]
    host: Option<String>,

    /// Specify a port number to connect to. Defaults to `6600`.
    #[arg(long, short)]
    port: Option<u16>,

    /// Maximum length of the queue
    length: Option<u32>,
}

fn trim_queue<S: Read + Write>(conn: &mut Client<S>, len: u32) -> mpd::error::Result<()> {
    let queue_len = conn.status()?.queue_len;
    if queue_len > len {
        conn.delete(0..(queue_len - len))?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let length = args.length.unwrap_or(10);

    let host = args
        .host
        .or_else(|| std::env::var("MPD_HOST").ok())
        .unwrap_or_else(|| String::from("localhost"));
    let port = args
        .port
        .or_else(|| std::env::var("MPD_PORT").ok().and_then(|v| v.parse().ok()))
        .unwrap_or(6600);
    let mut conn = Client::connect((host, port)).unwrap();

    loop {
        trim_queue(&mut conn, length).unwrap();
        conn.wait(&[Subsystem::Queue]).unwrap();
    }
}
