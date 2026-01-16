// Variant attributes/Field attributes

use serde::{Deserialize, Serialize};

// 还可以应用于enum variant(作用与应用于结构体字段类型类似)
#[test]
fn rename_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        // Serialize and deserialize this field with the given name instead of its Rust name.
        #[serde(rename = "ser_deser_id")] // 设置该字段序列化和反序列化名称ser_deser_name
        id: i32,
        #[serde(rename(serialize = "ser_name"))]
        // 设置该字段序列化名称为ser_name(反序列化名称仍为name)
        name: String,
        #[serde(rename(deserialize = "deser_age"))]
        // 设置该字段反序列化名称为ser_name(序列化名称仍为age)
        age: i32,
        #[serde(rename(serialize = "ser_grade", deserialize = "deser_grade"))]
        // 设置该字段序列化名称为ser_grade,反序列化名称为deser_grade
        grade: char,
    }

    let user = User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        grade: 'A',
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("{}", serialized); // {"ser_deser_id":1,"ser_name":"Alice","age":30,"ser_grade":"A"}

    let data = r#"{ "ser_deser_id": 1, "name": "Alice", "deser_age": 30 , "deser_grade": "B" }"#;
    let deserialized: User = serde_json::from_str(data).unwrap();
    println!("{:?}", deserialized); // print->User { id: 1, name: "Alice", age: 30, grade: 'B' }
}

#[test]
fn default_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        id: i32,
        // If the value is not present when deserializing, use the Default::default().
        #[serde(default)]
        name: String,
        // If the value is not present when deserializing, call a function to get a default value. The given function must be callable as fn() -> T.
        #[serde(default = "default_age")]
        age: i32,
    }

    fn default_age() -> i32 {
        18
    }

    let json_data = [r#"{ "id": 1, "name": "Alice" }"#, r#"{ "id": 2 }"#];

    /*
    print->
    User { id: 1, name: "Alice", age: 18 }
    User { id: 2, name: "", age: 18 }
     */
    for json in json_data.iter() {
        let deserialized: User = serde_json::from_str(json).unwrap();
        println!("{:?}", deserialized);
    }
}

#[test]
fn flatten_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct ExtraInfo {
        email: String,
        phone: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        id: i32,
        name: String,
        // Flatten the contents of this field into the container it is defined in.
        #[serde(flatten)]
        extra: ExtraInfo,
    }

    let user = User {
        id: 1,
        name: String::from("Alice"),
        extra: ExtraInfo {
            email: String::from("alice@example.com"),
            phone: String::from("123-456-7890"),
        },
    };

    // extra字段未添加#[serde(flatten)]时打印为: {"id":1,"name":"Alice","extra":{"email":"alice@example.com","phone":"123-456-7890"}}
    let serialized = serde_json::to_string(&user).unwrap();
    println!("{}", serialized); // print->{"id":1,"name":"Alice","email":"alice@example.com","phone":"123-456-7890"}

    let json_data = r#"{
        "id": 1,
        "name": "Alice",
        "email": "alice@example.com",
        "phone": "123-456-7890"
    }"#;

    let deserialized: User = serde_json::from_str(json_data).unwrap();
    println!("{:?}", deserialized); // print->User { id: 1, name: "Alice", extra: ExtraInfo { email: "alice@example.com", phone: "123-456-7890" } }
}

// 还可以应用于enum variant(作用与应用于结构体字段类型类似)
#[test]
fn skip_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        id: i32,
        /*
        Skip this field: do not serialize or deserialize it.

        When deserializing, Serde will use Default::default() or the function given by default = "..." to get a default value for this field.
         */
        #[serde(skip, default = "default_name")]
        name: String,
        #[serde(skip)]
        debug_info: String,
    }

    fn default_name() -> String {
        String::from("dmm")
    }

    let user = User {
        id: 1,
        name: String::from("Alice"),
        debug_info: String::from("This is debug information that should not be serialized."),
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("{}", serialized); // pirnt->{"id":1}

    let json_data = r#"{"id": 2, "name": "Bob"}"#;
    let deserialized: User = serde_json::from_str(json_data).unwrap();
    println!("{:?}", deserialized); // print->User { id: 2, name: "dmm", debug_info: "" }
}
