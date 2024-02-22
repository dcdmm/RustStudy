// unit-like structs

// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 
struct AlwayEqual; // 没有任意字段,且以分号结尾

fn main() {
    let _obj = AlwayEqual{};
    let _obj1 = AlwayEqual; // 与上等价且代码更加简洁
}