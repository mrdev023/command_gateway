use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    let mut unix_stream =
        UnixStream::connect(socket_path).context("Could not create stream")?;

    unix_stream
        .write(b"Hello?")       // we write bytes, &[u8]
        .context("Failed at writing onto the unix stream")?;

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    // to be filled
    Ok(())
}