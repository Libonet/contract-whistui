use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        broadcast,
        mpsc::{self, Sender},
        oneshot,
    },
};

use serde::{Deserialize, Serialize};

use crate::app::{Message, Player};

struct Server {
    players: Vec<Player>,
    host_pos: usize,
}

impl Server {
    pub fn new(host: Player) -> Self {
        let mut player_list = Vec::with_capacity(7);
        player_list.push(host);
        Self {
            players: player_list,
            host_pos: 0,
        }
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
                Some(data) = rx.recv() => {
                    let (addr, msg) = data;

                    match msg {
                        Message::GameStart => break,
                        Message::Rename(str) => {},
                        _ => {},
                    }
                },
            }
        }

        // Game
        loop {
            todo!("Implement game handling")
        }
    });
}

fn handle_user(mut socket: TcpStream, addr: SocketAddr, tx: Sender<(SocketAddr, Message)>) {
    tokio::spawn(async move {
        loop {
            let mut buffer = vec![0; 64];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    if let Ok(msg) = serde_json::from_slice(&buffer) {
                        match tx.send((addr, msg)).await {
                            Ok(()) => {}
                            Err(_err) => todo!("Handle send errors"),
                        }
                    }
                }
                Err(e) => eprintln!("Failed to read from socket: {}", e),
                _ => {}
            }
        }
    });
}

pub fn join_server() {
    todo!("Implement join_server")
}
