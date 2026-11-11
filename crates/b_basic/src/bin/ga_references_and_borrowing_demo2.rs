// 引用与借用实战

// # std::mem::take
// Replaces dest with the default value of T, returning the previous dest value.

// # std::mem::replace
// Moves src into the referenced dest, returning the previous dest value.

// # Option::take
// Takes the value out of the option, leaving a None in its place.

use std::mem;

#[test]
fn t0() {
    // 从&mut引用中取出值: 拿走值的同时,在原位置放入一个新值

    struct Player {
        name: String,
        weapon: Option<String>,
        bag: Vec<String>,
    }

    impl Player {
        fn drop_weapon(&mut self) -> Option<String> {
            // // error[E0507]: cannot move out of `self.weapon` which is behind a mutable reference
            // self.weapon
            self.weapon.take() // 原位置留下None
        }

        fn empty_bag(&mut self) -> Vec<String> {
            mem::take(&mut self.bag) // 原位置留下默认值(空Vec),要求类型实现Default
        }

        fn rename(&mut self, new_name: String) -> String {
            mem::replace(&mut self.name, new_name) // 原位置放入指定的值
        }
    }

    let mut p = Player {
        name: String::from("tom"),
        weapon: Some(String::from("sword")),
        bag: vec![String::from("apple")],
    };

    assert_eq!(p.drop_weapon(), Some(String::from("sword")));
    assert_eq!(p.weapon, None);

    assert_eq!(p.empty_bag(), vec![String::from("apple")]);
    assert!(p.bag.is_empty());

    assert_eq!(p.rename(String::from("jerry")), "tom");
    assert_eq!(p.name, "jerry");
}



fn main() {}
