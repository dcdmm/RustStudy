// 模式匹配

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // 解构嵌套的结构体和枚举
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
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        _ => (),
    }

    // *******************************************************************

    // 解构结构体和元组
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}", feet);
    println!("inches: {}", inches);
    println!("Point x:: {}", x);
    println!("Point y:: {}", y);
}
