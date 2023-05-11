// unit-like structs

// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
struct AlwayEqual; // ★★★★★分号结尾

#[test]
fn t0() {
    let _obj = AlwayEqual;
}