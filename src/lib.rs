#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod models;

use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

use models::ConnectionType;
use native_tls::{TlsConnector, TlsStream};

const HOST: &str = "xapi.xtb.com";
const PORT_LIVE: &str = "5112";
const PORT_DEMO: &str = "5124";

pub struct Client {
    main_connection: TlsStream<TcpStream>,
    stream_session_id: Option<String>,
    // stream_connection: Option<TlsStream<TcpStream>>, // Will be used for actual streaming connection
}

impl Client {
    pub fn new(connection_type: ConnectionType) -> Client {
        let connector = TlsConnector::new().unwrap();
        let main_connection = Client::build_main_connection(&connection_type, &connector);

        Client {
            main_connection: main_connection,
            stream_session_id: None,
            // stream_connection: None,
        }
    }

    pub fn login(&mut self, user_id: u64, password: String) -> Result<(), Error> {
        let login_request = models::LoginRequest {
            command: models::Command::login.to_string(),
            arguments: models::LoginRequestArguments {
                user_id: user_id,
                password: password,
            },
        };

        let request = serde_json::to_string(&login_request)?;

        self.main_connection
            .write_all(request.as_bytes())
            .expect("write failed");

        let mut buffer = [0; 4096];
        self.main_connection.read(&mut buffer)?;

        let data_string = String::from_utf8_lossy(&buffer);

        let trimmed_data_string = &data_string.split("\n\n").next().unwrap();

        let login_response: models::LoginResponse = serde_json::from_str(&trimmed_data_string)?;

        self.stream_session_id = Some(login_response.stream_session_id);

        Ok(())
    }

    pub fn balance(&mut self) -> Result<models::Response<models::Balance>, Error> {
        let balance_request = models::Request {
            command: models::Command::getMarginLevel.to_string(),
        };

        let request = serde_json::to_string(&balance_request)?;

        self.main_connection
            .write_all(request.as_bytes())
            .expect("write failed");

        let mut buffer = [0; 4096];
        self.main_connection.read(&mut buffer)?;

        let data_string = String::from_utf8_lossy(&buffer);

        let trimmed_data_string = data_string.split("\n\n").next().unwrap();

        let balance_response: models::Response<models::Balance> =
            serde_json::from_str(&trimmed_data_string)?;

        Ok(balance_response)
    }

    fn build_main_connection(
        connection_type: &ConnectionType,
        connector: &TlsConnector,
    ) -> TlsStream<TcpStream> {
        let addr = Client::build_addr(connection_type);
        let stream = TcpStream::connect(addr).expect("Could not connect to the XTB API");

        connector.connect(HOST, stream).unwrap()
    }

    fn build_addr(connection_type: &ConnectionType) -> String {
        match connection_type {
            ConnectionType::Live => format!("{}:{}", HOST, PORT_LIVE),
            ConnectionType::Demo => format!("{}:{}", HOST, PORT_DEMO),
        }
    }
}
