# AI-Playground

A C++ project for experimenting with various LLM APIs and AI components.

## Overview

AI-Playground is a modular C++ application designed to provide a foundation for experimenting with Large Language Models (LLMs) and AI-related functionality. The project includes:

- **LLM Interface**: Abstract interface for communicating with various LLM APIs
- **Prompt Manager**: Template-based prompt management system
- **Modular Architecture**: Extensible component-based design
- **Interactive CLI**: User-friendly command-line interface

## Features

- 🧠 **LLM Integration**: Ready-to-use interface for LLM APIs (OpenAI, Anthropic, etc.)
- 📝 **Prompt Templates**: Pre-built and customizable prompt templates
- 🔧 **Modular Design**: Easy to extend with new AI components
- 🎯 **Interactive Demo**: Built-in demonstration of all features
- 📚 **Well Documented**: Comprehensive code documentation

## Project Structure

```
AI-Playground/
├── CMakeLists.txt          # Build configuration
├── README.md              # This file
├── .gitignore             # Git ignore rules
├── include/               # Header files
│   ├── AIComponent.h      # Base component interface
│   ├── LLMInterface.h     # LLM API interface
│   └── PromptManager.h    # Prompt management
├── src/                   # Source files
│   ├── main.cpp           # Main application
│   └── components/        # Component implementations
│       ├── AIComponent.cpp
│       ├── LLMInterface.cpp
│       └── PromptManager.cpp
└── build/                 # Build output (generated)
```

## Prerequisites

- **C++17 compatible compiler** (GCC 7+, Clang 5+, MSVC 2017+)
- **CMake 3.10 or higher**
- **Make** or **Ninja** build system

## Building the Project

### 1. Clone and Navigate
```bash
cd AI-Playground
```

### 2. Create Build Directory
```bash
mkdir build
cd build
```

### 3. Configure with CMake
```bash
cmake ..
```

### 4. Build the Project
```bash
make
# or with Ninja: ninja
```

### 5. Run the Application
```bash
./bin/ai-playground
```

## Usage

### Interactive Mode
The application provides an interactive menu with the following options:

1. **Initialize components** - Set up all AI components
2. **Run demo** - Execute a demonstration of all features
3. **Set API key** - Configure your LLM API key
4. **Add prompt template** - Create custom prompt templates
5. **List all templates** - View available prompt templates
6. **Test LLM response** - Send test prompts to the LLM
7. **Exit** - Close the application

### Example Session
```
Welcome to AI Playground!
A C++ project for experimenting with AI and LLM APIs
Version 1.0.0

=== AI Playground Menu ===
1. Initialize components
2. Run demo
3. Set API key
4. Add prompt template
5. List all templates
6. Test LLM response
7. Exit
Choose an option: 1

=== AI Playground Initialization ===
Initializing LLM Interface for model: gpt-3.5-turbo
Initializing Prompt Manager with 8 templates
Initialization complete!
```

## Extending the Project

### Adding New Components

1. **Create Header File** (`include/NewComponent.h`):
```cpp
#pragma once
#include "AIComponent.h"

class NewComponent : public AIComponent {
public:
    NewComponent();
    void initialize() override;
    void process() override;
    // Add your custom methods
};
```

2. **Create Implementation** (`src/components/NewComponent.cpp`):
```cpp
#include "../../include/NewComponent.h"

NewComponent::NewComponent() 
    : AIComponent("New Component", "Description of new component") {}

void NewComponent::initialize() override {
    // Implementation
}

void NewComponent::process() override {
    // Implementation
}
```

3. **Update CMakeLists.txt** to include the new source file
4. **Integrate** into the main application

### Adding Real LLM API Support

The project is designed to easily integrate with real LLM APIs:

1. **Install HTTP client library** (e.g., libcurl)
2. **Install JSON library** (e.g., nlohmann/json)
3. **Uncomment** the relevant lines in `CMakeLists.txt`
4. **Implement** actual API calls in `LLMInterface.cpp`

## Development

### Code Style
- Follow C++17 standards
- Use meaningful variable and function names
- Include comprehensive documentation
- Implement proper error handling

### Testing
- Add unit tests for new components
- Test with different LLM providers
- Validate prompt template functionality

## Future Enhancements

- [ ] **Real API Integration**: Connect to actual LLM APIs
- [ ] **Web Interface**: Add web-based UI
- [ ] **Configuration Files**: Support for JSON/YAML configs
- [ ] **Plugin System**: Dynamic component loading
- [ ] **Batch Processing**: Handle multiple prompts
- [ ] **Response Caching**: Cache LLM responses
- [ ] **Template Variables**: Dynamic prompt templating
- [ ] **Export/Import**: Save and load configurations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source. Please check the license file for details.

## Support

For questions or issues, please open an issue on the project repository.
