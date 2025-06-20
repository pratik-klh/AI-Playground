#include "../../include/PromptManager.h"
#include <iostream>

PromptManager::PromptManager() 
    : AIComponent("Prompt Manager", "Manages and templates prompts") {
    promptTemplates = {
        "Explain {topic} in simple terms",
        "Write a {style} story about {subject}",
        "Analyze the following: {content}",
        "Generate code for {language} to {task}",
        "Summarize the key points of {text}",
        "Translate {text} to {language}",
        "Create a {type} plan for {goal}",
        "Debug this {language} code: {code}"
    };
}

void PromptManager::initialize() {
    std::cout << "Initializing Prompt Manager with " 
              << promptTemplates.size() << " templates" << std::endl;
    
    // TODO: Load templates from file if available
    // TODO: Validate template syntax
    // TODO: Set up template caching
}

void PromptManager::process() {
    if (currentPrompt.empty()) {
        std::cout << "No current prompt set" << std::endl;
        return;
    }
    std::cout << "Current prompt: " << currentPrompt << std::endl;
    
    // TODO: Process template variables
    // TODO: Apply prompt transformations
    // TODO: Validate prompt length and content
}

void PromptManager::setPrompt(const std::string& prompt) {
    currentPrompt = prompt;
}

std::string PromptManager::getTemplate(int index) const {
    if (index >= 0 && index < static_cast<int>(promptTemplates.size())) {
        return promptTemplates[index];
    }
    return "Invalid template index";
}

void PromptManager::addTemplate(const std::string& template_str) {
    if (!template_str.empty()) {
        promptTemplates.push_back(template_str);
        std::cout << "Added template: " << template_str << std::endl;
    } else {
        std::cout << "Cannot add empty template" << std::endl;
    }
} 