// src/lib.rs

// module declarations
pub mod domain;
pub mod error;
pub mod firecrawl_client;
pub mod openrouter_client;
pub mod source;

// re-exports
pub use domain::*;
pub use error::*;
pub use firecrawl_client::*;
pub use openrouter_client::*;
