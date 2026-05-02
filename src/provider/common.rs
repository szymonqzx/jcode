//! Common helper functions shared across providers

use std::sync::RwLock;

/// Helper function to recover from poisoned RwLock with logging
pub fn recover_rwlock_read<T, F>(lock: &RwLock<T>, fallback: F, provider_name: &str, context: &str) -> T
where
    F: FnOnce(&T) -> T,
{
    lock.read().unwrap_or_else(|e| {
        crate::logging::warn(&format!("Recovering from poisoned RwLock in {} provider ({})", provider_name, context));
        let guard = e.into_inner();
        fallback(&guard)
    }).clone()
}

/// Helper function to recover from poisoned RwLock with logging (write)
pub fn recover_rwlock_write<T, F>(lock: &RwLock<T>, fallback: F, provider_name: &str, context: &str) -> T
where
    F: FnOnce(&T) -> T,
{
    lock.write().unwrap_or_else(|e| {
        crate::logging::warn(&format!("Recovering from poisoned RwLock in {} provider ({})", provider_name, context));
        let guard = e.into_inner();
        fallback(&guard)
    })
}
