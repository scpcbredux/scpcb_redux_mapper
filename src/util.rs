pub fn seed_from_string(value: &str) -> u64 {
    value
        .chars()
        .enumerate()
        .fold(0, |temp, (index, c)| temp ^ ((c as u64) << (index % 24)))
}
