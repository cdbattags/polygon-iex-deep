use crate::*;

pub_struct!(IexDeepMessage {
    message_type: u8,
    message_subtype: u8,
    timestamp: u64,
    body: IexDeepMessageImpl,
    packet_number: u64,
    message_sequence_number: u64,
});

impl IexDeepMessage {
    pub fn symbol(&self) -> Option<String> {
        match &self.body {
            IexDeepMessageImpl::PriceLevelUpdate(m) => {
                Some(
                    m.symbol
                        .iter()
                        .filter(|x|
                            match x {
                                ' ' => false,
                                _ => true,
                            }
                        )
                        .collect()
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IexDeepMessageImpl {
    PriceLevelUpdate(PriceLevelUpdateMessage),
}

pub_struct!(PriceLevelUpdateMessage {
    symbol: MessageSymbol,
    size: u32,
    price: u64,
    event_flags: PriceLevelUpdateEventFlags,
});

pub type MessageSymbol = [char; 8];

impl PriceLevelUpdateEventFlags {
    pub fn from_u8(byte: u8) -> Option<PriceLevelUpdateEventFlags> {
        match byte {
            0x0 => Some(PriceLevelUpdateEventFlags::OrderBookIsProcessingAnEvent),
            0x1 => Some(PriceLevelUpdateEventFlags::EventProcessingComplete),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriceLevelUpdateEventFlags {
    OrderBookIsProcessingAnEvent = 0x0,
    EventProcessingComplete = 0x1,
}

pub_struct!(ParseMessageResponse {
    parsed_message: IexDeepMessage,
    consumed_bytes: usize,
});

pub fn parse_message(bytes: &[u8], packet_num: u64, message_seq_num: u64) -> Option<ParseMessageResponse> {
    let message_type = bytes[0];
    let message_subtype = bytes[1];
    let timestamp = bytes_u64!(bytes, 2);

    match message_type as char {
        '8' | '5' => {
            PriceLevelUpdateEventFlags::from_u8(message_subtype).map(|event_flags| {
                let message = PriceLevelUpdateMessage {
                    event_flags,
                    symbol: [
                        bytes[10] as char,
                        bytes[11] as char,
                        bytes[12] as char,
                        bytes[13] as char,
                        bytes[14] as char,
                        bytes[15] as char,
                        bytes[16] as char,
                        bytes[17] as char,
                    ],
                    size: bytes_u32!(bytes, 18),
                    price: bytes_u64!(bytes, 22),
                };
                let consumed_bytes = std::mem::size_of_val(&message);
                let body = IexDeepMessageImpl::PriceLevelUpdate(message);
                ParseMessageResponse {
                    parsed_message: IexDeepMessage {
                        message_type,
                        message_subtype,
                        timestamp,
                        body,
                        packet_number: packet_num,
                        message_sequence_number: message_seq_num,
                    },
                    consumed_bytes,
                }
            })
        },
        _ => {
            warn!("unknown message type '{}' in packet {} message {}",
                  message_type, packet_num, message_seq_num);
            None
        },
    }
}
