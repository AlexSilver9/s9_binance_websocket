use s9_binance_codec::websocket::SubscriptionRequest;
use std::collections::HashMap;
use std::error::Error;
use crossbeam_channel::{Receiver, Sender};
// re-export for lib users
pub use s9_websocket::websocket::ControlMessage;
pub use s9_websocket::websocket::S9WebSocketClient;
pub use s9_websocket::websocket::S9WebSocketClientHandler;
use s9_websocket::websocket::WebSocketEvent;


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

struct BinanceWebSocketCore {
    s9_websocket_client: S9WebSocketClient,
    sequence: u64,
}

impl BinanceWebSocketCore {
    fn connect(config: BinanceWebSocketConfig) -> Result<Self, Box<dyn Error>> {
        let url = config.connection.to_string();
        let s9_websocket_client = S9WebSocketClient::connect_with_headers(url.as_str(), &config.connection.headers)?;

        Ok(BinanceWebSocketCore {
            s9_websocket_client,
            sequence: 0,
        })
    }
}

pub struct BinanceNonBlockingWebSocket {
    core: BinanceWebSocketCore,
    control_tx: Option<Sender<ControlMessage>>,
}


impl BinanceNonBlockingWebSocket {
    pub fn connect(config: BinanceWebSocketConfig) -> Result<Self, Box<dyn Error>> {
        let core = BinanceWebSocketCore::connect(config)?;
        Ok(BinanceNonBlockingWebSocket {
            core,
            control_tx: None,
        })
    }

    pub fn run_non_blocking(mut self) -> (Sender<ControlMessage>, Receiver<WebSocketEvent>) {
        let (tx, rx) = self.core.s9_websocket_client.run_non_blocking();
        self.control_tx = Some(tx.clone());
        (tx, rx)
    }


    pub fn subscribe_to_streams_non_blocking(&mut self, streams: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut request = SubscriptionRequest::new(self.core.sequence);
        for stream in streams {
            request.add_stream(stream.as_str());
        }
        let json = request.to_json()?;
        self.control_tx.as_ref().unwrap().send(ControlMessage::SendText(json))?;
        self.core.sequence += 1;
        Ok(())
    }
}

pub struct BinanceBlockingWebSocket {
    core: BinanceWebSocketCore,
}

impl BinanceBlockingWebSocket {
    pub fn connect(config: BinanceWebSocketConfig) -> Result<Self, Box<dyn Error>> {
        let core = BinanceWebSocketCore::connect(config)?;
        Ok(BinanceBlockingWebSocket { core })
    }

    pub fn run_blocking<HANDLER>(&mut self, handler: &mut HANDLER, control_rx: Receiver<ControlMessage>)
    where
        HANDLER: S9WebSocketClientHandler,
    {
        self.core.s9_websocket_client.run_blocking(handler, control_rx)
    }


    pub fn subscribe_to_streams_blocking(&mut self, streams: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut request = SubscriptionRequest::new(self.core.sequence);
        for stream in streams {
            request.add_stream(stream.as_str());
        }
        let json = request.to_json()?;
        self.core.s9_websocket_client.send_text_message(json.as_str())?;
        self.core.sequence += 1;
        Ok(())
    }
}