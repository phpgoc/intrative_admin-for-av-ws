use crate::admin::db::dummy::{ChannelInfo, Dummy};
use crate::admin::db::traits::AsyncDbTrait;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;

pub async fn tcp_server() {
    let addr = format!(
        "127.0.0.1:{}",
        env::var("TCP_PORT").unwrap_or_else(|_| "9527".to_string())
    );
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Tcp Listening on : {}", addr);
    let db_impl = Dummy::new();
    loop {
        let (mut stream, _) = listener.accept().unwrap();
        println!("New connection: {}", stream.peer_addr().unwrap());
        let db = db_impl.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    return;
                }
                let request: TcpRequest = bincode::deserialize(&buf[..n]).unwrap();
                let response = match request {
                    TcpRequest::Ping => TcpResponse::Pong,
                    TcpRequest::ListChannels => db.list_channels().await,
                    TcpRequest::SetAuth(channel, user_name, opertion) => {
                        db.set_channel_user_auth(&channel, &user_name, opertion)
                            .await
                    }
                    TcpRequest::ListChannelUsers(channel) => db.list_channel_users(&channel).await,
                    // TcpRequest::ListChannelPublishedUsers(_) => {}
                    TcpRequest::SetRoomPublic(channel_id, is_public) => {
                        db.set_room_public(&channel_id, is_public).await
                    }
                    // TcpRequest::SetRoomMaxPublishers(_, _) => {}
                    TcpRequest::QueryRoom(chanel_id) => db.query(chanel_id.as_str()).await,
                    TcpRequest::KickUser(channel, user_name) => {
                        db.kick_out(&channel, &user_name).await
                    }
                    _ => TcpResponse::Unknown,
                };
                let response = bincode::serialize(&response).unwrap();
                stream.write(&response).unwrap();
            }
        });
        print!("Connection closed\n");
    }
}

pub(crate) static mut TCPSTEAM: *mut TcpStream = 0 as *mut TcpStream;

pub async fn connect_tcp() -> Result<(), Box<dyn Error>> {
    let addr = format!(
        "127.0.0.1:{}",
        env::var("TCP_PORT").unwrap_or_else(|_| "9527".to_string())
    );
    let stream = TcpStream::connect(addr).unwrap();
    unsafe {
        TCPSTEAM = Box::into_raw(Box::new(stream));
    }
    Ok(())
}
///该方法没有并发安全问题，因为只有一个线程会调用
pub(crate) fn send_tcp_request(request: TcpRequest) -> TcpResponse {
    let mut buf = vec![0; 1024];
    let req_vec = bincode::serialize(&request).unwrap();

    let mut stream = unsafe { Box::from_raw(TCPSTEAM) };

    while let Err(e) = stream.write(&req_vec) {
        println!("tcp write error: {}", e);
        exit(1);
    }
    let n = stream.read(&mut buf).unwrap();
    unsafe {
        TCPSTEAM = Box::into_raw(Box::new(*stream));
    }
    if let Ok(res) = bincode::deserialize::<TcpResponse>(&buf[..n]) {
        return res;
    };
    TcpResponse::Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum TcpRequest {
    SetAuth(String, String, bool),
    ListChannels,
    ListChannelUsers(String),
    ListChannelPublishedUsers(String),
    SetRoomPublic(String, bool),
    SetRoomMaxPublishers(String, u32),
    QueryRoom(String),
    KickUser(String, String),
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum TcpResponse {
    Ok,
    One(String),
    List(Vec<String>),
    Query(ChannelInfo),
    DbError,
    UnknownSelected,
    Unknown,
    Pong,
}

impl TcpResponse {
    pub(crate) fn unwrap(self) -> Self {
        match self {
            TcpResponse::DbError | TcpResponse::Unknown => {
                println!("TcpResponse unwrap error");
                std::io::stdin().read(&mut [0]).unwrap();
                exit(1);
            }
            _ => self,
        }
    }
}
