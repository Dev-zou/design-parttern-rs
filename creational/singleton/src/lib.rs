// 单例模式模块入口
// 导出五种单例模式实现
// 注意: 方案1和方案5需要允许static_mut_refs
#![allow(static_mut_refs)]

// 方案1: 基本懒汉式 (非线程安全)
mod singleton1;
pub use singleton1::Singleton1;

// 方案2: 使用lazy_static宏 (线程安全)
mod singleton2;
pub use singleton2::Singleton2;

// 方案3: 使用OnceLock (Rust 1.70+推荐方式)
mod singleton3;
pub use singleton3::Singleton3;

// 方案4: 饿汉式 (线程安全)
mod singleton4;
pub use singleton4::Singleton4;

// 方案5: 使用std::sync::Once (线程安全的延迟初始化)
mod singleton5;
pub use singleton5::Singleton5;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::MutexGuard;

    // 测试方案1
    #[test]
    fn test_singleton1() {
        let instance1 = Singleton1::get_instance();
        assert_eq!(instance1.get_data(), "Singleton1 instance");

        instance1.set_data("Updated data");
        let instance1_again = Singleton1::get_instance();
        assert_eq!(instance1_again.get_data(), "Updated data");
    }

    // 测试方案2
    #[test]
    fn test_singleton2() {
        let mut instance2: MutexGuard<'_, Singleton2> = Singleton2::get_instance();
        assert_eq!(instance2.get_data(), "Singleton2 instance");

        instance2.set_data("Updated data");
        drop(instance2); // 释放锁

        let instance2_again = Singleton2::get_instance();
        assert_eq!(instance2_again.get_data(), "Updated data");
    }

    // 测试方案3
    #[test]
    fn test_singleton3() {
        let instance3 = Singleton3::get_instance();
        assert_eq!(instance3.get_data(), "Singleton3 instance");

        // 注意: 方案3的实例是不可变的
    }

    // 测试方案4
    #[test]
    fn test_singleton4() {
        let instance4 = Singleton4::get_instance();
        assert_eq!(instance4.get_data(), "");
    }

    // 测试方案5
    #[test]
    fn test_singleton5() {
        let instance5 = Singleton5::get_instance();
        assert_eq!(instance5.get_data(), "Singleton5 instance");

        instance5.set_data("Updated data");
        let instance5_again = Singleton5::get_instance();
        assert_eq!(instance5_again.get_data(), "Updated data");
    }

    // 测试线程安全性
    #[test]
    fn test_thread_safety() {
        use std::thread;
        use std::thread::JoinHandle;

        // 测试多种单例的线程安全性
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for i in 0..10 {
            let handle = thread::spawn(move || {
                let data = format!("Thread {} data", i);
                
                // 测试方案2
                let mut instance2 = Singleton2::get_instance();
                instance2.set_data(&data);
                println!("Thread {} set singleton2 data: {}", i, instance2.get_data());
                drop(instance2);
                
                // 测试方案5
                let instance5 = Singleton5::get_instance();
                instance5.set_data(&data);
                println!("Thread {} set singleton5 data: {}", i, instance5.get_data());
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }

        // 验证最后设置的数据
        let final_instance2 = Singleton2::get_instance();
        println!("Final singleton2 data: {}", final_instance2.get_data());
        
        let final_instance5 = Singleton5::get_instance();
        println!("Final singleton5 data: {}", final_instance5.get_data());
    }
}
