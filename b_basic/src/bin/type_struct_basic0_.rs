// 结构体的定义与创建实例(类似元组)

// 结构体的定义
#[allow(warnings)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(warnings)]
fn build_user_sample(email: String, username: String) -> User {
    User {
        // username: username,
        username,  // 与上等价且代码更加简洁(参数名称与结构体字段名称同名时)
        email,

        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    /*
    创建结构体实例

    每个字段都需要进行初始化
    初始化时的字段顺序不需要和结构体定义时顺序一致    
    */
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // To get a specific value from a struct, we use dot notation.
    println!("{}", user1.username);
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field
    user1.email = String::from("anotheremail@example.com"); // user1必须为可变变量(Rust不支持仅将某个字段标记为可变)
    println!("{}", user1.email);

    #[allow(warnings)]
    let user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // error[E0594]: cannot assign to `user2.email`, as `user2` is not declared as mutable
    // user2.email = String::from("anotheremail@example.com");
}
