// Raw string literals

fn main() {
    let mm_will_escape = "c:\\now";
    let mm_r = r"c:\now"; // 与上等价(字符串中不能直接包含双引号,与python中类似)
    let mm_r1 = r#"c:\now"#; // 与上等价
    println!("{} {} {}", mm_will_escape, mm_r, mm_r1);

    let json_str_will_escape = "{\"id\": 2, \"name\": \"Bob\"}";
    let json_str_r1 = r#"{"id": 2, "name": "Bob"}"#; // 与上等价(允许字符串中包含双引号)
    println!("{} {}", json_str_will_escape, json_str_r1);

    println!("{}", r#"c:\now"#); // print->c:\now
}
