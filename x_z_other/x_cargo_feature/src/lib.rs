use serde::Serialize;

// 需在Cargo.toml中启用serde的derive feature
#[derive(Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// 开启display时才编译这段代码
#[cfg(feature = "display")]
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<<<{}, {}>>>", self.x, self.y)
    }
}
