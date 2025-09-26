//! Time-related operations.

use core::sync::atomic::{AtomicU64, Ordering};

/// 全局变量，用于记录计时器中断产生的ticks值
pub static GLOBAL_TICKS: AtomicU64 = AtomicU64::new(0);

/// 递增全局ticks计数器
pub fn increment_global_ticks() {
    GLOBAL_TICKS.fetch_add(1, Ordering::Relaxed);
}

#[cfg(feature = "irq")]
pub use axplat::time::set_oneshot_timer;
pub use axplat::time::{
    Duration, MICROS_PER_SEC, MILLIS_PER_SEC, NANOS_PER_MICROS, NANOS_PER_MILLIS, NANOS_PER_SEC,
    TimeValue, busy_wait, busy_wait_until, current_ticks, epochoffset_nanos, monotonic_time,
    monotonic_time_nanos, nanos_to_ticks, ticks_to_nanos, wall_time, wall_time_nanos,
};