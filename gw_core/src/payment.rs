use crate::card_scheme::CardScheme;
use validify::Validate;

pub type ExpiryDate = (u32, u8);

#[derive(Clone, Debug, PartialEq, Validate)]
// #[validate(validate_payment)]
pub enum Payment {
    Card {
        scheme: CardScheme,
        expiry_date: ExpiryDate,
        #[validate(length(min = 3, max = 4, message = "invalid length"))]
        security_code: String,
        #[validate(length(min = 16, max = 20, message = "invalid length"))]
        pan: String,
    },
    Account {
        account_number: String,
        sort_code: String,
    },
}

impl From<(CardScheme, ExpiryDate, &str, &str)> for Payment {
    fn from(value: (CardScheme, ExpiryDate, &str, &str)) -> Self {
        // TODO test
        Payment::Card {
            scheme: value.0,
            expiry_date: value.1.to_owned(),
            security_code: value.2.to_owned(),
            pan: value.3.to_owned(),
        }
    }
}

// fn validate_payment(p: &Payment) -> Result<(), ValidationErrors> {
//     match p {
//         Payment::Card {
//             security_code,
//             pan,
//             expiry_date,
//             scheme,
//         } => validate_card(&security_code, &pan, *scheme, *expiry_date),
//         Payment::Account {
//             account_number,
//             sort_code,
//         } => Ok(()),
//     }
// }

// #[schema_validation]
// fn validate_card(
//     security_code: &str,
//     pan: &str,
//     scheme: CardScheme,
//     expiry_date: ExpiryDate,
// ) -> Result<(), ValidationErrors> {
//     if security_code.len() < 3 || security_code.len() > 4 {
//         schema_err!("length", "security_code");
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{check_validation, ExpectedValidationErrors, ValidationErrorKind as V};
    use rstest::*;

    #[rstest]
    #[case((CardScheme::Visa, (2021, 3), "123", "4000111122223333"), vec![])]
    #[case((CardScheme::Visa, (2021, 3), "12345", "4000111122223333"), vec![(V::Field, "security_code", "length", "invalid length", "security_code", vec![("min", 3.into()), ("max", 4.into()), ("actual", 5.into())])])]
    #[case((CardScheme::Visa, (2021, 3), "12", "4000111122223333"), vec![(V::Field, "security_code", "length", "invalid length", "security_code", vec![("min", 3.into()), ("max", 4.into()), ("actual", 2.into())])])]
    #[case((CardScheme::Visa, (2021, 3), "123", "400011112222333"), vec![(V::Field, "pan", "length", "invalid length", "pan", vec![("min", 16.into()), ("max", 20.into()), ("actual", 15.into())])])]
    #[case((CardScheme::Visa, (2021, 3), "123", "40001111222233334444"), vec![])]
    #[case((CardScheme::Visa, (2021, 3), "123", "400011112222333344445"), vec![(V::Field, "pan", "length", "invalid length", "pan", vec![("min", 16.into()), ("max", 20.into()), ("actual", 21.into())])])]
    #[case((CardScheme::Visa, (2021, 3), "12345", "400011112"), vec![(V::Field, "security_code", "length", "invalid length", "security_code", vec![("min", 3.into()), ("max", 4.into()), ("actual", 5.into())]), (V::Field, "pan", "length", "invalid length", "pan", vec![("min", 16.into()), ("max", 20.into()), ("actual", 9.into())])])]
    fn test_validate_card(
        #[case] inputs: (CardScheme, ExpiryDate, &str, &str),
        #[case] errors: ExpectedValidationErrors,
    ) {
        check_validation(Payment::from(inputs), errors);
    }
}
