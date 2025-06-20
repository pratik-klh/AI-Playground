use crate::components::{AIComponent, NamedComponent};
use anyhow::Result;
use tracing::{info, warn};
use std::collections::HashMap;

/// Manages prompt templates and prompt processing
/// 
/// This struct provides functionality for creating, storing, and
/// managing prompt templates for LLM interactions.
#[derive(Debug)]
pub struct PromptManager {
    name: String,
    description: String,
    prompt_templates: Vec<String>,
    current_prompt: Option<String>,
    template_variables: HashMap<String, String>,
}

impl PromptManager {
    /// Create a new Prompt Manager with default templates
    pub fn new() -> Self {
        let prompt_templates = vec![
            "Explain {topic} in simple terms".to_string(),
            "Write a {style} story about {subject}".to_string(),
            "Analyze the following: {content}".to_string(),
            "Generate code for {language} to {task}".to_string(),
            "Summarize the key points of {text}".to_string(),
            "Translate {text} to {language}".to_string(),
            "Create a {type} plan for {goal}".to_string(),
            "Debug this {language} code: {code}".to_string(),
        ];
        
        Self {
            name: "Prompt Manager".to_string(),
            description: "Manages and templates prompts".to_string(),
            prompt_templates,
            current_prompt: None,
            template_variables: HashMap::new(),
        }
    }
    
    /// Set the current prompt
    pub fn set_prompt(&mut self, prompt: String) {
        self.current_prompt = Some(prompt);
    }
    
    /// Get a prompt template by index
    pub fn get_template(&self, index: usize) -> Option<&str> {
        self.prompt_templates.get(index).map(|s| s.as_str())
    }
    
    /// Add a new prompt template
    pub fn add_template(&mut self, template: String) -> Result<()> {
        if template.trim().is_empty() {
            warn!("Cannot add empty template");
            return Err(anyhow::anyhow!("Template cannot be empty"));
        }
        
        self.prompt_templates.push(template.clone());
        info!("Added template: {}", template);
        Ok(())
    }
    
    /// Get the current prompt
    pub fn current_prompt(&self) -> Option<&str> {
        self.current_prompt.as_deref()
    }
    
    /// Get the number of available templates
    pub fn template_count(&self) -> usize {
        self.prompt_templates.len()
    }
    
    /// Get all available templates
    pub fn all_templates(&self) -> &[String] {
        &self.prompt_templates
    }
    
    /// Set a template variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.template_variables.insert(key, value);
    }
    
    /// Process a template with variables
    pub fn process_template(&self, template: &str) -> String {
        let mut result = template.to_string();
        
        for (key, value) in &self.template_variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
    
    /// Get a processed template by index
    pub fn get_processed_template(&self, index: usize) -> Option<String> {
        self.get_template(index)
            .map(|template| self.process_template(template))
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NamedComponent for PromptManager {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_description(&self) -> &str {
        &self.description
    }
}

impl AIComponent for PromptManager {
    fn initialize(&mut self) -> Result<()> {
        info!("Initializing Prompt Manager with {} templates", self.prompt_templates.len());
        
        // TODO: Load templates from file if available
        // TODO: Validate template syntax
        // TODO: Set up template caching
        
        info!("Prompt Manager initialized successfully");
        Ok(())
    }
    
    fn process(&self) -> Result<()> {
        match &self.current_prompt {
            Some(prompt) => {
                info!("Current prompt: {}", prompt);
                // TODO: Process template variables
                // TODO: Apply prompt transformations
                // TODO: Validate prompt length and content
                Ok(())
            }
            None => {
                warn!("No current prompt set");
                Err(anyhow::anyhow!("No current prompt"))
            }
        }
    }
    
    fn name(&self) -> &str {
        self.get_name()
    }
    
    fn description(&self) -> &str {
        self.get_description()
    }
} 