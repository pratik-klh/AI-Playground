use crate::components::{AIComponent, NamedComponent};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use serde_json;

/// Configuration for LLM API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub api_key: Option<String>,
}

/// Message structure for LLM conversations
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Response structure from LLM API
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LLMResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Choice {
    message: Message,
}

/// Interface for Large Language Model interactions
/// 
/// This struct provides an abstraction layer for communicating
/// with various LLM APIs such as OpenAI GPT, Anthropic Claude, etc.
#[derive(Debug)]
pub struct LLMInterface {
    name: String,
    description: String,
    config: LLMConfig,
    client: Option<reqwest::Client>,
    is_connected: bool,
}

impl LLMInterface {
    /// Create a new LLM Interface
    pub fn new(model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "gpt-3.5-turbo".to_string());
        
        Self {
            name: "LLM Interface".to_string(),
            description: "Interface for Large Language Models".to_string(),
            config: LLMConfig {
                model,
                max_tokens: Some(1000),
                temperature: Some(0.7),
                api_key: None,
            },
            client: None,
            is_connected: false,
        }
    }
    
    /// Set the API key for authentication
    pub fn set_api_key(&mut self, key: String) {
        self.config.api_key = Some(key);
        info!("API key set for model: {}", self.config.model);
    }
    
    /// Generate a response from the LLM
    pub async fn generate_response(&self, prompt: &str) -> Result<String> {
        if !self.is_connected {
            return Err(anyhow::anyhow!("LLM Interface not initialized"));
        }

        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("HTTP client not initialized"))?;

        // Ollama does not require an API key for local use
        // Prepare the request for Ollama's /api/chat endpoint
        let url = "http://localhost:11434/api/chat";
        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        // Optionally, you could add a system message here if desired
        // messages.insert(0, Message { role: "system".to_string(), content: "You are a helpful assistant.".to_string() });

        let mut body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "stream": false,
        });

        // Add temperature if set
        if let Some(temp) = self.config.temperature {
            body["options"] = serde_json::json!({ "temperature": temp });
        }

        let resp = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!(format!("Ollama API error: {}", resp.status())));
        }

        let resp_json: serde_json::Value = resp.json().await?;
        // The response is expected to have a structure like:
        // { "message": { "role": "assistant", "content": "..." }, ... }
        let content = resp_json
            .get("message")
            .and_then(|msg| msg.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| anyhow::anyhow!("Malformed response from Ollama"))?;

        Ok(content.to_string())
    }
    
    /// Get the current model name
    pub fn model_name(&self) -> &str {
        &self.config.model
    }
    
    /// Check if the interface is connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}

impl NamedComponent for LLMInterface {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_description(&self) -> &str {
        &self.description
    }
}

impl AIComponent for LLMInterface {
    fn initialize(&mut self) -> Result<()> {
        info!("Initializing LLM Interface for model: {}", self.config.model);
        
        // Create HTTP client
        self.client = Some(reqwest::Client::new());
        
        // TODO: Add actual API initialization
        // - Validate API key format
        // - Test connection to API endpoint
        // - Set up HTTP client configuration
        
        self.is_connected = true;
        info!("LLM Interface initialized successfully");
        Ok(())
    }
    
    fn process(&self) -> Result<()> {
        if !self.is_connected {
            warn!("LLM Interface not connected. Please initialize first.");
            return Err(anyhow::anyhow!("Interface not connected"));
        }
        
        info!("Processing with LLM model: {}", self.config.model);
        Ok(())
    }
    
    fn name(&self) -> &str {
        self.get_name()
    }
    
    fn description(&self) -> &str {
        self.get_description()
    }
} 