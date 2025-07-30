// 单例模式实现方案2: 使用lazy_static宏 (线程安全)
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Singleton2 {
    data: String,
}

lazy_static! {
    static ref INSTANCE2: Mutex<Singleton2> = Mutex::new(Singleton2 {
        data: "Singleton2 instance".to_string(),
    });
}

impl Singleton2 {
    // 获取单例实例
    pub fn get_instance() -> std::sync::MutexGuard<'static, Singleton2> {
        INSTANCE2.lock().unwrap()
    }

    // 设置数据
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }

    // 获取数据
    pub fn get_data(&self) -> &str {
        &self.data
    }
}