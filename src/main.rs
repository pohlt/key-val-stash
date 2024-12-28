mod database;
mod message;

use database::Database;
use message::{Command, Message, MAX_MESSAGE_LENGTH};
use std::{
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

const SERVER_ADDRESS: &str = "0.0.0.0";
const SERVER_PORT: u16 = 28536;
const DATABASE_FILEPATH: &str = "database.cbor";

fn process_message(buf: &[u8], db: &mut Database) -> Result<Vec<u8>, String> {
    Message::unpack(buf).and_then(|msg| {
        let response = match msg.command {
            Command::Get => Message {
                command: Command::Ok,
                key: msg.key,
                value: db.get(&msg.key).cloned().unwrap_or_default(),
            },
            Command::Put => {
                db.put(&msg.key, msg.value);
                Message {
                    command: Command::Ok,
                    key: msg.key,
                    value: Vec::new(),
                }
            }
            Command::Keep => {
                db.keep(&msg.key);
                Message {
                    command: Command::Ok,
                    key: msg.key,
                    value: Vec::new(),
                }
            }
            _ => Message {
                command: Command::NOk,
                key: msg.key,
                value: Vec::new(),
            },
        };
        response.pack()
    })
}

fn main() -> std::io::Result<()> {
    let mut db = Database::new(Some(DATABASE_FILEPATH));

    let mut buf = [0; MAX_MESSAGE_LENGTH];
    let socket = UdpSocket::bind((SERVER_ADDRESS, SERVER_PORT))?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    socket.set_write_timeout(Some(Duration::from_millis(500)))?;

    let run_loop = Arc::new(AtomicBool::new(true));
    let r = run_loop.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("error setting Ctrl-C handler");

    println!("starting server on {}:{}", SERVER_ADDRESS, SERVER_PORT);
    let mut last_purge = Instant::now();

    while run_loop.load(Ordering::SeqCst) {
        let now = Instant::now();
        if now.duration_since(last_purge).as_secs() > 24 * 60 * 60 {
            db.age_and_purge();
            last_purge = now;
        }

        match socket.recv_from(&mut buf) {
            Ok((msg_length, src)) => {
                if let Ok(response) = process_message(&buf[..msg_length], &mut db) {
                    if let Err(e) = socket.send_to(&response, src) {
                        eprintln!("error sending response: {}", e);
                    }
                } else {
                    eprintln!("error processing message");
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => eprintln!("error: {}", e),
        }
    }
    let _ = db.save(DATABASE_FILEPATH);
    Ok(())
}
