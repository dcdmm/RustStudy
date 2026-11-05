//! # 文档注释(描述整个other_module_模块)
//!
//! 置于other_module_.rs文件顶部

/// public_struct模块
pub mod public_struct {
    /// public_struct模块下Breakfast结构体
    ///
    /// 这个结构体必须通过关联函数summer创建
    ///
    /// # Examples
    ///
    /// ```rust
    /// use x_docs::other_module_::public_struct;
    ///
    /// let _ = public_struct::Breakfast::summer("rust");
    /// ```
    pub struct Breakfast {
        pub toast: String,
        // cargo doc默认生成的Breakfast API文档Fields列表不显示私有字段
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    impl Breakfast {
        /// 字段toast是否以rust开头
        ///
        /// # Examples
        ///
        /// ```rust
        /// use x_docs::other_module_::public_struct;
        /// let x = public_struct::Breakfast::summer("rust base");
        /// let x_bool = x.startwiths_rust();
        /// assert_eq!(x_bool, true);
        /// ```
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
