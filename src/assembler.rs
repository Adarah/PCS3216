use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::{prelude::*, BufReader};
use crate::Symbols;

pub struct Assembler {
    file: String,
    line_count: u16,
    symbols: Symbols,
    listing: Vec<Vec<String>>,
    distances: Vec<u16>,
}

impl Assembler {
    pub fn run(filename: String) {
        if fs::remove_file("program.bin").is_ok() {
            info!("Removed previously existing program.bin");
        }
        let mut ass = Assembler::new(filename);
        ass.run_first_pass();
        let buffer = ass.run_second_pass();
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open("program.bin")
            .unwrap_or_else(|err| panic!("{}", err));

        match f.write(&buffer) {
            Ok(_) => info!("Binary successfully generated"),
            Err(err) => panic!("{}", err),
        };
    }

    fn new(file: String) -> Assembler {
        let line_count = 0;
        let symbols = Symbols::new();
        let listing = vec![];
        let distances = vec![0];
        Assembler {
            file,
            symbols,
            line_count,
            listing,
            distances,
        }
    }

    fn run_first_pass(&mut self) {
        let file = self.load_file();
        info!("Starting first pass of the assembler");
        let statements = self.get_valid_statements(file);
        for statement in statements {
            if self.handle_statement(statement) {
                break;
            }
        }
        let d_len = self.distances.len() - 1;
        if d_len < 1 {
            eprintln!("File does not have enough basic blocks: A basic block must
 start with @ /xyz, and end with # LABEL");
            std::process::exit(1);
        }
        self.distances = self.distances[1..d_len].to_vec();
    }

    fn run_second_pass(&mut self) -> Vec<u8> {
        info!("Starting second pass of the assembler");
        // println!("{:?}", self.distances);
        // println!("{:?}", self.symbols);
        // let mut base = 0;
        let mut byte_buffer: Vec<u8> = vec![];
        // This is the number of punch cards we have to read.
        byte_buffer.push(self.distances.len() as u8);
        for statement in self.listing.iter() {
            trace!("{:?}", statement);
            let mnemonic = &statement[0];
            let argument = &statement[1];
            let code = self.symbols.get(&mnemonic);
            let arg = self.convert_argument(argument);
            if mnemonic == "@" {
                let word = format!("{:04X}", arg);
                trace!("{}", word);
                let (msb, lsb) = self.split_word(word);
                byte_buffer.push(msb);
                byte_buffer.push(lsb);
                byte_buffer.push(self.distances.remove(0) as u8);
                continue;
            }
            if mnemonic == "K" {
                let word = format!("{:04X}", arg);
                trace!("{}", word);
                let (_, lsb) = self.split_word(word);
                byte_buffer.push(lsb);
                continue;
            }
            trace!("{:X}{:03X}", code, arg);
            let (msb, lsb) = self.split_word(format!("{:X}{:03X}", code, arg));
            byte_buffer.push(msb);
            byte_buffer.push(lsb);
            // println!("{:?}", byte_buffer);
        }
        // for byte in byte_buffer.iter() {
        //     println!("{:02X}", byte);
        // }
        byte_buffer
    }

    fn split_word(&self, word: String) -> (u8, u8) {
        let msb = u8::from_str_radix(&word[..2], 16).unwrap();
        let lsb = u8::from_str_radix(&word[2..], 16).unwrap();
        (msb, lsb)
    }

    fn parse_nums(&self, argument: &str) -> u16 {
        if argument.starts_with('/') {
            let a =  u16::from_str_radix(&argument[1..], 16).unwrap();
            return a;
        }
        argument.parse::<u16>().unwrap()
    }

    fn convert_argument(&self, argument: &str) -> u16 {
        if argument.contains('+') {
            let words = argument
                .split('+')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            return self.symbols.get(&words[0]) + words[1].parse::<u16>().unwrap();
        }
        if self.symbols.table.get(argument).is_some() {
            return self.symbols.get(&argument.to_string());
        }
        if argument.contains('"') {
            return argument.chars().nth(1).unwrap() as u16;
        }
        self.parse_nums(argument)
    }

    fn handle_statement(&mut self, mut statement: Vec<String>) -> bool {
        let n = statement.len();
        let label = &statement[0];
        // println!("{:?}", statement);
        if n == 2 {
            if label.starts_with('@') {
                let new_linecount = self.parse_nums(&statement[1]);
                self.update_distances(new_linecount);
                self.line_count = new_linecount;
                self.listing.push(statement);
            } else if label.starts_with('#') {
                self.update_distances(self.line_count);
                statement[0] = String::from("JP");
                self.listing.insert(0, statement);
                // TODO: Find a better solution than this hack
                return true; // signals we have to break
            } else {
                self.listing.push(statement.clone());
                self.line_count += 2;
                if label.starts_with('K') {
                    self.line_count -= 1;
                }
            }
        } else if n == 1 {
            self.symbols.insert(label, self.line_count);
        } else if n == 3 {
            self.symbols.insert(label, self.line_count);
            statement.remove(0);
            if statement[0].starts_with('K') {
                self.line_count -= 1;
            }
            self.listing.push(statement);
            self.line_count += 2;
        } else {
            panic!("Line has more than 3 'words': {}", statement.join(" "))
        }
        false
    }

    fn update_distances(&mut self, new_linecount: u16) {
        let n = self.distances.len() - 1;
        self.distances[n] = self.line_count - self.distances.last().unwrap();
        self.distances.push(new_linecount);
        // println!("{:?}", self.distances);
        // println!("{:?}", self.line_count);
    }

    fn load_file(&self) -> Vec<String> {
        info!("Opening input asm file");
        let file = match File::open(&self.file) {
            Ok(x) => x,
            Err(_) => std::process::exit(1),
        };
        let reader = BufReader::new(file);
        reader.lines().map(|x| x.unwrap()).collect()
    }

    fn get_valid_statements(&self, lines: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        for line in lines {
            let words: Vec<String> = line
                .split(' ')
                .take_while(|&word| !word.starts_with(';'))
                .filter(|&x| x != "")
                .map(|x| x.to_owned())
                .collect();
            if !words.is_empty() {
                result.push(words);
            }
        }
        result
    }
}
