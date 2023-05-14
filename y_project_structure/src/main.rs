// For a module named `as_public` declared in the crate root, the compiler will look for the module’s code in:
// 方式一: src/as_public.rs
// 方式二: src/as_public/mod.rs
//
// For a module named `as_public_sub` that is a submodule of `as_public`, the compiler will look for the module’s code in:
// 方式一: src/as_public/as_public_sub.rs
// 方式二: src/as_public/as_public_sub/mod.rs
// 注意:方式一、方式二不能混用

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
mod as_public;

mod use_keyword2_;

fn main() {
    use use_keyword0_::customer::eat_at_restaurant;
    use as_public::enums_public_::eat_at_restaurant as eat_at_restaurant_;

    eat_at_restaurant_();
    eat_at_restaurant();

    #[allow(unused)]
    use use_keyword2_::hosting;
}