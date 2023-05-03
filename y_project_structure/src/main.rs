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
// 文件夹作为一个模块(注意:方式1、方式2不能混用):
// 方式1. 该文件夹同级目录下创建一个与目录同名的rs文件(推荐)
// 方式2. 该文件夹目录中创建一个mod.rs文件
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
