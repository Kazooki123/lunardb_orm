//! This library provides a unified interface for database operations across different
//! storage paradigms while maintaining high performance through intelligent caching.

mod core;
mod sql;
mod nosql; 
mod cache;

// Re-export main components for easier access
pub use crate::core::LunarDB;
pub use crate::sql::{Query, Schema, Table, Column};
pub use crate::nosql::{Document, Collection};
pub use crate::cache::CacheStrategy;

// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::LunarDB;
    pub use crate::sql::{Query, Schema, Table, Column};
    pub use crate::nosql::{Document, Collection};
    pub use crate::cache::CacheStrategy;
    pub use crate::core::{Result, Error};
}
