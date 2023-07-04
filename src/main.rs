mod bsky_client;
mod config;

use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_cbor_2::Deserializer;
use tungstenite::connect;

use std::collections::HashMap;
use std::convert::TryFrom;

use bsky_client::client::{Client, DEFAULT_BSKY_STREAM_SERVICE};
use config::{AppConfig, AppConfigVariableName};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageHeader {
    pub op: u8, // Should be 1 or -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayloadCommit {
    seq: u128,
    rebase: bool,
    too_big: bool,
    repo: String,
    // commit: String,
    // prev: Option<String>,
    time: String,
    ops: Vec<RepoOp>,
    #[serde(with = "serde_bytes")]
    blocks: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RepoOp {
    action: String, // Can do better with an enum here
    path: String,
}

fn main() {
    AppConfig::load().expect("Cannot initialize app config");

    let mut client = Client::new(
        AppConfig::get(AppConfigVariableName::BskyIdentifier),
        AppConfig::get(AppConfigVariableName::BskyPassword),
    );
    let create_session_result = client.create_session();

    let connection_result = connect(format!(
        "{}/xrpc/com.atproto.sync.subscribeRepos",
        DEFAULT_BSKY_STREAM_SERVICE
    ));

    match connection_result {
        Err(tungstenite::Error::Http(e)) => {
            if let Some(raw_data) = e.body() {
                if let Ok(text) = std::str::from_utf8(raw_data) {
                    println!("{}", text)
                }
            }
        }
        Ok((mut stream, response)) => {
            println!("CONNECTED {}", response.status());

            loop {
                let message = stream.read_message().unwrap();
                let data = message.into_data();

                let mut deserializer = Deserializer::from_slice(&data);
                let header: MessageHeader =
                    serde::de::Deserialize::deserialize(&mut deserializer).unwrap();

                if let Some(atproto_subtype) = header.t {
                    if atproto_subtype == "#commit" {
                        let payload: MessagePayloadCommit =
                            serde::de::Deserialize::deserialize(&mut deserializer).unwrap();

                        match &payload {
                            MessagePayloadCommit { ops: v, .. }
                                if !v.is_empty()
                                    && v[0].path.contains("app.bsky.feed.post")
                                    && v[0].action == "create" =>
                            {
                                println!("ops {:?}: repo {:?}", payload.ops, payload.repo);
                                let file = std::io::Cursor::new(payload.blocks.clone());
                                let blocks_slice = payload.blocks.as_slice();
                                decode_cad(blocks_slice);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        data => {
            println!("{:?}", data)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CarHeader {
    version: usize,
    roots: Vec<serde_bytes::ByteBuf>,
}

fn decode_cad(bytes: &[u8]) -> () {
    let mut block: HashMap<Cid, Vec<u8>> = HashMap::new();

    // Get header
    let (varint_value, bytes_read) = read_varint(bytes).unwrap();
    let (_, bytes) = bytes.split_at(bytes_read);
    let (header_cbor_block, car_data) = bytes.split_at(varint_value.into());
    let decoded_header: CarHeader = serde_cbor_2::from_slice(header_cbor_block).unwrap();
    println!("DECODED HEADER {:?}", decoded_header);
    // The 3 firsts bytes are... weird?
    let cid = Cid::try_from(decoded_header.roots[0].to_vec().split_at(3).1).unwrap();
    println!("DECODED CID {:?}", cid);
    // End get header

    // Decode first block
    let (varint_value, bytes_read) = read_varint(car_data).unwrap();
    let (_, car_data) = car_data.split_at(bytes_read);
    let (block_data, cad_data) = car_data.split_at(varint_value);
    match block_data {
        [_, _, 0x12, 0x20, ..] => {
            println!("CIDv0");
            let (cid_bytes, data_bytes) = block_data[2..].split_at(34);
            let data_cid = Cid::try_from(cid_bytes).unwrap();
            let decoded_data: serde_cbor_2::Value = serde_cbor_2::from_slice(data_bytes).unwrap();
            println!("DATA_CID {:?}", data_cid);
            println!("DECODED_DATA {:?}", decoded_data);
        }
        _ => println!("CIDv1"),
    }

    println!("Decoded header {:?}", &decoded_header)
}

fn read_varint(buffer: &[u8]) -> Option<(usize, usize)> {
    let mut value: usize = 0;
    let mut shift: u32 = 0;
    let mut bytes_read = 0;

    for &byte in buffer {
        value |= ((byte & 0x7F) as usize) << shift;
        shift += 7;
        bytes_read += 1;

        if byte & 0x80 == 0 {
            return Some((value, bytes_read));
        }
    }

    None
}
