#[macro_export] // 标有#[macro_export]的宏自动成为公共的
macro_rules! export_ {
    () => {
        println!("print export!")
    };
}

mod macro_export1_;
