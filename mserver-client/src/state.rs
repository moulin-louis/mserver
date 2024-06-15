use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

use mserver_mpacket::mpacket::Packet;

use crate::{configuration_handlers, handshaking_handler, login_handler, play_handler, status_handler};
use crate::client::Client;

#[derive(Debug, Eq, PartialEq)]
pub enum StateClient {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
    Target,
    Transfer,
}

pub type HandlerFunction = fn(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>>;

lazy_static! {
    pub static ref HANDSHAKE_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x0, handshaking_handler::set_protocol);
        m
    };
    pub static ref STATUS_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, status_handler::status_request);
        m.insert(0x01, status_handler::ping_request);
        m
    };
    pub static ref LOGIN_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, login_handler::login_start);
        m.insert(0x03, login_handler::login_ack);
        m.insert(0x05, login_handler::cookie_request);
        m
    };
    pub static ref CONFIGURATION_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, configuration_handlers::client_info);
        m.insert(0x02, configuration_handlers::serv_plugin_message);
        m.insert(0x03, configuration_handlers::finish_config_ack);
        m
    };
    pub static ref PLAY_HANDLERS: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
    pub static ref TARGET_HANDLERS: HashMap<i64, HandlerFunction> = {
        let mut m: HashMap<i64, HandlerFunction> = HashMap::new();
        m.insert(0x00, play_handler::confirm_teleportation);
        m
    };
    pub static ref TRANSFER_HANDLERS: HashMap<i64, HandlerFunction> = {
        let m: HashMap<i64, HandlerFunction> = HashMap::new();
        m
    };
}

