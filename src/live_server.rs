use std::io::{Read, Write};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use tokio::time::Duration;
use crate::tcp_message::{PublicField, TcpMessage};
use crate::tui_app;

async fn handle_client(
    mut stream: TcpStream,
    id: u16,
    mut server_to_client_rx: broadcast::Receiver<TcpMessage>,
    client_to_server_tx: Sender<TcpMessage>,
) {
    loop {
        let mut buffer = [0; 1024];
        tokio::select! {
            res = stream.read(&mut buffer) => {
                match res {
                    Ok(0) => {
                        println!("Client closed connection.");
                        break;
                    },
                    Ok(bytes_read) => {
                        let tcp_message = TcpMessage::decode(&buffer[..bytes_read]);

                        if tcp_message.is_some() {
                            println!("Message received: {:?}", tcp_message.unwrap());
                        } else {
                            println!("Message decoding failed.");
                        }

                        stream.write_all(b"Hello from the server").await.unwrap();
                        let _send_status = client_to_server_tx.send(TcpMessage::PublicMessage(
                            PublicField { message: "This is a public message".to_string() })).await;
                    },
                    Err(e) => { println!("Error: {}", e) },
                    _ => eprintln!("Error reading from client"),
                }
            }

            Ok(msg) = server_to_client_rx.recv() => {
                match msg {
                    TcpMessage::PrivateMessage(msg) => {
                        stream.write_all(msg.message.as_bytes()).await.unwrap();
                    },
                    TcpMessage::PublicMessage(msg) => {
                        stream.write_all(&msg.message.as_bytes()).await.unwrap();
                    }
                    _ => {
                        stream.write_all(b"Unknown message").await.unwrap();
                    }
                }
            }

        }
    }
}

pub async fn start_tcp_server() -> std::io::Result<()> {
    let mut current_id = 0;

    let (
        server_to_client_tx,
        mut server_to_client_rx
    ) = broadcast::channel::<TcpMessage>(128);

    let (
        client_to_server_tx,
        mut client_to_server_rx
    ) = channel::<TcpMessage>(128);



    let server_to_client_tx_clone = server_to_client_tx.clone();
    let client_to_server_tx_clone = client_to_server_tx.clone();

    tokio::spawn(async move {
        while let Some(msg) = client_to_server_rx.recv().await {
            let _ = server_to_client_tx.send(msg);
        }
    });
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

        println!("Listening on: {}", listener.local_addr().unwrap());

        loop {
            let (stream, addr) = listener.accept().await.unwrap();

            println!("Accepted connection from: {}", addr);

            let next_id = current_id;
            current_id += 1;

            let server_to_client_rx = server_to_client_tx_clone.subscribe();
            let client_to_server_tx = client_to_server_tx_clone.clone();

            tokio::spawn(handle_client(stream, next_id, server_to_client_rx, client_to_server_tx));
        }
    });
    tokio::spawn(async move {
        let _ = ratatui::run(|terminal| tui_app::App::default().run(terminal));
    });

    Ok(())
}