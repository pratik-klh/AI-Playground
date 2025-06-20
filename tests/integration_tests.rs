use ai_playground::prelude::*;
use anyhow::Result;

#[tokio::test]
async fn test_llm_interface_initialization() -> Result<()> {
    let mut llm = LLMInterface::new(Some("gpt-3.5-turbo".to_string()));
    
    // Test initialization
    assert!(llm.initialize().is_ok());
    assert!(llm.is_connected());
    assert_eq!(llm.model_name(), "gpt-3.5-turbo");
    
    Ok(())
}

#[tokio::test]
async fn test_prompt_manager_functionality() -> Result<()> {
    let mut pm = PromptManager::new();
    
    // Test initialization
    assert!(pm.initialize().is_ok());
    
    // Test template count
    assert_eq!(pm.template_count(), 8);
    
    // Test adding template
    assert!(pm.add_template("Test template {variable}".to_string()).is_ok());
    assert_eq!(pm.template_count(), 9);
    
    // Test template retrieval
    assert_eq!(pm.get_template(0), Some("Explain {topic} in simple terms"));
    
    // Test variable substitution
    pm.set_variable("variable".to_string(), "value".to_string());
    let processed = pm.get_processed_template(8).unwrap();
    assert_eq!(processed, "Test template value");
    
    Ok(())
}

#[tokio::test]
async fn test_ai_playground_integration() -> Result<()> {
    let mut playground = AIPlayground::new();
    
    // Test initialization
    assert!(playground.initialize().is_ok());
    
    // Test component names
    assert_eq!(playground.llm_interface.name(), "LLM Interface");
    assert_eq!(playground.prompt_manager.name(), "Prompt Manager");
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let mut pm = PromptManager::new();
    
    // Test adding empty template
    assert!(pm.add_template("".to_string()).is_err());
    
    let llm = LLMInterface::new(None);
    
    // Test generating response without initialization
    assert!(llm.generate_response("test").await.is_err());
    
    Ok(())
} 