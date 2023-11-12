mod bank;

fn main() -> Result<(), std::io::Error> {
    let mut b = bank::Bank::init()?;
    b.run()
}
