use crate::components::{AIComponent, LLMInterface, PromptManager};
use anyhow::Result;
use std::io::{self, Write};
use tracing::{info, error};

/// Main AI Playground class that orchestrates all components
pub struct AIPlayground {
    pub llm_interface: LLMInterface,
    pub prompt_manager: PromptManager,
}

impl AIPlayground {
    /// Create a new AI Playground instance
    pub fn new() -> Self {
        Self {
            llm_interface: LLMInterface::new(None),
            prompt_manager: PromptManager::new(),
        }
    }
    
    /// Initialize all components
    pub fn initialize(&mut self) -> Result<()> {
        info!("=== AI Playground Initialization ===");
        
        self.llm_interface.initialize()?;
        self.prompt_manager.initialize()?;
        
        info!("Initialization complete!");
        Ok(())
    }
    
    /// Run the demo functionality
    pub async fn run_demo(&self) -> Result<()> {
        info!("=== AI Playground Demo ===");
        
        // Demo prompt management
        println!("\n1. Prompt Management Demo:");
        for i in 0..3 {
            if let Some(template) = self.prompt_manager.get_template(i) {
                println!("Template {}: {}", i, template);
            }
        }
        
        // Demo LLM interaction
        println!("\n2. LLM Interaction Demo:");
        let test_prompt = "Hello, how are you?";
        println!(" prompt: {}", test_prompt);
        
        match self.llm_interface.generate_response(test_prompt).await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => println!("Error: {}", e),
        }
        
        // Demo component information
        println!("\n3. Component Information:");
        println!("- {}: {}", self.llm_interface.name(), self.llm_interface.description());
        println!("- {}: {}", self.prompt_manager.name(), self.prompt_manager.description());
        
        println!("\nDemo complete!");
        Ok(())
    }
    
    /// Show the interactive menu
    fn show_menu(&self) {
        println!("\n=== AI Playground Menu ===");
        println!("1. Initialize components");
        println!("2. Run demo");
        println!("3. Set API key");
        println!("4. Add prompt template");
        println!("5. List all templates");
        println!("6. Test LLM response");
        println!("7. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
    }
    
    /// Get user input
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    
    /// Handle menu option 3: Set API key
    fn handle_set_api_key(&mut self) {
        print!("Enter API key: ");
        io::stdout().flush().unwrap();
        let api_key = self.get_input();
        self.llm_interface.set_api_key(api_key);
    }
    
    /// Handle menu option 4: Add prompt template
    fn handle_add_template(&mut self) {
        print!("Enter new prompt template: ");
        io::stdout().flush().unwrap();
        let template = self.get_input();
        
        if let Err(e) = self.prompt_manager.add_template(template) {
            println!("Error: {}", e);
        }
    }
    
    /// Handle menu option 5: List all templates
    fn handle_list_templates(&self) {
        println!("\nAll available templates:");
        for (i, template) in self.prompt_manager.all_templates().iter().enumerate() {
            println!("{}: {}", i, template);
        }
    }
    
    /// Handle menu option 6: Test LLM response
    async fn handle_test_llm(&self) -> Result<()> {
        print!("Enter a test prompt: ");
        io::stdout().flush().unwrap();
        let prompt = self.get_input();
        
        match self.llm_interface.generate_response(&prompt).await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    }
    
    /// Run the main application loop
    pub async fn run(&mut self) -> Result<()> {
        println!("Welcome to AI Playground!");
        println!("A Rust project for experimenting with AI and LLM APIs");
        println!("Version 1.0.0");
        
        loop {
            self.show_menu();
            let choice = self.get_input();
            
            match choice.as_str() {
                "1" => {
                    if let Err(e) = self.initialize() {
                        error!("Initialization failed: {}", e);
                    }
                }
                "2" => {
                    if let Err(e) = self.run_demo().await {
                        error!("Demo failed: {}", e);
                    }
                }
                "3" => self.handle_set_api_key(),
                "4" => self.handle_add_template(),
                "5" => self.handle_list_templates(),
                "6" => {
                    if let Err(e) = self.handle_test_llm().await {
                        error!("LLM test failed: {}", e);
                    }
                }
                "7" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid option. Please try again."),
            }
        }
        
        Ok(())
    }
}

impl Default for AIPlayground {
    fn default() -> Self {
        Self::new()
    }
} 