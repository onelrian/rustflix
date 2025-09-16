//! WebSocket handler for real-time communication

use rustflix_core::{Result, RustFlixError};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;
use tracing::{info, warn, debug};

/// WebSocket handler for real-time events
#[derive(Debug, Clone)]
pub struct WebSocketHandler {
    event_sender: broadcast::Sender<WebSocketEvent>,
}

/// WebSocket event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketEvent {
    MediaAdded { media_id: Uuid, title: String },
    MediaUpdated { media_id: Uuid, title: String },
    MediaRemoved { media_id: Uuid },
    StreamStarted { stream_id: Uuid, user_id: Uuid },
    StreamStopped { stream_id: Uuid },
    TranscodeProgress { job_id: Uuid, progress: f32 },
    TranscodeCompleted { job_id: Uuid },
    UserConnected { user_id: Uuid },
    UserDisconnected { user_id: Uuid },
}

/// WebSocket message from client
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Subscribe { events: Vec<String> },
    Unsubscribe { events: Vec<String> },
    Ping,
}

/// WebSocket message to client
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Event(WebSocketEvent),
    Pong,
    Error { message: String },
}

impl WebSocketHandler {
    /// Create a new WebSocket handler
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);
        
        Self { event_sender }
    }

    /// Handle WebSocket upgrade
    pub async fn handle_upgrade(self, ws: WebSocketUpgrade) -> Response {
        let event_sender = self.event_sender.clone();
        ws.on_upgrade(move |socket| async move {
            let handler = WebSocketHandler { event_sender };
            handler.handle_socket(socket).await;
        })
    }

    /// Handle WebSocket connection
    async fn handle_socket(&self, socket: WebSocket) {
        info!("New WebSocket connection established");
        
        let (mut sender, mut receiver) = socket.split();
        let mut event_receiver = self.event_sender.subscribe();
        
        // Create a channel for communication between tasks
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
        
        // Handle incoming messages
        let tx_clone = tx.clone();
        let incoming_task = tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        debug!("Received WebSocket message: {}", text);
                        
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Ping) => {
                                let response = ServerMessage::Pong;
                                if let Ok(json) = serde_json::to_string(&response) {
                                    let _ = tx_clone.send(Message::Text(json));
                                }
                            }
                            Ok(ClientMessage::Subscribe { events }) => {
                                debug!("Client subscribed to events: {:?}", events);
                                // Handle subscription logic
                            }
                            Ok(ClientMessage::Unsubscribe { events }) => {
                                debug!("Client unsubscribed from events: {:?}", events);
                                // Handle unsubscription logic
                            }
                            Err(e) => {
                                warn!("Failed to parse WebSocket message: {}", e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed by client");
                        break;
                    }
                    Ok(_) => {
                        // Handle other message types (binary, ping, pong)
                    }
                    Err(e) => {
                        warn!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });

        // Handle outgoing messages (server events)
        let outgoing_task = tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                let message = ServerMessage::Event(event);
                if let Ok(json) = serde_json::to_string(&message) {
                    let _ = tx.send(Message::Text(json));
                }
            }
        });

        // Handle sending messages to WebSocket
        let sender_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });

        // Wait for any task to complete
        tokio::select! {
            _ = incoming_task => {
                debug!("Incoming WebSocket task completed");
            }
            _ = outgoing_task => {
                debug!("Outgoing WebSocket task completed");
            }
            _ = sender_task => {
                debug!("Sender WebSocket task completed");
            }
        }
        
        info!("WebSocket connection closed");
    }

    /// Broadcast event to all connected clients
    pub fn broadcast_event(&self, event: WebSocketEvent) -> Result<()> {
        self.event_sender
            .send(event)
            .map_err(|e| RustFlixError::internal(format!("Failed to broadcast event: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_handler_creation() {
        let handler = WebSocketHandler::new();
        // Basic creation test
        assert!(true);
    }

    #[test]
    fn test_event_serialization() {
        let event = WebSocketEvent::MediaAdded {
            media_id: Uuid::new_v4(),
            title: "Test Movie".to_string(),
        };
        
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_client_message_deserialization() {
        let json = r#"{"type": "Subscribe", "events": ["media", "stream"]}"#;
        let message: Result<ClientMessage, _> = serde_json::from_str(json);
        assert!(message.is_ok());
    }
}
