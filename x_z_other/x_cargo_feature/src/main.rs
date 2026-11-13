// 条件编译: 条件编译的代码是指只在特定条件下才会被编译的代码
// 运行方式:
// 1. cargo run -p x_cargo_feature                    -> 不开启display
// 2. cargo run -p x_cargo_feature --features display -> 开启display

use serde::Serialize;

// 需在Cargo.toml中启用serde的derive feature
#[derive(Serialize)]
struct Point {
    x: i32,
    y: i32,
}

// 开启display时才编译这段代码
#[cfg(feature = "display")]
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<<<{}, {}>>>", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };

    // 1. #[cfg]: 条件不成立时,代码直接被删除
    // 下面的println!依赖上面的impl,未开启display时两者一起被删除
    #[cfg(feature = "display")]
    println!("{p}"); // print-><<<1, 2>>>

    // not(...): 条件取反,即未开启display时才编译
    #[cfg(not(feature = "display"))]
    println!("display未开启: {} {}", p.x, p.y); // print->display未开启: 1 2

    // 2. cfg!(): 只返回bool,不会删除代码,两个分支都会参与编译
    if cfg!(feature = "display") {
        println!("cfg!: display已开启"); // 开启display时print->cfg!: display已开启
    } else {
        println!("cfg!: display未开启"); // 未开启display时print->cfg!: display未开启
    }

    // 3. 使用第三方库的条件编译
    println!("{}", serde_json::to_string(&p).unwrap()); // print->{"x":1,"y":2}
}
