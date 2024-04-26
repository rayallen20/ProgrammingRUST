fn main() {
    let s1: String = String::from("s1");
    let s2: String = String::from("s2");
    let s3: String = String::from("s3");
    let s4: String = String::from("s4");

    let v: Vec<String> = vec![s1, s2, s3, s4];
    println!("{:?}", v[4]);
}
