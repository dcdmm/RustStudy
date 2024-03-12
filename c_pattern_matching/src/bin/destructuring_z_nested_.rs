// 解构嵌套项

#[allow(warnings)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(warnings)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(warnings)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[test]
fn t0() {
    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    let msg = Message::Move { x: 8, y: 100 };

    // 解构嵌套的枚举和结构体
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        Message::Move { x: x @ 1..=10, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        _ => (),
    }
}

#[test]
fn t1() {
    // 解构嵌套的结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: -3, y: -10 });
    println!("feet: {}", feet); // print->feet: 3
    println!("inches: {}", inches); // print->inches: 10
    println!("Point x: {}", x); // print->Point x: -3
    println!("Point y: {}", y); // print->Point y: -10
}

fn main() {}
