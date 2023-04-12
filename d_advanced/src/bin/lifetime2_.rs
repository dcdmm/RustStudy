// 生命周期(Lifetime Annotations in Method Definitions)

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str, // a reference
}

// 类似方法中泛型的定义
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // 生命周期消除第一规则
    fn level(&self) -> i32 {
        3
    }
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // 生命周期消除第一规则
    // 生命周期消除第三规则
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    
}
