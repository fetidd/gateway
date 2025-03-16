#[derive(Debug, Clone, PartialEq)]
pub struct BankOneAccount {
    pub merchant_identification_value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BankTwoAccount {
    pub merchant_reference: String,
}

#[derive(Debug, PartialEq)]
pub enum AcquirerAccount {
    BankOne(BankOneAccount),
    BankTwo(BankTwoAccount),
}

trait Iso8853<'a> {
    fn merchant_id(&'a self) -> &'a str;
}

impl<'a> Iso8853<'a> for BankOneAccount {
    fn merchant_id(&'a self) -> &'a str {
        &self.merchant_identification_value
    }
}

trait Apacs30<'a> {
    fn merchant_id(&'a self) -> &'a str;
}

impl<'a> Apacs30<'a> for BankOneAccount {
    fn merchant_id(&'a self) -> &'a str {
        &self.merchant_identification_value[0..1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_inner() {
        let acq_acct = AcquirerAccount::BankOne(BankOneAccount {
            merchant_identification_value: "123".into(),
        });
        if let AcquirerAccount::BankOne(acct) = acq_acct {
            assert_eq!(<BankOneAccount as Iso8853>::merchant_id(&acct), "123");
            assert_eq!(<BankOneAccount as Apacs30>::merchant_id(&acct), "1");
        }
    }
}
