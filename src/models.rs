use std::fmt;

use serde::{Deserialize, Serialize};

pub enum ConnectionType {
    Live,
    Demo,
}

#[derive(Debug)]
pub enum Command {
    login,
    getMarginLevel,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub command: String,
    pub arguments: LoginRequestArguments,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestArguments {
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub status: bool,
    #[serde(rename = "streamSessionId")]
    pub stream_session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub command: String,
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
