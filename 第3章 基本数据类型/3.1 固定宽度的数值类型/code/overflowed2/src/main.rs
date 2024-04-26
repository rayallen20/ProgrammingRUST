fn main() {
    // 左移17位 溢出 实际上左移的位数是 17 % 16 = 1
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}
