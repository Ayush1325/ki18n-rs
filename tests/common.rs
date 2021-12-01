use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

/// There can only be one thread running at the time with a QQuickEngine
/// (in principle, everything should be in the same main thread)
pub fn lock_for_test() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}
