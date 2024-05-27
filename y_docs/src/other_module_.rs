#![allow(warnings)]

//! # 整个other_module_模块的文档注释
//!
//! 置于other_module_.rs文件代码内容前

/// public_struct模块
pub mod public_struct {
    /// public_struct模块下Breakfast结构体
    ///
    /// 这个结构体必须通过关联函数sumer创建
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::other_module_::public_struct;
    ///
    /// let _ = public_struct::Breakfast::summer("rust");
    /// ```
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 结构体Breakfast文档Fields中不含seasonal_fruit字段(cargo build生成时)
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    /// 字段toast是否以rust开头
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::other_module_::public_struct;
    /// let x = public_struct::Breakfast::summer("rust base");
    /// let x_bool = x.startwiths_rust();
    /// assert_eq!(x_bool, true);
    /// ```
    impl Breakfast {
        pub fn startwiths_rust(&self) -> bool {
            self.toast.starts_with("rust")
        }
    }
}

/// public_enum模块
pub mod public_enum {
    /// public_enum模块下Appetizer枚举
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
