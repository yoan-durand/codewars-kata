pub fn find_short(s: &str) -> u32 {
    let mut min_len: usize = 10000000;
    let split_str = s.split_whitespace();

    for item in split_str {
        if item.len() < min_len {
            min_len = item.len();
        }
    }

    min_len as u32
}