#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    fmt,
    io::{Error, Read, Write},
    net::TcpStream,
};

use native_tls::{TlsConnector, TlsStream};
use serde::{Deserialize, Serialize};

const HOST: &str = "xapi.xtb.com";
const PORT_LIVE: &str = "5112";
const PORT_DEMO: &str = "5124";

#[derive(Debug)]
enum Command {
    login,
    getMarginLevel,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    command: String,
    arguments: LoginRequestArguments,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequestArguments {
    #[serde(rename = "userId")]
    user_id: u64,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    status: bool,
    #[serde(rename = "streamSessionId")]
    stream_session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub status: bool,
    #[serde(rename = "returnData")]
    pub return_data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub balance: f64,
    pub credit: f64,
    pub currency: String,
    pub equity: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
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

        let login_request = LoginRequest {
            command: Command::login.to_string(),
            arguments: LoginRequestArguments {
                user_id: user_id,
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

        let login_response: LoginResponse = serde_json::from_str(&trimmed_data_string)?;

        self.stream_session_id = Some(login_response.stream_session_id);

        Ok(())
    }

    pub fn balance(&mut self) -> Result<Response<Balance>, Error> {
        let balance_request = Request {
            command: Command::getMarginLevel.to_string(),
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

        let balance_response: Response<Balance> = serde_json::from_str(&trimmed_data_string)?;

        Ok(balance_response)
    }
}
