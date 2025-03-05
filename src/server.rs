use std::{error::Error, fmt::Display};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        broadcast,
        mpsc,
        oneshot,
    },
};

use crate::app::{Message, Player};

pub type Id = i8;

#[derive(Debug, PartialEq, Eq)]
enum ServerError {
    TooManyPlayers,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::TooManyPlayers => write!(f, "too many players. max is 7"),
        }
    }
}

impl Error for ServerError {}

struct Server {
    players: Vec<Player>,
    host_id: Id,
}

impl Server {
    pub fn new() -> Self {
        let player_list = Vec::with_capacity(7);
        Self {
            players: player_list,
            host_id: 0,
        }
    }

    pub fn add_user(&mut self) -> Result<Id, ServerError> {
        let id = self.players.len();
        if id >= 7 {
            return Err(ServerError::TooManyPlayers);
        }
        self.players.push(Player::new(id as Id));
        Ok(id as i8)
    }

    pub fn change_name(&mut self, id: Id, new_name: String) {
        self.players[id as usize].name = new_name;
    }

    #[allow(dead_code)]
    pub fn change_host(&mut self, new_host_id: Id) {
        self.host_id = new_host_id;
    }

    //pub fn get_player_handle(&self, id: Id) -> &oneshot::Sender<Message> {
    //    &self.players[id as usize].0
    //}
}

pub fn create_server() {
    let mut server = Server::new();

    tokio::spawn(async move {
        // server code

        let listener = TcpListener::bind("127.0.0.1:50000").await.unwrap();
        let (tx, mut rx) = mpsc::channel(16);
        let (broad_tx, _broad_rx) = broadcast::channel(16);

        // Lobby
        loop {
            tokio::select! {
                Ok((socket, _addr)) = listener.accept() => {
                    let res = server.add_user();
                    match res {
                        Ok(id) => handle_user(socket, tx.clone(), broad_tx.subscribe(), id),
                        Err(_err) => todo!("Handle too many players"),
                    };
                },
                Some(data) = rx.recv() => {
                    let (id, response_tx, msg) = data;

                    match msg {
                        Message::GameStart => {
                            if id == server.host_id {
                                break;
                            }
                        },
                        Message::Rename(str) => { 
                            server.change_name(id, str);
                            let _ = response_tx.send(Message::Ok);
                        },
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

fn handle_user(mut user_conn: TcpStream, tx: mpsc::Sender<(Id, oneshot::Sender<Message>, Message)>, mut broad_rx: broadcast::Receiver<Message>, id: Id) {
    tokio::spawn(async move {
        loop {
            let mut buffer = [0; 64];

            tokio::select! {
                Ok(n) = user_conn.read(&mut buffer) => {
                    if n > 0 {
                        if let Ok(msg) = serde_json::from_slice(&buffer) {
                            let (response_tx, _response_rx) = oneshot::channel();
                            match tx.send((id, response_tx, msg)).await {
                                Ok(()) => {
                                    todo!("get response from server");
                                },
                                Err(_err) => todo!("Handle send errors"),
                            }
                        }
                    }
                },
                Ok(msg) = broad_rx.recv() => {
                    if msg == Message::GameStart {
                        let serialized_msg = serde_json::to_vec(&msg)
                            .expect("should be serializable");

                        if let Err(_err) = user_conn.write_all(&serialized_msg).await {
                            todo!("handle errors on write_all");
                        }
                    }
                },
            }
        }
    });
}

pub fn join_server() {
    todo!("Implement join_server")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_host() {
        let mut server = Server::new();

        let _ = server.add_user();
        let _ = server.add_user();
        server.change_host(1);

        assert_eq!(server.host_id, 1);
    }

    #[test]
    fn test_too_many_users() {
        let mut sv = Server::new();

        for _i in 0..7 {
            let _ = sv.add_user();
        }

        let ret = sv.add_user();

        assert_eq!(Err(ServerError::TooManyPlayers), ret);
    }
}
