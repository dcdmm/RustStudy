// 语句和表达式

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. 
fn main() {
    let _y = 6; // 语句
    // Function definitions are also statements;
    
    // 报错:error: expected expression, found statement (`let`)
    // Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    // let x = (let y = 6);

    let z = {
        let x = 3;
        // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 
        x + 1  // 表达式(不以分号结尾)
    };
    println!("The value of_z is {z}");
}