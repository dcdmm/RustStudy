use serde::{Deserialize, Serialize};

// 前端传来的对象,必须实现Deserialize
#[derive(Deserialize)]
pub struct Person {
    name: String,
    age: u32,
}

// 返回给前端的对象,必须实现Serialize;
// rename_all让is_adult在前端变成isAdult
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Greeting {
    message: String,
    is_adult: bool,
}

// 前端invoke("greet", { person: { name, age } })调用,函数名和参数名需一致
#[tauri::command]
pub fn greet(person: Person) -> Result<Greeting, String> {
    if person.name.trim().is_empty() {
        // 返回Err时invoke抛出异常
        return Err("名字不能为空".to_string());
    }
    if person.age > 150 {
        return Err(format!("年龄{}不合理", person.age));
    }
    // 返回Ok时invoke成功
    Ok(Greeting {
        message: format!(
            "你好, {}! 你今年{}岁了! 这个是rust后端生成的问候!",
            person.name, person.age
        ),
        is_adult: person.age >= 18,
    })
}
