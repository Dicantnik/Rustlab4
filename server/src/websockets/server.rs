use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use base64::Engine;
use std::fs::File;
use std::io::{Write};
use std::fs;
use std::path::Path;
use crate::dtos::messages::{NewMessageRequest};

type WebSocketConnections = Arc<Mutex<Vec<mpsc::UnboundedSender<Message>>>>;

pub async fn start_websocket_server(db_pool: PgPool) {
    let address = "127.0.0.1:9000";
    let listener = TcpListener::bind(address).await.expect("Failed to bind WebSocket server");
    let connections = Arc::new(Mutex::new(Vec::new()));

    println!("WebSocket server running on ws://{}", address);

    while let Ok((stream, _)) = listener.accept().await {
        let connections = connections.clone();
        let pool = db_pool.clone();

        tokio::spawn(async move {
            if let Ok(websocket_stream) = accept_async(stream).await {
                handle_client_connection(websocket_stream, connections, pool).await;
            }
        });
    }
}

async fn handle_client_connection(
    websocket_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    connections: WebSocketConnections,
    db_pool: PgPool,
) {
    let (mut writer, mut reader) = websocket_stream.split();

    let (message_sender, mut message_receiver) = mpsc::unbounded_channel();
    connections.lock().unwrap().push(message_sender);

    tokio::spawn(async move {
        while let Some(Ok(message)) = reader.next().await {
            if message.is_text() {
                let text_message = message.to_text().unwrap();
                println!("Received message: {}", text_message);

                if let Ok(request) = serde_json::from_str::<NewMessageRequest>(text_message) {
                    println!("Parsed message: {:?}", request);

                    if request.message_type == "file" {
                        if let Some(ref file_data) = request.file_data {
                            if let Some(ref file_path) = request.file_path {
                                if let Err(error) = save_file(file_data, file_path).await {
                                    eprintln!("Failed to save file: {}", error);
                                }
                            } else {
                                eprintln!("File path not provided");
                            }
                        }
                    }

                    if let Err(error) = save_message_to_db(&db_pool, &request).await {
                        eprintln!("Failed to save message: {}", error);
                    }

                    let response = serde_json::json!(request);
                    let response_text = response.to_string();

                    for sender in connections.lock().unwrap().iter() {
                        let _ = sender.send(Message::Text(response_text.clone()));
                    }
                } else {
                    eprintln!("Failed to parse message");
                }
            }
        }
    });

    while let Some(message) = message_receiver.recv().await {
        let _ = writer.send(message).await;
    }
}

async fn save_message_to_db(
    pool: &PgPool,
    request: &NewMessageRequest,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO messages (chat_id, user_id, content, created_at, file_path, message_type)
        VALUES ($1, $2, $3, NOW(), $4, $5)
        "#,
        request.chat_id,
        request.user_id,
        request.content,
        request.file_path.as_deref(),
        request.message_type
    )
        .execute(pool)
        .await;

    match result {
        Ok(_) => {
            println!("Message saved successfully");
            Ok(())
        }
        Err(error) => {
            eprintln!("Failed to save message: {}", error);
            Err(error)
        }
    }
}

async fn save_file(file_data: &str, file_path: &str) -> Result<(), std::io::Error> {
    let file_path = format!("uploads/{}", file_path);

    if let Some(parent_dir) = Path::new(&file_path).parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    let file_content = file_data.split(',').nth(1).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file data"))?;

    let decoded_data = base64::prelude::BASE64_STANDARD.decode(file_content)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to decode base64"))?;

    let mut file = File::create(&file_path)?;
    file.write_all(&decoded_data)?;

    println!("File saved: {}", file_path);

    Ok(())
}
