mod bank;

use shared::gw_error::GwError;

fn main() -> Result<(), GwError> {
    let mut b = bank::Bank::init()?;
    b.run()
}
