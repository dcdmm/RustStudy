fn main() {
    static FOO: [i32; 5] = [1, 2, 3, 4, 5];

    let r1 = &FOO as *const _;
    let r2 = &FOO as *const _;
// With a strictly read-only static, references will have the same address
    print!("{:?}", r1);
// A static item can be used just like a variable in many cases
    println!("{FOO:?}");
}
