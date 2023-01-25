// 结构体的定义与创建实例(类似元组)

// User struct definition
#[allow(dead_code)]
struct User {
    // 拥有4个字段
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user_complex(email: String, username: String) -> User {
    User {
        // parameter names and the struct field names are exactly the same
        username: username,
        email: email,

        active: true,
        sign_in_count: 1,
    }
}

// 与上等价
// 代码更加简洁
#[allow(dead_code)]
fn build_user_sample(email: String, username: String) -> User {
    User {
        username,  // 等价于:username: username 
        email,  // email: email

        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // Creating an instance of the User struct
    /*
    每个字段都需要进行初始化
    初始化时的字段顺序不需要和结构体定义时的顺序一致

    Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
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
    user1.email = String::from("anotheremail@example.com"); // user1必须为可变变量
    println!("{}", user1.email);
}
