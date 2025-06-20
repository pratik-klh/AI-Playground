#include <iostream>
#include <string>
#include <vector>
#include <memory>

// Include our component headers
#include "../include/AIComponent.h"
#include "../include/LLMInterface.h"
#include "../include/PromptManager.h"

// Main AI Playground class
class AIPlayground {
private:
    std::unique_ptr<LLMInterface> llmInterface;
    std::unique_ptr<PromptManager> promptManager;
    std::vector<AIComponent*> components;

public:
    AIPlayground() {
        llmInterface = std::make_unique<LLMInterface>();
        promptManager = std::make_unique<PromptManager>();
        
        // Store raw pointers to components for the vector
        components.push_back(llmInterface.get());
        components.push_back(promptManager.get());
    }
    
    void initialize() {
        std::cout << "=== AI Playground Initialization ===" << std::endl;
        for (auto component : components) {
            component->initialize();
        }
        std::cout << "Initialization complete!" << std::endl;
    }
    
    void runDemo() {
        std::cout << "\n=== AI Playground Demo ===" << std::endl;
        
        // Demo prompt management
        std::cout << "\n1. Prompt Management Demo:" << std::endl;
        for (int i = 0; i < 3; ++i) {
            std::cout << "Template " << i << ": " 
                      << promptManager->getTemplate(i) << std::endl;
        }
        
        // Demo LLM interaction
        std::cout << "\n2. LLM Interaction Demo:" << std::endl;
        std::string testPrompt = "Hello, how are you?";
        std::cout << "Sending prompt: " << testPrompt << std::endl;
        std::cout << "Response: " << llmInterface->generateResponse(testPrompt) << std::endl;
        
        // Demo component information
        std::cout << "\n3. Component Information:" << std::endl;
        for (auto component : components) {
            std::cout << "- " << component->getName() << ": " 
                      << component->getDescription() << std::endl;
        }
        
        std::cout << "\nDemo complete!" << std::endl;
    }
    
    void showMenu() {
        std::cout << "\n=== AI Playground Menu ===" << std::endl;
        std::cout << "1. Initialize components" << std::endl;
        std::cout << "2. Run demo" << std::endl;
        std::cout << "3. Set API key" << std::endl;
        std::cout << "4. Add prompt template" << std::endl;
        std::cout << "5. List all templates" << std::endl;
        std::cout << "6. Test LLM response" << std::endl;
        std::cout << "7. Exit" << std::endl;
        std::cout << "Choose an option: ";
    }
    
    void run() {
        int choice;
        std::string input;
        
        while (true) {
            showMenu();
            std::cin >> choice;
            
            switch (choice) {
                case 1:
                    initialize();
                    break;
                case 2:
                    runDemo();
                    break;
                case 3:
                    std::cout << "Enter API key: ";
                    std::cin.ignore();
                    std::getline(std::cin, input);
                    llmInterface->setApiKey(input);
                    break;
                case 4:
                    std::cout << "Enter new prompt template: ";
                    std::cin.ignore();
                    std::getline(std::cin, input);
                    promptManager->addTemplate(input);
                    break;
                case 5:
                    std::cout << "\nAll available templates:" << std::endl;
                    for (size_t i = 0; i < promptManager->getTemplateCount(); ++i) {
                        std::cout << i << ": " << promptManager->getTemplate(i) << std::endl;
                    }
                    break;
                case 6:
                    std::cout << "Enter a test prompt: ";
                    std::cin.ignore();
                    std::getline(std::cin, input);
                    std::cout << "Response: " << llmInterface->generateResponse(input) << std::endl;
                    break;
                case 7:
                    std::cout << "Goodbye!" << std::endl;
                    return;
                default:
                    std::cout << "Invalid option. Please try again." << std::endl;
            }
        }
    }
};

int main() {
    std::cout << "Welcome to AI Playground!" << std::endl;
    std::cout << "A C++ project for experimenting with AI and LLM APIs" << std::endl;
    std::cout << "Version 1.0.0" << std::endl;
    
    AIPlayground playground;
    playground.run();
    
    return 0;
} 