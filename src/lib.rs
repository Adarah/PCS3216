#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use std::collections::HashMap;
mod assembler;
mod cpu;

pub use crate::assembler::Assembler;
pub use crate::cpu::{Config, CPU};

#[derive(Default)]
pub struct Mnemonics<'a> {
    pub from_mnemonic: HashMap<&'a str, u16>,
    pub from_code: HashMap<u8, &'a str>,
}


impl<'a> Mnemonics<'a> {
    pub fn new() -> Mnemonics<'a> {
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
        Mnemonics {
            from_mnemonic,
            from_code,
        }
    }

    pub fn insert_symbol(&'a mut self, symbol: &'a str, address: u16) {
        self.from_mnemonic.insert(symbol, address);
    }
}


#[derive(Debug)]
struct Symbols {
    table: HashMap<String, u16>,
}

impl Symbols {
    pub fn new() -> Symbols {
        let mut table: HashMap<String, u16> = HashMap::new();
        table.insert("JP".to_string(), 0);
        table.insert("JZ".to_string(), 1);
        table.insert("JN".to_string(), 2);
        table.insert("LV".to_string(), 3);
        table.insert("+".to_string(), 4);
        table.insert("-".to_string(), 5);
        table.insert("*".to_string(), 6);
        table.insert("/".to_string(), 7);
        table.insert("LD".to_string(), 8);
        table.insert("MM".to_string(), 9);
        table.insert("SC".to_string(), 10);
        table.insert("RS".to_string(), 11);
        table.insert("HM".to_string(), 12);
        table.insert("GD".to_string(), 13);
        table.insert("PD".to_string(), 14);
        table.insert("OS".to_string(), 15);
        table.insert("@".to_string(), 16);
        table.insert("#".to_string(), 17);
        table.insert("K".to_string(), 18);

        Symbols { table }
    }

    pub fn insert(&mut self, key: &str, val: u16) {
        if self.table.insert(key.to_string(), val).is_some() {
            eprintln!("Label {} already previously found in symbols table", key);
            std::process::exit(1);
        }
    }

    pub fn get(&self, key: &str) -> u16 {
        *self.table.get(key).unwrap()
    }
}
