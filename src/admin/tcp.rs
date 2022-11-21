use crate::admin::structs_types::AdminError;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub async fn tcp_server() {
    let addr = format!(
        "127.0.0.1:{}",
        env::var("TCP_PORT").unwrap_or_else(|_| "9527".to_string())
    );
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Tcp Listening on : {}", addr);

    loop {
        let (mut socket, _) = listener.accept().unwrap();
        println!(" Accepted connection from: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket.read(&mut buf).map_or(0, |n| n);

                if n == 0 {
                    println!("Tcp Connection closed");
                    return;
                }
                println!("Received data from client: {:?}", &buf[0..n]);
                let Ok(request) = bincode::deserialize::<TcpRequest>(&buf[..n]) else {
                    let res_vec = bincode::serialize(&TcpResponse::Unknown).unwrap();
                    socket.write_all(&res_vec).unwrap();
                    continue;
                };
                let res = match request {
                    TcpRequest::SetAuth(_, _) => TcpResponse::Unknown,
                    TcpRequest::ListChannels => TcpResponse::Unknown,
                    TcpRequest::ListChannelUsers(_) => TcpResponse::Unknown,
                    TcpRequest::ListChannelPublishedUsers(_) => TcpResponse::Unknown,
                    TcpRequest::SetRoomPublic(_, _) => TcpResponse::Unknown,
                    TcpRequest::SetRoomMaxPublishers(_, _) => TcpResponse::Unknown,
                    TcpRequest::QueryRoom(_) => TcpResponse::Unknown,
                    TcpRequest::KickUser(_, _) => TcpResponse::Unknown,
                    TcpRequest::Ping => TcpResponse::Pong,
                };
                let res_vec = bincode::serialize(&res).unwrap();
                socket
                    .write_all(&res_vec)
                    .expect("failed to write data to socket");
            }
        });
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
pub(crate) async fn send_tcp_request(request: TcpRequest) -> TcpResponse {
    let mut buf = vec![0; 1024];
    let req_vec = bincode::serialize(&request).unwrap();

    let mut stream = unsafe { Box::from_raw(TCPSTEAM) };

    stream.write(&req_vec).unwrap();
    let n = stream.read(&mut buf).unwrap();
    println!("n:{}", n);
    println!(" Admin Tcp Received: {:?}", &buf[0..n]);

    let Ok(res) = bincode::deserialize::<TcpResponse>(&buf[..n]) else{
        println!(" Admin Tcp Connection OK");
        return TcpResponse::Unknown;
    };
    println!(" Admin Tcp Received: {:?}", res);
    TcpResponse::Unknown

}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum TcpRequest {
    SetAuth(String, String),
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
    ChannelList(Vec<String>),
    Unknown,
    Pong,
}
