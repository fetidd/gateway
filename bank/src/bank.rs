use shared::{
    codec::{BankCodec, Codec},
    gw_error::GwError,
    transaction::Transaction,
};
use std::io::Read;
use std::net;

const ADDR: &str = "127.0.0.1";
const PORT: &str = "1234";

pub struct Bank {
    listener: net::TcpListener,
    codec: BankCodec,
}

impl Bank {
    pub fn init() -> Result<Self, GwError> {
        let l = net::TcpListener::bind(&format!("{}:{}", ADDR, PORT))?;
        let b = BankCodec::init();
        Ok(Bank {
            listener: l,
            codec: b,
        })
    }

    pub fn run(&mut self) -> Result<(), GwError> {
        for stream in self.listener.incoming() {
            let mut buf = String::new();
            stream?.read_to_string(&mut buf)?;
            println!("received: {:?}", &buf);
            let mut t = Transaction::default();
            self.codec.decode(&mut t, &buf)?;
        }
        Ok(())
    }
}
