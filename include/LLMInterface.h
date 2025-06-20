#pragma once

#include "AIComponent.h"
#include <string>

/**
 * @brief Interface for Large Language Model interactions
 * 
 * This class provides an abstraction layer for communicating
 * with various LLM APIs such as OpenAI GPT, Anthropic Claude, etc.
 */
class LLMInterface : public AIComponent {
private:
    std::string apiKey;
    std::string modelName;
    bool isConnected;

public:
    /**
     * @brief Constructor for LLM Interface
     * @param model Name of the LLM model to use (default: "gpt-3.5-turbo")
     */
    explicit LLMInterface(const std::string& model = "gpt-3.5-turbo");
    
    /**
     * @brief Initialize the LLM interface
     * 
     * Sets up the connection and validates the API key
     */
    void initialize() override;
    
    /**
     * @brief Process LLM requests
     * 
     * Handles the main processing logic for LLM interactions
     */
    void process() override;
    
    /**
     * @brief Set the API key for authentication
     * @param key The API key string
     */
    void setApiKey(const std::string& key);
    
    /**
     * @brief Generate a response from the LLM
     * @param prompt The input prompt to send to the LLM
     * @return The generated response from the LLM
     */
    std::string generateResponse(const std::string& prompt);
    
    /**
     * @brief Get the current model name
     * @return The name of the current LLM model
     */
    const std::string& getModelName() const { return modelName; }
    
    /**
     * @brief Check if the interface is connected
     * @return True if connected, false otherwise
     */
    bool isInterfaceConnected() const { return isConnected; }
}; 