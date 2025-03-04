use std::net::SocketAddr;

use tokio::{
    sync::{oneshot, broadcast, mpsc::{self, Sender}},
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

use serde::{Serialize, Deserialize};

use crate::app::{
    Player,
    Message,
};

struct Server {
    players: Vec<Player>,
    owner: Player,
}

impl Server {
    pub fn new(owner: Player) -> Self {
        Self { players: Vec::with_capacity(7), owner }
    }
}

pub fn create_server(owner: Player) {
    let server = Server::new(owner);

    tokio::spawn(async move {
        // server code

        let listener = TcpListener::bind("127.0.0.1:50000").await.unwrap();
        let (tx, mut rx) = mpsc::channel(16);
        
        // Lobby
        loop {
            tokio::select! {
                Ok((socket, addr)) = listener.accept() => {
                    // hago algo con el socket
                    handle_user(socket, addr, tx.clone());
                },
                Some(msg) = rx.recv() => {
                    match msg {
                        (addr, Message::GameStart) => { 
                            if server.players.len() >= 3 {
                                break;
                            }
                        },
                        _ => {},
                    }
                },
            }
        }

        // Game
        loop {
            break;
        }
    });
}

fn handle_user(mut socket: TcpStream, addr: SocketAddr, tx: Sender<(SocketAddr, Message)>) {
    tokio::spawn(async move {
        loop {
            let mut buffer = vec![0; 64];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    match serde_json::from_slice(&buffer) {
                        Ok(msg) => {
                            tx.send((addr, msg));
                        },
                        _ => {},
                    }
                },
                Err(e) => eprintln!("Failed to read from socket: {}", e),
                _ => {}
            }
        }
    });
}

pub fn join_server() {
    
}
