mod use_keyword0_;
// mod use_keyword1_ {
//     // use_keyword1_.rs文件内容
// }
// 与上等价
mod use_keyword1_;

// mod as_public {
//     // as_public.rs文件内容
// }
// 与上等价
// For a module named `as_public` declared in the crate root, the compiler will look for the module’s code in:
// 方式一: src/as_public.rs
// 方式二: src/as_public/mod.rs
// For a module named `as_public_sub` that is a submodule of `as_public`, the compiler will look for the module’s code in:
// 方式一: src/as_public/as_public_sub.rs
// 方式二: src/as_public/as_public_sub/mod.rs
// 注意:方式一、方式二不能混用
mod as_public;

// module tree remains the same
use use_keyword0_::customer::eat_at_restaurant;
use as_public::enums_public_::eat_at_restaurant as eat_at_restaurant_cat;

// Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.
use rand::Rng; // 第三方库

fn main() {
    eat_at_restaurant();

    eat_at_restaurant_cat();

    let _secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Hello, world!");
}
