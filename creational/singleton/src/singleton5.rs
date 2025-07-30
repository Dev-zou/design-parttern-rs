// 单例模式实现方案5: 使用std::sync::Once (线程安全的延迟初始化)
use std::sync::Once;
use std::ptr;

pub struct Singleton5 {
    data: String,
}

// 用于确保初始化代码只执行一次
static ONCE: Once = Once::new();
// 存储单例实例的原始指针
static mut INSTANCE5: *mut Singleton5 = ptr::null_mut();

impl Singleton5 {
    // 获取单例实例（可变）
    pub fn get_instance() -> &'static mut Singleton5 {
        unsafe {
            ONCE.call_once(|| {
                // 分配内存并初始化实例
                INSTANCE5 = Box::into_raw(Box::new(Singleton5 {
                    data: "Singleton5 instance".to_string(),
                }));
            });

            // 将原始指针转换为可变引用
            &mut *INSTANCE5
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

// 实现Drop trait以释放内存
impl Drop for Singleton5 {
    fn drop(&mut self) {
        println!("Singleton5 is being dropped");
    }
}