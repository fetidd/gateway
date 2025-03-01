pub fn mask_pan(pan: &str) -> String {
    mask_number(pan, '#', |i| i < 6 || i >= pan.len().saturating_sub(4))
}

pub fn mask_account_number(num: &str) -> String {
    mask_number(num, '#', |i| i >= num.len().saturating_sub(4))
}

fn mask_number(num: &str, ch: char, predicate: impl Fn(usize) -> bool) -> String {
    num.chars()
        .enumerate()
        .map(|(i, c)| if predicate(i) { c } else { ch })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1234123412341234", "123412######1234")]
    #[case("12341234123412341234", "123412##########1234")]
    #[case("1234123412", "1234123412")]
    #[case("123412", "123412")]
    fn test_mask_pan(#[case] num: &str, #[case] exp: &str) {
        assert_eq!(mask_pan(num), exp);
    }

    #[rstest]
    #[case("12341234", "####1234")]
    #[case("123412341234", "########1234")]
    #[case("1234", "1234")]
    #[case("12", "12")]
    fn test_mask_account_number(#[case] num: &str, #[case] exp: &str) {
        assert_eq!(mask_account_number(num), exp);
    }
}
