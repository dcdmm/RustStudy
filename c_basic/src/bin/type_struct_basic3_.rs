// unit-like structs

// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
struct AlwayEqual; // ★★★★★分号结尾
fn main() {
    let _obj = AlwayEqual;
}