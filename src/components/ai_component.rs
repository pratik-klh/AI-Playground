use std::fmt::Debug;

/// Base trait for all AI components in the playground
/// 
/// This trait provides a common interface for all AI-related components
/// such as LLM interfaces, prompt managers, and other AI utilities.
pub trait AIComponent: Debug + Send + Sync {
    /// Initialize the component
    /// 
    /// This method should be called before using the component
    /// to set up any necessary resources or connections.
    fn initialize(&mut self) -> anyhow::Result<()>;
    
    /// Process the component's main functionality
    /// 
    /// This method should implement the main processing logic
    /// for the component.
    fn process(&self) -> anyhow::Result<()>;
    
    /// Get the component name
    fn name(&self) -> &str;
    
    /// Get the component description
    fn description(&self) -> &str;
}

/// Common implementation for components that have name and description fields
pub trait NamedComponent {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
} 