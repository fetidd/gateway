use std::fmt::Display;

use serde::Serialize;

use crate::currency::Currency;

#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Amount {
    Base { val: u64, cur: Currency },
    Decimal { val: u64, cur: Currency },
}

impl From<(u64, Currency)> for Amount {
    fn from(value: (u64, Currency)) -> Amount {
        Amount::Base {
            val: value.0,
            cur: value.1,
        }
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Amount {
        Amount::Base {
            val: value,
            cur: Currency::default(),
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::Base { val, .. } => write!(f, "{}", val),
            Amount::Decimal { val, cur } => {
                let dec_places = cur.get_decimal_places();
                let mut value = val.to_string();
                if value == "0" && dec_places > 0 {
                    value.push_str("00");
                }
                if dec_places > 0 {
                    write!(
                        f,
                        "{}.{}",
                        &value[..dec_places - 1],
                        &value[dec_places - 1..]
                    )
                } else {
                    write!(f, "{}", value)
                }
            }
        }
    }
}

impl Amount {
    pub fn value(&self) -> u64 {
        match self {
            Amount::Base { val, .. } | Amount::Decimal { val, .. } => *val,
        }
    }

    pub fn currency(&self) -> Currency {
        match self {
            Amount::Base { cur, .. } | Amount::Decimal { cur, .. } => *cur,
        }
    }

    pub fn to_dec(self) -> Self {
        match self {
            Amount::Base { val, cur } => Amount::Decimal { val, cur },
            Amount::Decimal { .. } => self,
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
    fn test_display(#[case] amount: Amount, #[case] exp_base: &str, #[case] exp_dec: &str) {
        assert_eq!(amount.to_string(), exp_base);
        assert_eq!(amount.to_dec().to_string(), exp_dec);
    }
}
