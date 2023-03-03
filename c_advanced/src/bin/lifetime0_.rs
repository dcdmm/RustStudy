// 生命周期

// Lifetime Annotations in Function Signatures:
// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
// The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. 
// In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. 
// Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. 
// Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
// In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. 
// Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // **************************************************************
    let string11 = String::from("long string is long");

    {
        let string21 = String::from("xyz");
        let result1 = longest(string11.as_str(), string21.as_str());
        println!("The longest string is {}", result1);
    }

    // **************************************************************
    // let string12 = String::from("long string is long");
    // let result2;
    // {
    //     let string22 = String::from("xyz");
    //     result2 = longest(string12.as_str(), string22.as_str());
    // }
    // println!("The longest string is {}", result2);
    // 程序运行结果:
    // error[E0597]: `string22` does not live long enough
    // --> c_advanced\src\bin\lifetime0_.rs:41:46
    // |
    // 41 |         result2 = longest(string12.as_str(), string22.as_str()); // 报错:error[E0597]: `string22` does not live long enough
    // |                                              ^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
    // 42 |     }
    // |     - `string22` dropped here while still borrowed
    // 43 |     println!("The longest string is {}", result2);
    // |                                          ------- borrow later used here
}