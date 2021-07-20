use clap::{App,Arg};
use log::warn;
use chrono::TimeZone;
use chrono_tz::US::Eastern;

pub mod iex;
pub mod macros;
pub mod fs;
pub mod order_book;

use order_book::OrderBook;
use order_book::side::Side;
use iex::header::{parse_header,IexTpHeader};
use iex::message::IexDeepMessageImpl;
use iex::body::parse_body;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("IEX Deep Bookmake")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use (i.e. .pcap)")
            .required(true)
            .index(1))
        .arg(Arg::with_name("SYMBOL")
            .help("Symbol to make book for")
            .required(true)
            .index(2))
        .arg(Arg::with_name("TIME")
            .help("Time to get \"top of book\"")
            .required(true)
            .index(3))
        .get_matches();

    let pcap = matches.value_of("INPUT").unwrap();
    let symbol_to_book = matches.value_of("SYMBOL").unwrap();
    let time = matches.value_of("TIME").unwrap();

    let trade_date = fs::trade_date_from_path(pcap)
        .unwrap_or_else(|e| panic!("{:?}", e));

    let mut capture = match fs::load_capture_from_pcap(pcap) {
        Ok(cap) => cap,
        Err(e) => panic!("Failed to load {} with error: {:?}", pcap, e),
    };

    let time_to_book = Eastern.datetime_from_str(
        format!(
            "{} {}",
            trade_date,
            time,
        ).as_str(),
        "%Y-%m-%d %H:%M:%S%.6f",
    ).unwrap().timestamp_nanos();

    let mut last_tick_time = 0;
    let mut packet_counter = 0;
    let mut order_book = OrderBook::new();

    while let Ok(raw_packet) = capture.next() {
        let packet = match etherparse::SlicedPacket::from_ethernet(raw_packet.data) {
            Err(value) => panic!("Failed to parse from ethernet: {:?}", value),
            Ok(value) => value,
        };

        let iex_header = match parse_header(packet.payload) {
            Some(hdr) => hdr,
            None => panic!("Failed to parse header because it was too short"),
        };

        assert!(iex_header.version == 0x1);
        assert!(iex_header.message_protocol_id == 0x8004);

        let messages = parse_body(&packet.payload[std::mem::size_of::<IexTpHeader>()..], packet_counter, iex_header.first_message_sequence_number);

        for message in messages {
            let symbol = match message.symbol() {
                Some(symbol) => symbol,
                None => panic!("Trade tick needs to have a symbol"),
            };

            if symbol.eq(&symbol_to_book) {
                let side = match message.message_type as char {
                    '8' => {
                        Some(Side::Buy)
                    },
                    '5' => {
                        Some(Side::Sell)
                    },
                    _ => {
                        None
                    }
                };

                let (size, price) = match message.body {
                    IexDeepMessageImpl::PriceLevelUpdate(m) => {
                        (Some(m.size), Some(m.price))
                    },
                };

                order_book.add_limit(
                    side.unwrap(),
                    size.unwrap(),
                    price.unwrap(),
                );
            }

            last_tick_time = message.timestamp;
        }

        packet_counter += 1;

        if !last_tick_time.lt(&(time_to_book as u64)) {
            break;
        }
    }

    println!("buys:\n{}", order_book.buys.iter().rev().map(|(price, size)| format!("price: {}, size: {}\n", *price as f64 / 10_000 as f64, size)).collect::<String>());
    println!("sells:\n{}", order_book.sells.iter().map(|(price, size)| format!("price: {}, size: {}\n", *price as f64 / 10_000 as f64, size)).collect::<String>());
    println!("spread:\n{}\n", order_book.spread().unwrap() as f64 / 10_000 as f64);

    println!("packets processed:\n{}", packet_counter);
}
