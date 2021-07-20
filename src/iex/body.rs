use log::trace;

use crate::*;
use crate::iex::message::{parse_message, IexDeepMessage};

pub fn parse_body(bytes: &[u8], packet_num: u64, message_seq_num_start: u64) -> Vec<IexDeepMessage> {
    let mut messages = Vec::new();
    let mut offset = 0;
    let mut message_seq_num = message_seq_num_start;
    while 2 + offset < bytes.len() {
        let message_length = bytes_u16!(bytes, offset);
        offset += 2;
        if message_length == 0 {
            warn!("encountered 0-length message at offset {}. breaking", offset);
            break;
        }
        if let Some(response) = parse_message(&bytes[offset..], packet_num, message_seq_num) {
            messages.push(response.parsed_message);
            trace!("consumed bytes: {}", response.consumed_bytes);
        } else {
            // warn!("Failed to parse message {} in packet {} at offset {}",
            //       message_seq_num, packet_num, offset);
        }
        offset += message_length as usize;
        message_seq_num += 1;
    }
    messages
}
