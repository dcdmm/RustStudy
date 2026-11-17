// 所有command按功能拆分到子模块中
// 子模块中的command需要声明为pub,lib.rs才能通过commands::xxx::yyy注册
pub mod countdown;
pub mod counter;
pub mod cpu;
pub mod greet;
pub mod io;
pub mod math;
pub mod system;
