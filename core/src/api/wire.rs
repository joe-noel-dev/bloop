use anyhow::Result;
use protobuf::Message;

use crate::bloop::{Request, Response};

pub fn decode_request(bytes: &[u8]) -> Result<Request> {
    Ok(Request::parse_from_bytes(bytes)?)
}

pub fn encode_response(response: &Response) -> Result<Vec<u8>> {
    Ok(response.write_to_bytes()?)
}
