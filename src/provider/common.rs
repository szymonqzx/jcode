//! Common helper functions shared across providers

use std::sync::RwLock;

/// Helper function to recover from poisoned RwLock with logging
pub fn recover_rwlock_read<T, F>(lock: &RwLock<T>, fallback: F, provider_name: &str, context: &str) -> T
where
    T: Clone,
    F: FnOnce(&T) -> T,
{
    let guard = lock.read().unwrap_or_else(|e| {
        crate::logging::warn(&format!("Recovering from poisoned RwLock in {} provider ({})", provider_name, context));
        e.into_inner()
    });
    fallback(&guard)
}

/// Helper function to recover from poisoned RwLock with logging (write)
/// The closure receives a mutable reference and should perform the assignment directly
pub fn recover_rwlock_write<T, F>(lock: &RwLock<T>, fallback: F, provider_name: &str, context: &str)
where
    F: FnOnce(&mut T),
{
    let mut guard = lock.write().unwrap_or_else(|e| {
        crate::logging::warn(&format!("Recovering from poisoned RwLock in {} provider ({})", provider_name, context));
        e.into_inner()
    });
    fallback(&mut guard);
}
