fn xo(string: &'static str) -> bool {
    let char_array = string.chars();
    let mut count_o = 0;
    let mut count_x = 0;

    for i in char_array {
        if i == 'x' || i == 'X' {
            count_x += 1;
        }
        if i == 'o' || i == 'O' {
            count_o += 1;
        }
    }

    return count_x == count_o;
}

fn main() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
