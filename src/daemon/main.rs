#![cfg_attr(not(unix), allow(unused_imports))]

use std::sync::Mutex;
mod server;
mod sessions;

pub(self) static SESSIONS : Mutex<Vec<libcommand::Session>> = Mutex::new(Vec::new());

pub use sessions::{
    get_sessions_lock,
    remove_session_by_id
};

use std::path::Path;
use std::time::Duration;
#[cfg(unix)]
use tokio::net::UnixListener;

#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

use libcommand::interpreter::unix_server::UnixServer;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(Path::new(libcommand::SOCK_FILE).parent().unwrap())?;

    let server = server::DaemonServer::default();

    let uds = UnixListener::bind(libcommand::SOCK_FILE)?;
    let uds_stream = UnixListenerStream::new(uds);

    std::thread::spawn(|| {
        loop {
            if let Ok(lock) = get_sessions_lock() {
                println!("Sessions log");
                for session in lock.iter() {
                    println!("ID: {id}, PID: {pid}", id = session.id, pid = session.pid);
                }
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    Server::builder()
        .add_service(UnixServer::new(server))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}