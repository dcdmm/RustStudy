// Lifetime Annotations in Method Definitions

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str, // a reference
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> { // 类似方法中泛型的定义
    // ======>applies the first rule
    fn level(&self) -> i32 {
        3
    }
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // ======>applies the first rule
    // ======>applies the third rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
