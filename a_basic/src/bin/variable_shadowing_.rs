// 变量遮蔽(shadowing)

fn main() {
    // binds x to a value of 5
    let x = 5;
    // it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6.
    let x = x + 1; // 变量遮蔽
    {
        // within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    //  When that scope is over, the inner shadowing ends and x returns to being 6
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len(); // 变量遮蔽
    println!("{}", spaces);

    let spaces_mut = "   "; // 类型:`&str`
    // 报错:expected `&str`, found `usize`
    // spaces_mut = spaces_mut.len()  // 类型: `usize`
    println!("{}", spaces_mut)
}
