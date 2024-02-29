// trait

#[allow(warnings)]
trait Summary {
    fn new() -> Self; // 未实现

    fn summarize(&self) -> String; // 未实现
}

#[allow(warnings)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
}

/*
Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.
 */
// 必须实现trait中的所有未实现的方法/关联函数
impl Summary for Tweet {
    fn new() -> Self {
        Self {
            username: String::from("dmm"),
            content: String::from("i love you!"),
            reply: true,
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let xiaowang = Tweet {
        username: String::from("xiaowang"),
        content: String::from("who are you?"),
        reply: false,
    };
    println!("{}", xiaowang.summarize());

    let dmm = Tweet::new();
    println!("{}", dmm.summarize()); 
}

/*
上述功能c++实现:
#include <iostream>
#include <string>

struct Summary {
    // 基类指针可以在不进行显式类型转换的情况下指向派生类对象
    virtual Summary *new_() = 0;

    virtual string summarize() = 0;
};


struct Tweet : Summary {
    string username;
    string content;
    bool reply;

    Tweet() = default;

    Tweet(string username_, string content_, bool reply_) : username(std::move(username_)),
                                                            content(std::move(content_)), reply(reply_) {}

    string summarize() override { return username + ": " + content; }

    Summary *new_() override { return new Tweet("dmm", "i love you!", true); }
};


int main() {
    auto xiaoming = Tweet{"xiaowang", "who are you?", false};
    std::cout << xiaoming.summarize() << std::endl;

    auto dmm = Tweet().new_();
    std::cout << dmm->summarize() << std::endl;
}
*/
