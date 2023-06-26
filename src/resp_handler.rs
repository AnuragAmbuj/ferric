
use std::io::Write;
use std::net::TcpStream;
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
    for token in frame_vector {
        match token {
            BlobString {data,attributes} => {
                let byte_slice: &[u8] = &data;
                let string = String::from_utf8_lossy(byte_slice);
                if string == "SET" {
                    //TODO
                }
                _ = write_message_to_stream(String::from("TOKEN PUSHED"),stream,&mut BytesMut::with_capacity(size))
            }
            BlobError {data,attributes} => {}
            SimpleString {data,attributes} => {}
            SimpleError {data,attributes} => {}
            Boolean {data,attributes} => {}
            Null => {}
            Number {data,attributes} => {}
            Double {data,attributes} => {}
            BigNumber {data,attributes} => {}
            VerbatimString { data, format, attributes } => {}
            Array { data,attributes} => {}
            Map { data,attributes } => {}
            Set { data,attributes } => {

            }
            Push { data,attributes } => {}
            Hello { version, auth } => {
                _= stream.write_all(&[version.to_byte()]);
            }
            ChunkedString(_) => {}
        }
    }
}