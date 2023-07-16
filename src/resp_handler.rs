use std::io::Write;
use std::net::TcpStream;
use std::ops::Deref;
use bytes::{Bytes, BytesMut};
use Frame::Array;
use Frame::BigNumber;
use Frame::BlobError;
use Frame::BlobString;
use Frame::Boolean;
use Frame::ChunkedString;
use Frame::Double;
use Frame::Hello;
use Frame::Map;
use Frame::Null;
use Frame::Number;
use Frame::Push;
use Frame::Set;
use Frame::SimpleError;
use Frame::SimpleString;
use Frame::VerbatimString;
use redis_protocol::resp3::decode::complete::decode;
use redis_protocol::resp3::encode::complete::encode_bytes;
use redis_protocol::resp3::prelude::Frame;
use redis_protocol::resp3::types::Attributes;
use redis_protocol::types::RedisProtocolError;
use crate::cache_ops::map_ops::MAP_CACHE;

pub fn handle_query_requests(resp_command: [u8; 1024], mut stream: &mut TcpStream) {
    let bytes = Bytes::copy_from_slice(resp_command.as_slice());
    match decode(&bytes) {
        Ok(Some((frame,size))) => {
            handle_command_frame(frame,size,stream)
        }
        Err(err) => {
            _ = stream.write_all(err.description().as_bytes())
        }
        _=> {
            //Do nothing
        }
    };
}

fn handle_command_frame(frame: Frame, size: usize, stream: &mut TcpStream) {
    match frame {
        BlobString { data, attributes } => {
            println!("Size{}, data {:?}",size, data);
        }
        BlobError { data, attributes  } => {
            println!("Size{}, data {:?}",size, data);
        }
        SimpleString { data, attributes  } => {
            println!("Size{}, data {:?}",size, data);
        }
        SimpleError { data, attributes  } => {
            println!("Size{}, data {}",size, data);
        }
        Boolean { data, attributes  } => {
            println!("Size{}, data {}",size, data);
        }
        Null => println!("Null data received"),
        Number { data, attributes } => {
            println!("Size{}, data {}",size, data);
        }
        Double { data, attributes } => {
            println!("Size{}, data {}",size, data);
        }
        BigNumber { data, attributes } => {
            println!("Size{}, data {:?}",size, data);
        }
        VerbatimString { data, format, attributes } => {
            println!("Size{}, data {:?}",size, data);
        }
        Array { data, attributes } => {
            println!("Size{}, data {:?}",size, data);
            handle_repl_command(data,attributes,size,stream);
        }
        Map { data, attributes } => {
            println!("Size{}, data {:?}",size, data);
        }
        Set { data, attributes } => {
            println!("Size{}, data {:?}",size, data);
        }
        Push { data, attributes } => {
            println!("Size{}", size);
        }
        Hello { version, auth } => {
            println!("{:?},{}",version.to_byte(),auth.unwrap().username)
        }
        ChunkedString( chunk_in_bytes)=> {
            println!("Size{}, data {:?}",size, chunk_in_bytes);
        }
    }
}

fn write_message_to_stream(message: String, stream: &mut TcpStream, buf: &mut BytesMut) -> Result<(), RedisProtocolError>{
    let frame = BlobString { data: message.into(), attributes: None };
    let offset = encode_bytes(buf, &frame).expect("Failed to encode frame");
    stream.write_all(buf).expect("Failed to write to socket");
    let _ = buf.split_to(offset);
    Ok(())
}

fn handle_repl_command(frame_vector: Vec<Frame>,
                       attributes_wrapped: Option<Attributes>,
                       size: usize,
                       stream: &mut TcpStream) {
    let mut command_token_in_bytes: Vec<Bytes> = Vec::new();
    for (i,token) in frame_vector.iter().enumerate() {
        command_token_in_bytes.insert(i, Bytes::copy_from_slice(token.as_bytes().unwrap()));
    }
    let command = command_token_in_bytes.get(0).unwrap();
    let command_string = String::from_utf8_lossy(command);
    let result: String = execute_command(command_string.to_string(),command_token_in_bytes);
    _ = write_message_to_stream(result,stream,&mut BytesMut::with_capacity(size))
}

fn execute_command(command: String, command_token_in_bytes: Vec<Bytes>) -> String {
    match command.as_str() {
        "SET" => {
            if let storage_key  =  command_token_in_bytes.get(1) {
                let key = storage_key.unwrap();

            } else {
                return String::from("INVALID SET STATEMENT");
            }

        },
        "GET" => {
            return String::from("VALUE RETRIEVED")
        }
        _ => return String::from("Invalid Command")
    }
}