fn main() {
    let g = gcd(15, 5);
    println!("gcd(15, 5) = {}", g);
}

// 本函数用于计算两个数的最大公约数
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}