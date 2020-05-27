#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use std::collections::HashMap;
mod assembler;
mod cpu;

pub use crate::assembler::Assembler;
pub use crate::cpu::{Config, CPU};

pub struct MnemonicsTable<'a> {
    pub from_mnemonic: HashMap<&'a str, u16>,
    pub from_code: HashMap<u8, &'a str>,
}

impl<'a> MnemonicsTable<'a> {
    pub fn new() -> MnemonicsTable<'a> {
        let mut from_mnemonic: HashMap<&str, u16> = HashMap::new();
        from_mnemonic.insert("jp", 0);
        from_mnemonic.insert("jz", 1);
        from_mnemonic.insert("jn", 2);
        from_mnemonic.insert("lv", 3);
        from_mnemonic.insert("+", 4);
        from_mnemonic.insert("-", 5);
        from_mnemonic.insert("*", 6);
        from_mnemonic.insert("/", 7);
        from_mnemonic.insert("ld", 8);
        from_mnemonic.insert("mm", 9);
        from_mnemonic.insert("sc", 10);
        from_mnemonic.insert("rs", 11);
        from_mnemonic.insert("hm", 12);
        from_mnemonic.insert("gd", 13);
        from_mnemonic.insert("pd", 14);
        from_mnemonic.insert("os", 15);

        let mut from_code: HashMap<u8, &str> = HashMap::new();
        for (k, v) in from_mnemonic.iter() {
            from_code.insert(*v as u8, *k);
        }
        MnemonicsTable {
            from_mnemonic,
            from_code,
        }
    }

    pub fn insert_symbol(&'a mut self, symbol: &'a str, address: u16) {
        self.from_mnemonic.insert(symbol, address);
    }
}
