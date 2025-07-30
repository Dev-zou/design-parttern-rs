// 单例模式实现方案3: 使用OnceLock (Rust 1.70+推荐方式)
use std::sync::OnceLock;

pub struct Singleton3 {
    data: String,
}

static INSTANCE3: OnceLock<Singleton3> = OnceLock::new();

impl Singleton3 {
    // 获取单例实例
    pub fn get_instance() -> &'static Singleton3 {
        INSTANCE3.get_or_init(|| Singleton3 {
            data: "Singleton3 instance".to_string(),
        })
    }

    // 获取数据
    pub fn get_data(&self) -> &str {
        &self.data
    }
}