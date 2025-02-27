pub fn mask_pan(pan: &str) -> String {
    mask_number(pan, '#', |i| i < 6 || i >= pan.len().saturating_sub(4))
}

pub fn mask_account_number(num: &str) -> String {
    mask_number(num, '#', |i| i > num.len().saturating_sub(4))
}

fn mask_number(num: &str, ch: char, predicate: impl Fn(usize) -> bool) -> String {
    num.chars()
        .enumerate()
        .map(|(i, c)| if predicate(i) { c } else { ch })
        .collect()
}
