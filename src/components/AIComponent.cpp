#include "../../include/AIComponent.h"

AIComponent::AIComponent(const std::string& n, const std::string& desc) 
    : name(n), description(desc) {}

const std::string& AIComponent::getName() const { 
    return name; 
}

const std::string& AIComponent::getDescription() const { 
    return description; 
} 