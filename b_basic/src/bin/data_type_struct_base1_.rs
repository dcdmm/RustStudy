// Creating Instances From Other Instances With Struct Update Syntax

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
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

    // 与上等价且代码更加简洁
    let user2 = User {
        email: String::from("another@example.com"),
        
        // 必须在结构体的尾部使用且不以分号(;)结尾
        ..user1 // 其余字段值从user1(必须与user2具有相同的结构体类型)对应字段移动或复制
    };
    println!("{}", user2.active); // true
    println!("{}", user2.sign_in_count); // 1
    println!("{}", user2.email); // another@example.com
    println!("{}", user2.username); // someusername123

    println!("{}", user1.active); // true
    println!("{}", user1.sign_in_count); // 1
    println!("{}", user1.email); // someone@example.com
    // error[E0382]: borrow of moved value: `user1.username`
    // note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
    // println!("{}", user1.username);
}
