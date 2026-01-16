// 参考文档: https://serde.rs/data-model.html

use serde::{Deserialize, Serialize};

// 序列化trait: Serialize
// 反序列化trait: Deserialize
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn t0() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized); // print->serialized = {"x":1,"y":2}

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap(); // 必须显式指定类型反序列化后的类型

    println!("deserialized = {:?}", deserialized); // print->deserialized = Point { x: 1, y: 2 }
}
