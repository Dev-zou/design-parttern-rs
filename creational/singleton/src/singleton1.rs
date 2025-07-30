// 单例模式实现方案1: 基本懒汉式 (非线程安全)
// 注意: 这种实现不是线程安全的，仅用于演示
#![allow(static_mut_refs)]

pub struct Singleton1 {
    data: String,
}

// 静态变量存储单例实例
static mut INSTANCE1: Option<Box<Singleton1>> = None;

impl Singleton1 {
    // 获取单例实例
    pub fn get_instance() -> &'static mut Singleton1 {
        unsafe {
            if INSTANCE1.is_none() {
                INSTANCE1 = Some(Box::new(Singleton1 {
                    data: "Singleton1 instance".to_string(),
                }));
            }
            INSTANCE1.as_mut().unwrap()
        }
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