use ai_playground::prelude::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("=== AI Playground Basic Usage Example ===");
    
    // Create components
    let mut llm = LLMInterface::new(Some("gpt-3.5-turbo".to_string()));
    let mut pm = PromptManager::new();
    
    // Initialize components
    llm.initialize()?;
    pm.initialize()?;
    
    // Set API key (you would normally get this from environment)
    llm.set_api_key("your-api-key-here".to_string());
    
    // Add a custom template
    pm.add_template("Write a {style} poem about {topic}".to_string())?;
    
    // Set template variables
    pm.set_variable("style".to_string(), "haiku".to_string());
    pm.set_variable("topic".to_string(), "artificial intelligence".to_string());
    
    // Get a processed template
    if let Some(processed_template) = pm.get_processed_template(8) { // Index 8 is our new template
        println!("Processed template: {}", processed_template);
        
        // Generate response (this will be a mock response in current implementation)
        match llm.generate_response(&processed_template).await {
            Ok(response) => println!("LLM Response: {}", response),
            Err(e) => println!("Error: {}", e),
        }
    }
    
    // List all templates
    println!("\nAll available templates:");
    for (i, template) in pm.all_templates().iter().enumerate() {
        println!("{}: {}", i, template);
    }
    
    println!("\nExample completed successfully!");
    Ok(())
} 