// 条件编译: 通过feature控制代码是否参与编译
// 运行方式:
// 1. cargo run -p x_cargo_feature                    -> 不开启display
// 2. cargo run -p x_cargo_feature --features display -> 开启display

struct Point {
    x: i32,
    y: i32,
}

// 开启display时才编译这段代码
#[cfg(feature = "display")]
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };

    // 未开启display时,上面的impl被删除,这里若直接使用{p}会报错:
    // error[E0277]: `Point` doesn't implement `std::fmt::Display`
    #[cfg(feature = "display")]
    println!("{p}"); // print->(1, 2)

    // not(...): 条件取反,即未开启display时才编译
    #[cfg(not(feature = "display"))]
    println!("display未开启: {} {}", p.x, p.y); // print->display未开启: 1 2
}
