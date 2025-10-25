use std::error::Error;
use s9_websocket::S9WebSocketError;

#[derive(Debug)]
pub enum BinanceWebSocketError {
    WebSocket(S9WebSocketError),
    Serialization(serde_json::Error),
}

impl std::fmt::Display for BinanceWebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinanceWebSocketError::WebSocket(e) => write!(f, "WebSocket error: {}", e),
            BinanceWebSocketError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl Error for BinanceWebSocketError {}

impl From<S9WebSocketError> for BinanceWebSocketError {
    fn from(err: S9WebSocketError) -> Self {
        BinanceWebSocketError::WebSocket(err)
    }
}

impl From<serde_json::Error> for BinanceWebSocketError {
    fn from(err: serde_json::Error) -> Self {
        BinanceWebSocketError::Serialization(err)
    }
}

pub type BinanceResult<T> = Result<T, BinanceWebSocketError>;