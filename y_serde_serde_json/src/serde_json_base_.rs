use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[test]
fn json_from_str() -> Result<(), serde_json::Error> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    // Deserialize an instance of type `T` from a string of JSON text.
    let v: Value = serde_json::from_str(data)?;
    println!("{}", v["name"]);
    println!("{}", v["phones"]);

    /*
    Serde provides a powerful way of mapping JSON data into Rust data structures largely automatically.

    This is the same serde_json::from_str function as before, but this time we assign the return value to a variable of type Person so Serde will automatically interpret the input data as a Person and produce informative error messages if the layout does not conform to what a Person is expected to look like.

    Any type that implements Serde’s Deserialize trait can be deserialized this way. This includes built-in Rust standard library types like Vec<T> and HashMap<K, V>, as well as any structs or enums annotated with #[derive(Deserialize)]
     */
    let p: Person = serde_json::from_str(data)?;
    println!("{:?}", p); // print->Person { name: "John Doe", age: 43, phones: ["+44 1234567", "+44 2345678"] }

    Ok(())
}

#[test]
fn json_to_string() {
    let full_name = "John Doe";
    let age_last_year = 42;

    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": full_name,
        "age": age_last_year,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("{}", john["name"]);
    println!("{}", john["phones"]);

    // The Value::to_string() function converts a serde_json::Value into a String of JSON text.
    println!("{}", john.to_string()); // print->{"age":42,"name":"John Doe","phones":["+44 1234567","+44 2345678"]}
}

#[test]
fn serializing_data_to_string() -> Result<(), serde_json::Error> {
    #[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
    }

    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Any type that implements Serde’s Serialize trait can be serialized this way. This includes built-in Rust standard library types like Vec<T> and HashMap<K, V>, as well as any structs or enums annotated with #[derive(Serialize)].
    let j = serde_json::to_string(&address)?;
    println!("{}", j); // print->{"street":"10 Downing Street","city":"London"}

    let v = vec![1, 2, 3, 4];
    let vs = serde_json::to_string(&v)?;
    println!("{}", vs); // print->[1,2,3,4]

    Ok(())
}
