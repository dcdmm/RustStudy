mod lifetime;

fn main() {
    let r;          // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;   // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}