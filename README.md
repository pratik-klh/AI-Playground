# AI Playground

A Rust project for experimenting with AI and LLM APIs. This project provides a modular framework for building AI-powered applications with components for LLM interactions, prompt management, and extensible AI utilities.

## Features

- **LLM Interface**: Abstraction layer for communicating with various LLM APIs (OpenAI GPT, Anthropic Claude, etc.)
- **Prompt Manager**: Template-based prompt management with variable substitution
- **Modular Architecture**: Extensible component system for adding new AI capabilities
- **Async Support**: Built with Tokio for efficient async operations
- **Error Handling**: Comprehensive error handling with `anyhow`
- **Logging**: Structured logging with `tracing`

## Project Structure

```
AI-Playground/
├── Cargo.toml              # Rust project configuration
├── src/
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library exports
│   ├── components/         # AI component modules
│   │   ├── mod.rs          # Component module exports
│   │   ├── ai_component.rs # Base AIComponent trait
│   │   ├── llm_interface.rs # LLM API interface
│   │   └── prompt_manager.rs # Prompt template management
│   └── playground/         # Main application logic
│       ├── mod.rs          # Playground module exports
│       └── ai_playground.rs # Main AIPlayground struct
└── README.md               # This file
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd AI-Playground
```

2. Build the project:
```bash
cargo build
```

3. Run the application:
```bash
cargo run
```

### Development

For development with hot reloading:
```bash
cargo watch -x run
```

### Testing

Run the test suite:
```bash
cargo test
```

## Usage

The AI Playground provides an interactive command-line interface with the following options:

1. **Initialize components** - Set up all AI components
2. **Run demo** - Execute a demonstration of all features
3. **Set API key** - Configure your LLM API key
4. **Add prompt template** - Create new prompt templates
5. **List all templates** - View available prompt templates
6. **Test LLM response** - Send a test prompt to the LLM
7. **Exit** - Close the application

## Components

### AIComponent Trait

The base trait for all AI components:

```rust
pub trait AIComponent: Debug + Send + Sync {
    fn initialize(&mut self) -> anyhow::Result<()>;
    fn process(&self) -> anyhow::Result<()>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
```

### LLMInterface

Handles communication with Large Language Model APIs:

```rust
let mut llm = LLMInterface::new(Some("gpt-3.5-turbo".to_string()));
llm.initialize()?;
llm.set_api_key("your-api-key".to_string());
let response = llm.generate_response("Hello, world!").await?;
```

### PromptManager

Manages prompt templates and variable substitution:

```rust
let mut pm = PromptManager::new();
pm.add_template("Explain {topic} in simple terms".to_string())?;
pm.set_variable("topic".to_string(), "quantum physics".to_string());
let processed = pm.get_processed_template(0).unwrap();
```

## Configuration

The project uses environment variables for configuration. Create a `.env` file:

```env
LLM_API_KEY=your-api-key-here
LLM_MODEL=gpt-3.5-turbo
LLM_MAX_TOKENS=1000
LLM_TEMPERATURE=0.7
```

## Dependencies

- **reqwest**: HTTP client for API calls
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **anyhow**: Error handling
- **tracing**: Logging framework
- **clap**: CLI argument parsing
- **config**: Configuration management

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Migration from C++

This project was converted from a C++ implementation. Key changes include:

- **Memory Safety**: Rust's ownership system eliminates memory leaks and data races
- **Async/Await**: Native async support with Tokio runtime
- **Error Handling**: Comprehensive error handling with `anyhow`
- **Package Management**: Cargo for dependency management
- **Type Safety**: Strong type system with compile-time guarantees

## Roadmap

- [ ] Implement actual LLM API calls (OpenAI, Anthropic)
- [ ] Add support for multiple LLM providers
- [ ] Implement prompt template file loading
- [ ] Add configuration file support
- [ ] Create web interface
- [ ] Add unit and integration tests
- [ ] Implement plugin system for custom components
