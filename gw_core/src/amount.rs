use std::{fmt::Display, marker::PhantomData};

use serde::Serialize;

use crate::currency::Currency;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Amount<R: Repr> {
    pub value: u64,
    pub currency: Currency,
    _marker: PhantomData<R>,
}

impl From<(u64, Currency)> for Amount<BASE> {
    fn from(value: (u64, Currency)) -> Amount<BASE> {
        Amount {
            value: value.0,
            currency: value.1,
            _marker: PhantomData {},
        }
    }
}

impl From<u64> for Amount<BASE> {
    fn from(value: u64) -> Amount<BASE> {
        Amount {
            value,
            currency: Currency::default(),
            _marker: PhantomData {},
        }
    }
}

/// BASE reprsents the Amount as a whole integer.
/// DEC represents the amount as a decimal if the currency permits.
pub struct BASE;
pub struct DEC;
pub trait Repr {}
impl Repr for BASE {}
impl Repr for DEC {}

impl Amount<BASE> {
    fn to_dec(self) -> Amount<DEC> {
        Amount {
            value: self.value,
            currency: self.currency,
            _marker: PhantomData,
        }
    }
}

impl Display for Amount<BASE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.currency, self.value)
    }
}

impl Amount<DEC> {
    fn to_base(self) -> Amount<BASE> {
        Amount {
            value: self.value,
            currency: self.currency,
            _marker: PhantomData,
        }
    }
}

impl Display for Amount<DEC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dec_places = self.currency.get_decimal_places();
        let mut value = self.value.to_string();
        if value == "0" && dec_places > 0 {
            value.push_str("00");
        }
        if dec_places > 0 {
            write!(
                f,
                "{}{}.{}",
                self.currency,
                &value[..dec_places - 1],
                &value[dec_places - 1..]
            )
        } else {
            write!(f, "{}{}", self.currency, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Amount::from((123, Currency::GBP)), "123", "1.23")]
    #[case(Amount::from(123), "123", "1.23")]
    #[case(Amount::from((0, Currency::GBP)), "0", "0.00")]
    #[case(Amount::from((123, Currency::JPY)), "123", "123")]
    fn test_display(#[case] amount: Amount<BASE>, #[case] exp_base: &str, #[case] exp_dec: &str) {
        assert_eq!(amount.to_string(), exp_base);
        let amount = amount.to_dec();
        assert_eq!(amount.to_string(), exp_dec);
        let amount = amount.to_base();
        assert_eq!(amount.to_string(), exp_base);
    }
}
