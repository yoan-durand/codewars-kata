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

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<i64>>();
    let mut b1 = b;
    a1.sort();
    b1.sort();

    a1 == b1
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

fn main() {
    assert_eq!(xo("xo"), true);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![];
    let a2 = vec![];
    testing(a1, a2, true);

}
