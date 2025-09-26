#![no_std]
#![no_main]

extern crate axplat_aarch64_dyn;

#[macro_use]
extern crate axstd as std;

use std::os::arceos::modules::axhal;
use core::sync::atomic::Ordering;

#[unsafe(no_mangle)]
fn main() {
    println!("Hello, world!");
    
    // 周期性获取并显示ticks值
    println!("开始周期性获取ticks值...");
    
    // 设置循环次数或运行时间
    const ITERATIONS: usize = 5; // 运行5次
    const INTERVAL_MS: u64 = 1000; // 每秒获取一次
    
    for i in 0..ITERATIONS {
        // 获取当前ticks值
        let current_ticks = axhal::time::GLOBAL_TICKS.load(Ordering::Relaxed);
        
        // 显示ticks值
        println!("迭代 {}: 当前ticks值 = {}", i + 1, current_ticks);
        
        // 等待指定时间
        axhal::time::busy_wait(std::time::Duration::from_millis(INTERVAL_MS));
    }
    
    // 显示最终的ticks值
    let final_ticks = axhal::time::GLOBAL_TICKS.load(Ordering::Relaxed);
    println!("程序结束，最终ticks值 = {}", final_ticks);
}
