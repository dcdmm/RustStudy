use std::sync::Mutex;

use tauri::State;

#[derive(Default)]
pub struct Counter {
    count: Mutex<u32>,
}

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

#[tauri::command]
pub fn get_count(counter: State<'_, Counter>) -> u32 {
    counter.get()
}

#[tauri::command]
pub fn increment(counter: State<'_, Counter>) -> u32 {
    counter.increment()
}
