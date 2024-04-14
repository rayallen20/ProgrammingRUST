use num::Complex;

fn main() {
    println!("Hello, world!");
}

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.0;
    loop {
        x = x * x + c;
    }
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex{ re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}

/// 尝试测定`c`是否位于曼德博集中
/// 若`c`不是集合成员之一,则返回`Some(i)`,其中`i`是`c`离开以原点为中心的半径为2的圆时所需的迭代次数
/// 若`c`可能是集合成员之一(达到迭代次数限制时无法证明`c`不是成员),则返回`None`
fn escape_time(c: Complex<f64>, limit: usize) ->Option<usize> {
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}