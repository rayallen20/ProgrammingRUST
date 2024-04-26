fn main() {
    let a: u32 = 10;

    fn add_v1(x: u32) ->u32 {
            x + a
    }

    let add_v2 = |x: u32| -> u32 {
        x + a
    };

    let result1 = add_v1(10);
    let result2 = add_v2(10);

    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}
