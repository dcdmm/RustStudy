// 自定义序列化

/*
Serde's derive macro through #[derive(Serialize, Deserialize)] provides reasonable default serialization behavior for structs and enums and it can be customized to some extent using attributes. For unusual needs, Serde allows full customization of the serialization behavior by manually implementing Serialize and Deserialize traits for your type.
*/

use serde::ser::SerializeSeq;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[test]
fn example0() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 自定义序列化
    impl Serialize for Point {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let formatted = format!("{}:{}", self.x, self.y);
            serializer.serialize_str(&formatted)
        }
    }

    // 自定义反序列化
    impl<'de> Deserialize<'de> for Point {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(PointVisitor)
        }
    }

    struct PointVisitor;

    // Trait serde::de::Visitor: This trait represents a visitor that walks through a deserializer.
    impl<'de> Visitor<'de> for PointVisitor {
        // The value produced by this visitor.
        type Value = Point;

        // Required Methods
        // Format a message stating what data this Visitor expects to receive.
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string in the format 'x:y'")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // 反序列化step 1: "10:20" ==> ["10", "20"]
            let coords: Vec<&str> = value.split(':').collect();

            if coords.len() != 2 {
                return Err(E::custom("expected a string with one colon"));
            }

            // 反序列化step 2: "10" ===> 10
            // 反序列化step 3: "20" ===> 20
            let x = coords[0].parse::<i32>().map_err(de::Error::custom)?;
            let y = coords[1].parse::<i32>().map_err(de::Error::custom)?;

            // 反序列化step 4: 返回Point {x: 10, y: 20}
            Ok(Self::Value { x, y })
        }
    }

    let point = Point { x: 10, y: 20 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("{}", serialized); // print->"10:20"

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized); // print->Point { x: 10, y: 20 }
}