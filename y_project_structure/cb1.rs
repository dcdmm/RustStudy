use y_project_structure::base1::outer_mod::inner_mod::pub_mod_visible_fn; // pub修饰
// error[E0603]: function `crate_visible_fn` is private
// use y_project_structure::base1::outer_mod::inner_mod::crate_visible_fn; // pub(crate)修饰

fn main() {
    pub_mod_visible_fn();
    println!("Hello, world!");
}