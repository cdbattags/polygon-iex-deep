use std::collections::BTreeMap;
use std::fs::create_dir_all;
use std::env;

use crate::order_book::side::Side;

use sled::Db;

pub struct OrderBookDisk {
    dir: String,
    // TODO directory of individual order books "<SYMBOL>_BUY>" or "<SYMBOL>_SELL"
    // this doesn't solve the problem of book being a specific timestamp though
    tree_of_sled_paths: BTreeMap<String, String>,
}

impl OrderBookDisk {
    pub fn new(path: String) -> Self {
        let path_to_create = format!(
            "{}/{}",
            env::current_dir().unwrap().to_str().unwrap(),
            path,
        );

        create_dir_all(&path_to_create).unwrap();

        Self {
            dir: path_to_create,
            tree_of_sled_paths: BTreeMap::new(),
        }
    }

    pub fn add_limit(
        &mut self,
        symbol: String,
        side: Side,
        size: u32,
        price: u64,
    ) {
        todo!();
    }
}
