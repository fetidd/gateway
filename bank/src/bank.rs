use shared::{
    codec::{BankCodec, Codec},
    transaction::Transaction,
};
use std::io::Read;
use std::net;

const ADDR: &str = "127.0.0.1";
const PORT: &str = "1234";

pub struct Bank {
    listener: net::TcpListener,
}

impl Bank {
    pub fn init() -> Result<Self, std::io::Error> {
        let l = net::TcpListener::bind(&format!("{}:{}", ADDR, PORT))?;
        Ok(Bank { listener: l })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            let mut buf = String::new();
            stream?.read_to_string(&mut buf)?;
            println!("received: {}", &buf);
        }
        Ok(())
    }
}
