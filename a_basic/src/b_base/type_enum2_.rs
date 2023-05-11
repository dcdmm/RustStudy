// 枚举

// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum
// Listing 1:
#[allow(dead_code)]
enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    // Move has named fields like a struct does.
    Write(String),              // Write includes a single String.
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values.
}

// The following structs could hold the same data that the preceding enum variants hold:
#[allow(dead_code)]
struct QuitMessage; // unit struct
#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
struct WriteMessage(String); // tuple struct
#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 1, which is a single type.
#[test]
fn t0() {
    let _m1 = Message::Quit;
    let _m2 = Message::Move { x: 2, y: 3 };
    let _m3 = Message::Write(String::from(""));
    let _m4 = Message::ChangeColor(1, 2, 3);
}
