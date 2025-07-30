// 单例模式实现方案4: 饿汉式 (线程安全)
// 特点: 程序启动时就初始化，天然线程安全

pub struct Singleton4 {
    data: String,
}

// 静态变量存储单例实例，程序启动时就初始化
static INSTANCE4: Singleton4 = Singleton4 {
    data: String::new(),
};

impl Singleton4 {
    // 获取单例实例
    pub fn get_instance() -> &'static Singleton4 {
        &INSTANCE4
    }

    // 初始化数据
    pub fn init(data: &str) -> &'static Singleton4 {
        let _ = data;
        // 注意: 由于实例是不可变的，这里实际上是返回已初始化的实例
        // 饿汉式在程序启动时就已初始化，这里的init只是设置数据的一种方式
        // 但由于实例是不可变的，这种方式在Rust中并不适用
        // 以下是演示代码，实际使用中应在创建时就设置好数据
        &INSTANCE4
    }

    // 获取数据
    pub fn get_data(&self) -> &str {
        &self.data
    }
}