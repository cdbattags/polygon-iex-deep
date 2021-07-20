use std::path;

use pcap::Capture;
use chrono;

#[derive(Debug, PartialEq)]
pub enum TradeDateFromFileErr {
    WrongFileExtension,
    NoStem,
    InvalidUnicode,
    InvalidDate,
}

#[derive(Debug, PartialEq)]
pub enum LoadPcapError {
    PcapError(pcap::Error),
}

pub fn load_capture_from_pcap<P: AsRef<path::Path>>(path: P) -> Result<pcap::Capture<pcap::Offline>, LoadPcapError> {
    Capture::from_file(path).or_else(|e| Err(LoadPcapError::PcapError(e)))
}

pub fn trade_date_from_path(deep_pcap: &str)
    -> Result<chrono::NaiveDate, TradeDateFromFileErr> {
    let path = path::Path::new(deep_pcap);

    path.file_stem()
        .ok_or_else(|| TradeDateFromFileErr::NoStem)
        .and_then(|stem| stem.to_str().ok_or_else(|| TradeDateFromFileErr::InvalidUnicode))
        .and_then(|stem| yyyymmdd_prefix_from_stem(&stem[11..19]))
}

pub fn yyyymmdd_prefix_from_stem(stem: &str)
    -> Result<chrono::NaiveDate, TradeDateFromFileErr> {
    chrono::NaiveDate::parse_from_str(stem, "%Y%m%d")
        .or(Err(TradeDateFromFileErr::InvalidDate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_file() {
        assert_eq!(
            load_capture_from_pcap("./data/help.pcap").err().unwrap(),
            LoadPcapError::PcapError(
                pcap::Error::PcapError(
                    "./data/help.pcap: No such file or directory".to_string(),
                ),
            ),
        );
    }

    #[test]
    fn trade_date_from_file_name() {
        assert_eq!(
            trade_date_from_path("./data/data_feeds_20210712_20210712_IEXTP1_DEEP1.0.txt").unwrap().to_string(),
            "2021-07-12"
        )
    }
}
