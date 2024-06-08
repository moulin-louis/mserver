use std::error::Error;
use std::io::Write;

use uuid::Uuid;

use crate::mpacket::Packet;
use crate::mstring::MString;
use crate::Client;

struct LoginProperty {
    name: MString,
    value: MString,
    is_signed: bool,
    signature: Option<String>,
}

struct LoginSuccess {
    uuid: Uuid,
    username: MString,
    nbr_props: i64,
    props: LoginProperty,
    strict_error_handling: bool,
}

pub fn set_protocol() {}

pub fn cookie_request(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let id = packet.read_string().unwrap();
    println!("identifier = {id}");
    Ok(())
}

pub fn login_start(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    client.username = packet.read_string().unwrap();
    println!("username = {}", &client.username);
    client.uuid = packet.read_uuid().unwrap();
    Ok(())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn login_success(client: &mut Client, packet: &mut Packet) -> Result<(), Box<dyn Error>> {
    let loginSuccess = LoginSuccess {
        uuid: client.uuid,
        username: client.username.clone(),
        nbr_props: 0,
        props: LoginProperty {
            name: "toto".into(),
            value: "tata".into(),
            is_signed: false,
            signature: None,
        },
        strict_error_handling: true,
    };
    unsafe {
        client
            .connection
            .tcp_stream
            .write_all(any_as_u8_slice(&loginSuccess))
            .unwrap();
    }
    Ok(())
}
