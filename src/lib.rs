use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

use native_tls::{TlsConnector, TlsStream};
use serde::{Deserialize, Serialize};

const HOST: &str = "xapi.xtb.com";
const PORT_LIVE: &str = "5112";
const PORT_DEMO: &str = "5124";

#[derive(Serialize, Deserialize, Debug)]
struct XtbLoginRequest {
    command: String,
    arguments: XtbRequestArguments,
}

#[derive(Serialize, Deserialize, Debug)]
struct XtbRequestArguments {
    userId: u64,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct XtbLoginResponse {
    status: bool,
    streamSessionId: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct XtbBalanceRequest {
    command: String,
}
pub struct Client {
    connector: TlsConnector,
    stream: Option<TlsStream<TcpStream>>,
    stream_session_id: Option<String>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            connector: TlsConnector::new().unwrap(),
            stream: None,
            stream_session_id: None,
        }
    }

    pub fn login(&mut self, user_id: u64, password: String, demo: bool) -> Result<(), Error> {
        let mut addr = format!("{}:{}", HOST, PORT_LIVE);

        if demo {
            addr = format!("{}:{}", HOST, PORT_DEMO);
        }

        let stream = TcpStream::connect(addr).expect("connection failed");
        let stream = self.connector.connect(HOST, stream).unwrap();

        self.stream = Some(stream);

        let login_request = XtbLoginRequest {
            command: String::from("login"),
            arguments: XtbRequestArguments {
                userId: user_id,
                password: password,
            },
        };

        let request = serde_json::to_string(&login_request)?;

        self.stream
            .as_mut()
            .unwrap()
            .write_all(request.as_bytes())
            .expect("write failed");

        let mut buffer = [0; 4096];
        self.stream.as_mut().unwrap().read(&mut buffer)?;

        let data_string = String::from_utf8_lossy(&buffer);

        let trimmed_data_string = &data_string.split("\n\n").next().unwrap();

        let login_response: XtbLoginResponse = serde_json::from_str(&trimmed_data_string)?;

        self.stream_session_id = Some(login_response.streamSessionId);

        Ok(())
    }

    pub fn balance(&mut self) -> Result<String, Error> {
        let balance_request = XtbBalanceRequest {
            command: String::from("getMarginLevel"),
        };

        let request = serde_json::to_string(&balance_request)?;

        self.stream
            .as_mut()
            .unwrap()
            .write_all(request.as_bytes())
            .expect("write failed");

        let mut buffer = [0; 4096];
        self.stream.as_mut().unwrap().read(&mut buffer)?;

        let data_string = String::from_utf8_lossy(&buffer);

        let trimmed_data_string = data_string.split("\n\n").next().unwrap();

        Ok(trimmed_data_string.to_string())
    }
}
