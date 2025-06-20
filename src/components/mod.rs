//! AI Components module
//! 
//! This module contains the core AI components including:
//! - Base AIComponent trait
//! - LLMInterface for API interactions
//! - PromptManager for template management

pub mod ai_component;
pub mod llm_interface;
pub mod prompt_manager;

pub use ai_component::{AIComponent, NamedComponent};
pub use llm_interface::LLMInterface;
pub use prompt_manager::PromptManager; 