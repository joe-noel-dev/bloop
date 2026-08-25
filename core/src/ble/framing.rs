use std::{
    collections::HashMap,
    convert::TryInto,
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail, Result};

pub const PROTOCOL_VERSION: u8 = 1;
pub const HEADER_LENGTH: usize = 11;
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;
pub const MAX_CHUNKS: u32 = 262_144;
pub const MESSAGE_IDLE_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug)]
struct PendingMessage {
    message_id: u16,
    chunk_count: u32,
    chunks: HashMap<u32, Vec<u8>>,
    total_size: usize,
    last_activity: Instant,
}

#[derive(Debug, Default)]
pub struct Reassembler {
    pending: Option<PendingMessage>,
}

impl Reassembler {
    pub fn push(&mut self, value: &[u8]) -> Result<Option<Vec<u8>>> {
        self.expire_idle_message();
        let chunk = decode_chunk(value)?;

        match self.pending.as_ref() {
            Some(pending) if pending.message_id != chunk.message_id || pending.chunk_count != chunk.chunk_count => {
                bail!("BLE messages may not be interleaved")
            }
            None => {
                self.pending = Some(PendingMessage {
                    message_id: chunk.message_id,
                    chunk_count: chunk.chunk_count,
                    chunks: HashMap::new(),
                    total_size: 0,
                    last_activity: Instant::now(),
                });
            }
            _ => {}
        }

        let pending = self.pending.as_mut().expect("pending message was initialized");
        pending.last_activity = Instant::now();

        if let Some(existing) = pending.chunks.get(&chunk.chunk_index) {
            if existing == chunk.payload {
                return Ok(None);
            }
            self.pending = None;
            bail!("Conflicting duplicate BLE chunk")
        }

        if pending.total_size.saturating_add(chunk.payload.len()) > MAX_MESSAGE_SIZE {
            self.pending = None;
            bail!("BLE message exceeds the maximum size")
        }

        pending.total_size += chunk.payload.len();
        pending.chunks.insert(chunk.chunk_index, chunk.payload.to_vec());

        if pending.chunks.len() != pending.chunk_count as usize {
            return Ok(None);
        }

        let mut pending = self.pending.take().expect("complete pending message exists");
        let mut message = Vec::with_capacity(pending.total_size);
        for index in 0..pending.chunk_count {
            let payload = pending
                .chunks
                .remove(&index)
                .ok_or_else(|| anyhow!("BLE message is missing chunk {index}"))?;
            message.extend_from_slice(&payload);
        }
        Ok(Some(message))
    }

    pub fn reset(&mut self) {
        self.pending = None;
    }

    fn expire_idle_message(&mut self) {
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.last_activity.elapsed() >= MESSAGE_IDLE_TIMEOUT)
        {
            self.pending = None;
        }
    }
}

pub fn encode_message(message: &[u8], message_id: u16, maximum_value_length: usize) -> Result<Vec<Vec<u8>>> {
    if message.len() > MAX_MESSAGE_SIZE {
        bail!("BLE message exceeds the maximum size")
    }
    if maximum_value_length <= HEADER_LENGTH {
        bail!("BLE characteristic value length is too small for framing")
    }

    let payload_length = maximum_value_length - HEADER_LENGTH;
    let chunk_count = if message.is_empty() {
        1
    } else {
        message.len().div_ceil(payload_length)
    };
    if chunk_count > MAX_CHUNKS as usize {
        bail!("BLE message requires too many chunks")
    }

    let mut chunks = Vec::with_capacity(chunk_count);
    for chunk_index in 0..chunk_count {
        let start = chunk_index * payload_length;
        let end = message.len().min(start + payload_length);
        let payload = &message[start..end];
        let mut value = Vec::with_capacity(HEADER_LENGTH + payload.len());
        value.push(PROTOCOL_VERSION);
        value.extend_from_slice(&message_id.to_le_bytes());
        value.extend_from_slice(&(chunk_index as u32).to_le_bytes());
        value.extend_from_slice(&(chunk_count as u32).to_le_bytes());
        value.extend_from_slice(payload);
        chunks.push(value);
    }
    Ok(chunks)
}

struct DecodedChunk<'a> {
    message_id: u16,
    chunk_index: u32,
    chunk_count: u32,
    payload: &'a [u8],
}

fn decode_chunk(value: &[u8]) -> Result<DecodedChunk<'_>> {
    if value.len() < HEADER_LENGTH {
        bail!("BLE frame is shorter than its header")
    }
    if value[0] != PROTOCOL_VERSION {
        bail!("Unsupported BLE protocol version {}", value[0])
    }

    let message_id = u16::from_le_bytes([value[1], value[2]]);
    let chunk_index = u32::from_le_bytes(value[3..7].try_into().expect("four-byte chunk index"));
    let chunk_count = u32::from_le_bytes(value[7..11].try_into().expect("four-byte chunk count"));

    if chunk_count == 0 || chunk_count > MAX_CHUNKS {
        bail!("Invalid BLE chunk count {chunk_count}")
    }
    if chunk_index >= chunk_count {
        bail!("BLE chunk index {chunk_index} is outside chunk count {chunk_count}")
    }

    Ok(DecodedChunk {
        message_id,
        chunk_index,
        chunk_count,
        payload: &value[HEADER_LENGTH..],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_at_common_mtu_payload_sizes() {
        let message = (0..4096).map(|index| (index % 251) as u8).collect::<Vec<_>>();

        for maximum_value_length in [20, 182, 514] {
            let chunks = encode_message(&message, 42, maximum_value_length).unwrap();
            let mut reassembler = Reassembler::default();
            let mut result = None;
            for chunk in chunks {
                result = reassembler.push(&chunk).unwrap().or(result);
            }
            assert_eq!(result.as_deref(), Some(message.as_slice()));
        }
    }

    #[test]
    fn round_trips_empty_and_one_megabyte_messages() {
        for message in [Vec::new(), vec![0xa5; 1024 * 1024]] {
            let mut reassembler = Reassembler::default();
            let chunks = encode_message(&message, 7, 182).unwrap();
            let mut result = None;
            for chunk in chunks {
                result = reassembler.push(&chunk).unwrap().or(result);
            }
            assert_eq!(result, Some(message));
        }
    }

    #[test]
    fn accepts_shuffled_and_identical_duplicate_chunks() {
        let message = vec![0x5a; 1024];
        let mut chunks = encode_message(&message, 9, 64).unwrap();
        chunks.reverse();
        chunks.insert(1, chunks[0].clone());
        let mut reassembler = Reassembler::default();
        let mut result = None;

        for chunk in chunks {
            result = reassembler.push(&chunk).unwrap().or(result);
        }

        assert_eq!(result, Some(message));
    }

    #[test]
    fn rejects_conflicting_duplicate_chunks() {
        let message = vec![0x5a; 128];
        let chunks = encode_message(&message, 9, 64).unwrap();
        let mut conflicting = chunks[0].clone();
        *conflicting.last_mut().unwrap() ^= 0xff;
        let mut reassembler = Reassembler::default();

        assert!(reassembler.push(&chunks[0]).unwrap().is_none());
        assert!(reassembler.push(&conflicting).is_err());
    }

    #[test]
    fn rejects_invalid_headers_and_oversized_messages() {
        let mut invalid_version = encode_message(b"hello", 1, 20).unwrap().remove(0);
        invalid_version[0] = 2;
        assert!(Reassembler::default().push(&invalid_version).is_err());
        assert!(encode_message(&vec![0; MAX_MESSAGE_SIZE + 1], 1, 64).is_err());
    }

    #[test]
    fn abandons_an_idle_incomplete_message() {
        let first = encode_message(&vec![1; 100], 1, 64).unwrap().remove(0);
        let replacement = encode_message(b"replacement", 2, 64).unwrap().remove(0);
        let mut reassembler = Reassembler::default();
        assert!(reassembler.push(&first).unwrap().is_none());
        reassembler.pending.as_mut().unwrap().last_activity = Instant::now() - MESSAGE_IDLE_TIMEOUT;

        assert_eq!(reassembler.push(&replacement).unwrap(), Some(b"replacement".to_vec()));
    }
}
