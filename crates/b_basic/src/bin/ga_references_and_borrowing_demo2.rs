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

#[test]
fn t1() {
    // 字段拆分借用: 编译器能区分同一个struct的不同字段,但不会分析方法内部用了哪些字段

    struct Game {
        players: Vec<String>,
        log: Vec<String>,
    }

    impl Game {
        fn add_log(&mut self, msg: String) {
            self.log.push(msg);
        }

        fn start(&mut self) {
            for p in &self.players {
                // // error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
                // self.add_log(format!("{p} joined")); // 方法借用的是整个self,与&self.players冲突

                self.log.push(format!("{p} joined")); // 直接访问字段,只借用self.log,与&self.players不冲突
            }
            self.add_log(String::from("game start")); // 循环结束后&self.players已失效,可以调用方法
        }
    }

    let mut g = Game {
        players: vec![String::from("tom"), String::from("jerry")],
        log: vec![],
    };
    g.start();
    assert_eq!(g.log, vec!["tom joined", "jerry joined", "game start"]);
}

fn main() {}
