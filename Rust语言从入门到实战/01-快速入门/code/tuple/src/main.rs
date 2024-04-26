fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five, six_point_four, one);
}
