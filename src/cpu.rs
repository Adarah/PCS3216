use crate::Mnemonics;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::fs;
use std::io;
use std::io::{Read, Write};

pub struct Config {
    pub input: String,
    pub output: String,
    pub trace: bool,
    pub memory: Vec<u8>,
}

impl Config {
    pub fn new(input: String, output: String, loader: String, trace: bool) -> Config {
        let mut memory = vec![0; 4096];
        let mut loader_file = fs::File::open(loader).unwrap();
        let mut loader_buffer = Vec::new();
        loader_file.read_to_end(&mut loader_buffer).unwrap();
        // let loader = include_bytes!(loader);
        for (idx, byte) in loader_buffer.iter().skip(6).enumerate() {
            memory[idx] = *byte;
        }
        trace!("Memory initialized: {:?}", memory);
        Config { input, output, trace, memory }
    }
}

pub struct CPU {
    memory: Vec<u8>,
    pc: u16,
    ac: i8,
    trace: bool,
    input_file: io::Bytes<fs::File>,
    output_file: String,
}

impl CPU {
    pub fn run(config: Config) {
        info!("Initializing CPU");
        let mut cpu = CPU::new(config).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        let table = Mnemonics::new().from_code;
        info!("Starting code execution");
        loop {
            trace!("PC is {}", cpu.pc);
            trace!("AC is {}", cpu.ac);
            let next_instruction = cpu.fetch();
            cpu.decode_and_execute(next_instruction, &table);
            debug!("");
            if cpu.trace {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");
                println!("{}", input);
            }
            // trace!("{:?}", cpu.memory);
        }
    }

    fn new(config: Config) -> Result<CPU, Box<dyn Error>> {
        let memory = config.memory;
        let pc = 0;
        let ac = 0;
        let trace = config.trace;
        let f = fs::File::open(config.input)?;
        let input_file = f.bytes();
        let output_file = config.output;
        if fs::remove_file(&output_file).is_ok() {
            info!("Overwrote previously existing output.bin");
        }
        let save_path = Path::new(&output_file);
        if !save_path.exists() {
            fs::create_dir_all(save_path.parent().unwrap()).unwrap();
        }

        Ok(CPU {
            memory,
            pc,
            ac,
            trace,
            input_file,
            output_file,
        })
    }

    fn fetch(&mut self) -> (u8, u8) {
        let msb = self.memory[self.pc as usize];
        let lsb = self.memory[(self.pc + 1) as usize];
        self.pc += 2;
        debug!("Fetched instruction {:02X}{:02X}", msb, lsb);
        (msb, lsb)
    }

    fn decode_and_execute(&mut self, instruction: (u8, u8), table: &HashMap<u8, &str>) {
        // TODO: Fix this terrible "table" solution: use Enums instead
        // TODO: Group functions into groups to minimize log repetition
        if self.trace {
            self.treat_user_input();
        }
        let (msb, lsb) = instruction;
        let opcode = (msb & 0xF0) >> 4;
        let arg = ((0x0F & msb as u16) << 8) + lsb as u16;
        let f = match opcode {
            0 => CPU::jmp,
            1 => CPU::jmp_if_zero,
            2 => CPU::jmp_if_neg,
            3 => CPU::load_value,
            4 => CPU::add,
            5 => CPU::sub,
            6 => CPU::mul,
            7 => CPU::div,
            8 => CPU::load_data,
            9 => CPU::move_to_memory,
            10 => CPU::subroutine_call,
            11 => CPU::return_from_subroutine,
            12 => CPU::halt_machine,
            13 => CPU::get_data,
            14 => CPU::put_data,
            15 => CPU::os_call,
            _ => panic!("Unknown function called"),
        };
        let foo_name = table.get(&opcode).unwrap();
        debug!("Executing {} {:03X}", foo_name, arg);
        f(self, arg)
    }

    fn treat_user_input(&self) {}

    fn jmp(&mut self, arg: u16) {
        self.pc = arg;
        debug!("PC set to {:03X} ({} in decimal)", arg, arg);
    }

    fn jmp_if_zero(&mut self, arg: u16) {
        if self.ac == 0 {
            self.pc = arg;
            debug!("PC set to {:03X} ({} in decimal)", arg, arg);
        } else {
            debug!("No jump");
        }
    }

    fn jmp_if_neg(&mut self, arg: u16) {
        if self.ac < 0 {
            self.pc = arg;
            debug!("PC set to {:03X} ({} in decimal)", arg, arg);
        } else {
            debug!("No jump");
        }
    }

    fn load_value(&mut self, arg: u16) {
        self.ac = arg as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn add(&mut self, arg: u16) {
        self.ac += self.memory[arg as usize] as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn sub(&mut self, arg: u16) {
        self.ac -= self.memory[arg as usize] as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn mul(&mut self, arg: u16) {
        self.ac *= self.memory[arg as usize] as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn div(&mut self, arg: u16) {
        self.ac /= self.memory[arg as usize] as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn load_data(&mut self, arg: u16) {
        self.ac = self.memory[arg as usize] as i8;
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn move_to_memory(&mut self, arg: u16) {
        self.memory[arg as usize] = self.ac as u8;
        debug!(
            "Mem pos {:03X} set to {:02X} ({} in decimal)",
            arg, self.ac, self.ac
        );
    }

    fn subroutine_call(&mut self, arg: u16) {
        let msb = (self.pc & 0x0F00 >> 8) as u8;
        let lsb = (self.pc & 0x00FF) as u8;
        self.memory[arg as usize] = msb;
        self.memory[(arg + 1) as usize] = lsb;
        self.pc = arg + 2;
        debug!("PC set to {:03X} ({} in decimal)", self.pc, self.pc)
    }

    fn return_from_subroutine(&mut self, arg: u16) {
        let msb = (0x0F & self.memory[arg as usize] as u16) << 8;
        let lsb = self.memory[(arg + 1) as usize] as u16;
        self.pc = msb + lsb;
        debug!("PC set to {:03X} ({} in decimal)", self.pc, self.pc)
    }

    fn halt_machine(&mut self, arg: u16) {
        info!("Halting machine.");
        trace!("{:?}", self.memory);
        self.pc = arg;
        std::process::exit(0);
    }

    fn get_data(&mut self, _: u16) {
        self.ac = match self.input_file.next() {
            Some(res) => res.unwrap_or_else(|err| panic!(err)) as i8,
            None => {
                eprintln!("Trying to read after EOF");
                0
            }
        };
        debug!("AC set to {:02X} ({} in decimal)", self.ac, self.ac);
    }

    fn put_data(&mut self, _: u16) {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file)
            .unwrap_or_else(|err| panic!("{}", err));

        match f.write(&[self.ac as u8]) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
        debug!(
            "Wrote {:02X} ({} in decimal) to output file",
            self.ac, self.ac
        );
    }

    fn os_call(&mut self, _: u16) {
        unimplemented!()
    }
}
