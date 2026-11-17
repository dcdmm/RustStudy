// 示例3: 状态管理(State)
// 参考文档: https://tauri.app/develop/state-management/

use std::sync::Mutex;

use tauri::State;

// 交给Tauri托管的状态,在lib.rs中通过.manage()注册,整个应用共享一份
// State<'_, T>只能拿到&T,要修改数据需要Mutex(内部可变性 + 线程安全)
#[derive(Default)]
pub struct Counter {
    count: Mutex<u32>,
}

// 业务逻辑写成普通方法,不依赖Tauri,方便单元测试
impl Counter {
    pub fn get(&self) -> u32 {
        *self.count.lock().unwrap()
    }

    pub fn increment(&self) -> u32 {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        *count
    }
}

// command只做一层很薄的适配: 拿到State,再调用Counter的方法
// State参数由Tauri自动注入,前端调用时不需要传
#[tauri::command]
pub fn get_count(counter: State<'_, Counter>) -> u32 {
    counter.get()
}

#[tauri::command]
pub fn increment(counter: State<'_, Counter>) -> u32 {
    counter.increment()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_counter() {
        let counter = Counter::default();
        assert_eq!(counter.get(), 0);
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);
    }
}
