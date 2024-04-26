fn main() {
    for number in 1..4 {
        println!("{}!", number);
    }

    for number2 in (1..4).rev() {
        println!("{}!", number2);
    }
}
