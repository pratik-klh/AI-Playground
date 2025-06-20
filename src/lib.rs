//! AI Playground - A Rust project for experimenting with AI and LLM APIs
//! 
//! This library provides components for:
//! - LLM interface management
//! - Prompt template management
//! - AI component abstractions

pub mod components;
pub mod playground;

pub use components::{AIComponent, LLMInterface, PromptManager};
pub use playground::AIPlayground;

/// Re-export common types and traits
pub mod prelude {
    pub use crate::components::{AIComponent, LLMInterface, PromptManager};
    pub use crate::playground::AIPlayground;
} 