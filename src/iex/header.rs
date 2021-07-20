use crate::*;

pub_struct!(IexTpHeader {
    version: u8,
    reserved: u8,
    message_protocol_id: u16,
    channel_id: u32,
    session_id: u32,
    payload_length: u16,
    message_count: u16,
    stream_offset: u64,
    first_message_sequence_number: u64,
    send_time: iex::dt::UtcNs,
});

pub fn parse_header(bytes: &[u8]) -> Option<IexTpHeader> {
    let iex_header_length = std::mem::size_of::<IexTpHeader>();
    assert!(iex_header_length == 40);
    if bytes.len() < iex_header_length {
        return None;
    }

    Some(IexTpHeader {
        version: bytes[0],
        reserved: bytes[1],
        message_protocol_id: bytes_u16!(bytes, 2),
        channel_id: bytes_u32!(bytes, 4),
        session_id: bytes_u32!(bytes, 8),
        payload_length: bytes_u16!(bytes, 12),
        message_count: bytes_u16!(bytes, 14),
        stream_offset: bytes_u64!(bytes, 16),
        first_message_sequence_number: bytes_u64!(bytes, 24),
        send_time: bytes_u64!(bytes, 32),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn sample_header_parse() {
        let mut file = File::open("./data/sample-packet.bin").unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();

        let conv: &[u8] = &contents;
        let header = parse_header(conv).unwrap();

        assert!(header.version == 0x1);
        assert!(header.message_protocol_id == 0x8004);
    }
}
