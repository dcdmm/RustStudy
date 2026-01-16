// Container attributes
use serde::{Deserialize, Serialize};

#[test]
fn rename_all_test() {
    #[derive(Serialize, Deserialize, Debug)]
    /*
    Rename all the fields (if this is a struct) or variants (if this is an enum) according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE".
    */
    #[serde(rename_all = "UPPERCASE")]
    struct User {
        first_name: String,
        last_name: String,
        email_address: String,
        age: u8,
    }

    let user = User {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email_address: "john.doe@example.com".to_string(),
        age: 30,
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

// 仅适用于枚举
#[test]
fn tag_untag_test() {
    #[derive(Serialize, Deserialize, Debug)]
    // #[serde(tag = "type")]
    #[serde(untagged)]
    enum Event {
        Login { user_id: u32, username: String },
        Logout { user_id: u32 },
        Click { x: i32, y: i32 },
    }

    let login_event = Event::Login {
        user_id: 42,
        username: "alice".to_string(),
    };

    let serialized = serde_json::to_string(&login_event).unwrap();
    /*
    枚举Event未添加#[serde(tag = "type")]时打印为: {"Login":{"user_id":42,"username":"alice"}}
    枚举Event添加#[serde(untagged)]时打印为: {"user_id":42,"username":"alice"}
     */
    println!("{}", serialized); // print->{"type":"Login","user_id":42,"username":"alice"}

    let deserialized: Event = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized); // print->Login { user_id: 42, username: "alice" }
}
