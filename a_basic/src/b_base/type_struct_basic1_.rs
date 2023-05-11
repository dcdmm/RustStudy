// Creating Instances From Other Instances With Struct Update Syntax

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[test]
fn t0() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // 与上等价
    // 代码更加简洁
    let user2 = User {
        email: String::from("another@example.com"),

        // ..user1必须在结构体的尾部使用
        // ..user1不以分号(;)结尾
        ..user1
    };
    println!("{}", user2.active); // true
    println!("{}", user2.sign_in_count); // 1
    println!("{}", user2.email); // another@example.com
    println!("{}", user2.username); // someusername123

    println!("{}", user1.active); // true
    println!("{}", user1.sign_in_count); // 1
    println!("{}", user1.email); // someone@example.com

    // 报错:borrow of moved value: `user1.username`
    // Note that the struct update syntax uses = like an assignment; this is because it moves the data
    // println!("{}", user1.username);
}
