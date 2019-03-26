pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<i64>>();
    let mut b1 = b;
    a1.sort();
    b1.sort();

    a1 == b1
}