#pragma once

#include <string>

/**
 * @brief Base class for all AI components in the playground
 * 
 * This class provides a common interface for all AI-related components
 * such as LLM interfaces, prompt managers, and other AI utilities.
 */
class AIComponent {
protected:
    std::string name;
    std::string description;

public:
    /**
     * @brief Constructor for AI components
     * @param n Component name
     * @param desc Component description
     */
    AIComponent(const std::string& n, const std::string& desc);
    
    /**
     * @brief Virtual destructor for proper cleanup
     */
    virtual ~AIComponent() = default;
    
    /**
     * @brief Initialize the component
     * 
     * This method should be called before using the component
     * to set up any necessary resources or connections.
     */
    virtual void initialize() = 0;
    
    /**
     * @brief Process the component's main functionality
     * 
     * This method should implement the main processing logic
     * for the component.
     */
    virtual void process() = 0;
    
    /**
     * @brief Get the component name
     * @return Component name as string
     */
    const std::string& getName() const;
    
    /**
     * @brief Get the component description
     * @return Component description as string
     */
    const std::string& getDescription() const;
}; 