extern crate rosc;

use anyhow::{Context, Result};
use rosc::OscPacket;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct OscFaderValue {
    page: i32,
    index: i32,
    value: f32,
}

pub fn osc_start_listen(tx: Sender<OscFaderValue>) {
    let addr = SocketAddrV4::from_str("0.0.0.0:8000").unwrap();
    let sock = UdpSocket::bind(addr).unwrap();

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                let package_result = handle_packet(packet, &tx);
                if !package_result.is_ok() {
                    println!("failed to parse packet");
                }
            }
            Err(e) => {}
        }
    }
}

use std::{error::Error, fmt};

#[derive(Debug)]
struct OscErr;
impl Error for OscErr {}
impl fmt::Display for OscErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Osc error occured")
    }
}

fn handle_packet(packet: OscPacket, tx: &Sender<OscFaderValue>) -> Result<()> {
    match packet {
        OscPacket::Message(msg) => {
            // get value
            let value = msg
                .args
                .get(0)
                .ok_or(OscErr {})?
                .clone()
                .float()
                .ok_or(OscErr {})?;

            // get page and index
            let msg_arr: Vec<&str> = msg.addr.split("/").collect();

            // get page
            let page_index = msg_arr.get(1).ok_or(OscErr {})?.parse::<i32>()?;

            // get index
            let fader_index = msg_arr
                .last()
                .ok_or(OscErr {})?
                .replace("fader", "")
                .parse::<i32>()?;

            tx.send(OscFaderValue {
                index: fader_index,
                value: value,
                page: page_index,
            })?;
            Ok(())
        }
        _ => Ok(()),
    }
}
