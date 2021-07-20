pub mod dt;
pub mod header;
pub mod message;
pub mod body;

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;

    fn read_sample() -> Vec<u8> {
        let mut file = File::open("./data/sample-packet.bin").unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();

        contents
    }

    #[test]
    fn test_parse_header() {
        let payload = read_sample();
        let header = header::parse_header(&payload as &[u8]).unwrap();

        assert!(header.version == 0x1);
        assert!(header.message_protocol_id == 0x8004);
    }

    #[test]
    fn test_parse_body() {
        let payload = read_sample();
        let header = header::parse_header(&payload as &[u8]).unwrap();
        let messages = body::parse_body(
            &payload[std::mem::size_of::<header::IexTpHeader>()..],
            0,
            header.first_message_sequence_number
        );

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].symbol().unwrap(), "NET");
    }
}
