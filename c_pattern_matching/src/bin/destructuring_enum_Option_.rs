#![allow(non_snake_case)]

// 解构Option

#[allow(warnings)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // i绑定(复制,i32实现了Copy trait)了任意Option<i32>的Some变体中包含的值
        Some(i) => Some(i + 1),
    }
}

fn add_exclamation(x: Option<String>) -> Option<String> {
    match x {
        None => None,
        // text绑定(移动,String没有实现Copy trait)了任意Option<String>的Some变体中包含的值
        Some(mut text) => {
            text.push('!');
            Some(text)
        }
    }
}

fn main() {
    let five = Some(5);
    // i32实现了Copy trait
    let _six = plus_one(five);
    let _none = plus_one(None);
    println!("five 仍然可以使用: {:?}", five); // Some(5)

    let hello = Some(String::from("hello"));
    // String>没有实现Copy trait
    let _hello_with_exclamation = add_exclamation(hello);
    // // error[E0382]: borrow of moved value: `hello` 
    // println!("{:?}", hello);
}
