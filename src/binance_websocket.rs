use std::collections::HashMap;
use std::error::Error;
use s9_binance_codec::websocket::SubscriptionRequest;
use s9_websocket::websocket::{S9WebSocketClient, S9WebSocketClientHandler};

pub struct BinanceWebSocketConnection {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl std::fmt::Display for BinanceWebSocketConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}{}",
               self.protocol,
               self.host,
               self.port,
               self.path
        )
    }
}

pub struct BinanceWebSocketConfig {
    pub connection: BinanceWebSocketConnection,
}

pub struct BinanceWebSocket {
    pub config: BinanceWebSocketConfig,
    pub s9_websocket_client: S9WebSocketClient,
    sequence: u64,
}

impl BinanceWebSocket {

    pub fn connect(config: BinanceWebSocketConfig) -> Result<BinanceWebSocket, Box<dyn Error>> {
        let url = config.connection.to_string();
        let s9_websocket_client = S9WebSocketClient::connect_with_headers(url.as_str(), &config.connection.headers)?;

        Ok(BinanceWebSocket {
            config,
            s9_websocket_client,
            sequence: 0,
        })
    }

    pub fn run<HANDLER>(&mut self, handler: &mut HANDLER)
    where
        HANDLER: S9WebSocketClientHandler,
    {
        self.s9_websocket_client.run(handler);
    }

    pub fn subscribe_to_streams(&mut self, streams: Vec<String>) -> Result<(), Box<dyn Error>> {
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