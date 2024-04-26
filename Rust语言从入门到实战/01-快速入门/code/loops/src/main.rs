fn main() {
    let mut counter = 0;

    // result用于接收从loop中返回的值
    let result = loop {
        counter += 1;

        if counter == 10 {
            // 跳出循环时 从循环中返回一个值
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
