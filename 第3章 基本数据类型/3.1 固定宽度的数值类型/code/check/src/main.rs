fn main() {
    // 10与20的和 可以表示为u8类型
    assert_eq!(10_u8.checked_add(20), Some(30));

    // 100与200的和 不能表示为u8
    assert_eq!(100_u8.checked_add(200), None);

    // 带符号的n位类型,可以表示为-2^(n-1),但无法表示为2^(n-1)
    assert_eq!((-128_i8).checked_div(-1), None)
}
