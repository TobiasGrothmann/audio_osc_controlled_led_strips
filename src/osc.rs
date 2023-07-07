extern crate rosc;

use anyhow::Result;
use rosc::OscPacket;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct OscFaderValues {
    pub values: Vec<Vec<f32>>,
}
impl OscFaderValues {
    pub fn new() -> Self {
        let mut values = vec![vec![0.0; 30]; 10];
        values[0][0] = 0.25;
        values[0][1] = 0.5;
        values[1][1] = 1.0;
        values[2][0] = 0.4;
        Self { values: values }
    }
}

pub struct OscFaderValue {
    page: i32,
    index: i32,
    value: f32,
}

pub fn osc_start_listen(osc_fader_values_mutex: Arc<Mutex<OscFaderValues>>) {
    let addr = SocketAddrV4::from_str("0.0.0.0:8000").unwrap();
    let sock = UdpSocket::bind(addr).unwrap();

    let mut buf = [0u8; rosc::decoder::MTU];

    thread::spawn(move || loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                let package_result = handle_packet(packet, osc_fader_values_mutex.clone());
                // if !package_result.is_ok() {
                //     println!("failed to parse packet");
                // }
            }
            Err(e) => {}
        }
    });
}

use std::{error::Error, fmt};
use std::{thread, vec};

#[derive(Debug)]
struct OscErr;
impl Error for OscErr {}
impl fmt::Display for OscErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Osc error occured")
    }
}

fn handle_packet(
    packet: OscPacket,
    osc_fader_values_mutex: Arc<Mutex<OscFaderValues>>,
) -> Result<()> {
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
            let page_index = msg_arr.get(1).ok_or(OscErr {})?.parse::<i32>()? - 1;

            // get index
            let fader_index = msg_arr
                .last()
                .ok_or(OscErr {})?
                .replace("fader", "")
                .parse::<i32>()?
                - 1;

            // write to fader values
            let osc_fader_values_result = osc_fader_values_mutex.lock();
            if osc_fader_values_result.is_err() {
                Err(OscErr {})?;
            };
            let mut osc_fader_values = osc_fader_values_result.unwrap();
            osc_fader_values.values[page_index as usize][fader_index as usize] = value;

            if page_index < 0 || page_index >= 10 {
                Err(OscErr {})?;
            }
            if fader_index < 0 || fader_index >= 30 {
                Err(OscErr {})?;
            }

            // println!("{} - {}: {}", page_index, fader_index, value);

            Ok(())
        }
        _ => Ok(()),
    }
}
