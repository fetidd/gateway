use validify::Validify;

use crate::country::Country;

#[derive(Clone, Debug, PartialEq, Validify, Default)]
pub struct Merchant {
    merchant_id: String,
    #[modify(trim)]
    name: String,
    #[modify(trim)]
    premise: String,
    #[modify(trim)]
    street: String,
    #[modify(trim)]
    city: String,
    #[modify(trim)]
    postcode: String,
    #[modify(trim)]
    county: String,
    country: Country,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_trim_fields() {
        let mut m = Merchant {
            merchant_id: "merchant123".into(),
            name: "   name   ".into(),
            premise: "   premise   ".into(),
            street: "   street   ".into(),
            city: "   city   ".into(),
            postcode: "   postcode   ".into(),
            county: "   county   ".into(),
            country: Country::GB,
        };
        let exp = Merchant {
            merchant_id: "merchant123".into(),
            name: "name".into(),
            premise: "premise".into(),
            street: "street".into(),
            city: "city".into(),
            postcode: "postcode".into(),
            county: "county".into(),
            country: Country::GB,
        };
        m.validify();
        assert_eq!(m, exp);
    }
}
