use s9_binance_codec::websocket::SubscriptionRequest;
use std::collections::HashMap;
use std::error::Error;
use crossbeam_channel::{Receiver};
// re-export for lib users
pub use s9_websocket::websocket::ControlMessage;
pub use s9_websocket::websocket::S9WebSocketClientHandler;
pub use s9_websocket::websocket::{S9BlockingWebSocketClient, S9NonBlockingWebSocketClient, WebSocketEvent};
pub use s9_websocket::websocket::NonBlockingOptions;

pub struct BinanceWebSocketConfig {
    pub connection: BinanceWebSocketConnection,
}

pub struct BinanceWebSocketConnection {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl std::fmt::Display for BinanceWebSocketConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}{}?timeUnit=MICROSECOND",
               self.protocol,
               self.host,
               self.port,
               self.path,
        )
    }
}

pub struct BinanceNonBlockingWebSocket {
    pub s9_websocket_client: S9NonBlockingWebSocketClient,
    sequence: u64,
}

impl BinanceNonBlockingWebSocket {
    pub fn connect(config: BinanceWebSocketConfig) -> Result<Self, Box<dyn Error>> {
        let url = config.connection.to_string();
        let s9_websocket_client = S9NonBlockingWebSocketClient::connect_with_headers(url.as_str(), &config.connection.headers)?;

        Ok(BinanceNonBlockingWebSocket {
            s9_websocket_client,
            sequence:0,
        })
    }

    pub fn run_non_blocking(&mut self, non_blocking_options: NonBlockingOptions) -> Result<(), Box<dyn Error>> {
        self.s9_websocket_client.run_non_blocking(non_blocking_options)
    }


    pub fn subscribe_to_streams_non_blocking(&mut self, streams: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut request = SubscriptionRequest::new(self.sequence);
        for stream in streams {
            request.add_stream(stream.as_str());
        }
        let json = request.to_json()?;
        self.s9_websocket_client.send_text_message(json.as_str())?;
        self.sequence += 1;
        Ok(())
    }
}

pub struct BinanceBlockingWebSocket {
    s9_websocket_client: S9BlockingWebSocketClient,
    sequence: u64,
}

impl BinanceBlockingWebSocket {
    pub fn connect(config: BinanceWebSocketConfig) -> Result<Self, Box<dyn Error>> {
        let url = config.connection.to_string();
        let s9_websocket_client = S9BlockingWebSocketClient::connect_with_headers(url.as_str(), &config.connection.headers)?;

        Ok(BinanceBlockingWebSocket {
            s9_websocket_client,
            sequence: 0,
        })
    }

    pub fn run_blocking<HANDLER>(&mut self, handler: &mut HANDLER, control_rx: Receiver<ControlMessage>)
    where
        HANDLER: S9WebSocketClientHandler,
    {
        self.s9_websocket_client.run_blocking(handler, control_rx)
    }


    pub fn subscribe_to_streams_blocking(&mut self, streams: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut request = SubscriptionRequest::new(self.sequence);
        for stream in streams {
            request.add_stream(stream.as_str());
        }
        let json = request.to_json()?;
        self.s9_websocket_client.send_text_message(json.as_str())?;
        self.sequence += 1;
        Ok(())
    }
}