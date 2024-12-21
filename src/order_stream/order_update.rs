use crate::order_stream::messages::UserDataUpdate;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

#[derive(Clone, Debug)]
pub struct UserDataStream {
    pub listen_key: String,
}

impl UserDataStream {
    pub async fn listen_user_data(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        loop {
            let url = format!("wss://fstream.binance.com/ws/{}", self.listen_key);
            let (ws_stream, _) = match connect_async(&url).await {
                Ok(stream) => {
                    info!("Listen to User Data Stream");
                    stream
                }
                Err(e) => {
                    info!("Failed to connect: {}, retrying...", e);
                    continue;
                }
            };
            let (mut write, mut read) = ws_stream.split();
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        info!("Received message: {}", &text);
                        self.handle_user_data_update(&text).await;
                    }
                    Ok(Message::Ping(payload)) => {
                        info!("UserDataStream Ping {:?}", &payload);
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            info!("Failed to send Pong response: {}", e);
                        }
                    }
                    Ok(Message::Binary(binary)) => {
                        info!("{:?}", &binary);
                    }
                    Ok(non_text_message) => {
                        info!("Received Non-Text Message: {:?}", non_text_message);
                    }
                    Err(e) => {
                        info!("Error Message: {}", e);
                        break;
                    }
                }
            }
            info!("Use Data Connection lost, retrying immediately...");
            continue;
        }
    }
    async fn handle_user_data_update(&self, text: &str) {
        match serde_json::from_str::<UserDataUpdate>(text) {
            Ok(update) => {
                self.process_update(update).await;
            }
            Err(e) => {
                info!("Failed to deserialize message: {}, text: {}", e, text);
            }
        }
    }

    async fn process_update(&self, update: UserDataUpdate) {
        match update {
            UserDataUpdate::OrderTradeUpdate(order_update) => {
                info!("Received OrderTradeUpdate: {:?}", order_update);
            }
            _ => {
                info!("Received update: {:?}", update);
            }
        }
    }
}
