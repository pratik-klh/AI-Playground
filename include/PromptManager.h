#pragma once

#include "AIComponent.h"
#include <string>
#include <vector>

/**
 * @brief Manages prompt templates and prompt processing
 * 
 * This class provides functionality for creating, storing, and
 * managing prompt templates for LLM interactions.
 */
class PromptManager : public AIComponent {
private:
    std::vector<std::string> promptTemplates;
    std::string currentPrompt;

public:
    /**
     * @brief Constructor for Prompt Manager
     * 
     * Initializes with default prompt templates
     */
    PromptManager();
    
    /**
     * @brief Initialize the prompt manager
     * 
     * Sets up the prompt templates and validates the configuration
     */
    void initialize() override;
    
    /**
     * @brief Process current prompt
     * 
     * Handles the current prompt processing logic
     */
    void process() override;
    
    /**
     * @brief Set the current prompt
     * @param prompt The prompt to set as current
     */
    void setPrompt(const std::string& prompt);
    
    /**
     * @brief Get a prompt template by index
     * @param index The index of the template to retrieve
     * @return The template string or "Invalid template index" if out of bounds
     */
    std::string getTemplate(int index) const;
    
    /**
     * @brief Add a new prompt template
     * @param template_str The template string to add
     */
    void addTemplate(const std::string& template_str);
    
    /**
     * @brief Get the current prompt
     * @return The current prompt string
     */
    const std::string& getCurrentPrompt() const { return currentPrompt; }
    
    /**
     * @brief Get the number of available templates
     * @return The number of prompt templates
     */
    size_t getTemplateCount() const { return promptTemplates.size(); }
    
    /**
     * @brief List all available templates
     * @return Vector of all template strings
     */
    const std::vector<std::string>& getAllTemplates() const { return promptTemplates; }
}; 