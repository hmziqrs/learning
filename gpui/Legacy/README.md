# ReqForge - A Postman-like HTTP Client in Rust + GPUI

ReqForge is a modern HTTP client application built with Rust and GPUI, inspired by Postman. It provides a sleek, native interface for making HTTP requests, managing collections, and working with different environments.

## Project Overview

ReqForge consists of two main components:

1. **reqforge-core** - A headless library containing all domain logic, HTTP execution, environment interpolation, and JSON persistence
2. **reqforge-app** - A GPUI-based application that provides the user interface

The project is currently in **Phase 1** with the core functionality implemented. Phase 2 (the full GPUI application) is stubbed due to GPUI dependency challenges.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   GPUI Frontend                      │
│  (Views, Panels, Input Bindings, Theming)            │
│                                                      │
│  ┌───────────┐ ┌───────────┐ ┌────────────────────┐ │
│  │ Sidebar   │ │ Request   │ │ Response Viewer    │ │
│  │ (Tree)    │ │ Editor    │ │ (Body/Headers/Meta)│ │
│  └───────────┘ └───────────┘ └────────────────────┘ │
├─────────────────────────────────────────────────────┤
│                   Bridge Layer                       │
│  (Adapters: converts core types → GPUI view models)  │
├─────────────────────────────────────────────────────┤
│                 reqforge-core (lib)                   │
│  ┌────────┐ ┌──────────┐ ┌──────┐ ┌──────────────┐ │
│  │ HTTP   │ │ Environ- │ │Store │ │ Collections  │ │
│  │ Engine │ │ ments    │ │(JSON)│ │ & Folders    │ │
│  └────────┘ └──────────┘ └──────┘ └──────────────┘ │
└─────────────────────────────────────────────────────┘
```

## Features

- ✅ **Core HTTP Client** - Execute HTTP requests (GET, POST, PUT, DELETE, etc.)
- ✅ **Environment Variables** - Manage multiple environments with variables
- ✅ **Collection Management** - Organize requests in folders and collections
- ✅ **JSON Import/Export** - Save and load requests from JSON files
- ✅ **Response Viewer** - View responses with syntax highlighting
- ✅ **Variable Interpolation** - Use `{{variable}}` syntax in URLs and headers
- ✅ **Authentication Support** - Bearer tokens, API keys, and custom headers
- ✅ **Query Parameters** - Add URL query parameters to requests
- ✅ **Request Body** - Support for JSON, form data, and raw text bodies

## Project Status

### ✅ Phase 1 - Core Library (Complete)
- HTTP client engine
- Environment and collection models
- JSON persistence
- CLI tool for testing requests
- Variable interpolation
- Basic authentication support

### ⚠️ Phase 2 - GPUI Application (Stub)
Due to challenges with GPUI dependencies and compilation issues, the full GPUI application has been stubbed. The core library is complete and functional, but the UI components need to be implemented.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building the Project

```bash
# Clone the repository
git clone <repository-url>
cd gpui

# Build the project
cargo build --release

# Run tests
cargo test

# Build the CLI tool
cargo build -p reqforge-cli --release
```

## How to Run the CLI

The CLI tool allows you to execute HTTP requests defined in JSON files.

### Basic Usage

```bash
# Execute a simple GET request
cargo run -p reqforge-cli -- examples/basic-get-request.json

# Execute a POST request with JSON body
cargo run -p reqforge-cli -- examples/json-post-request.json

# Execute a form POST request
cargo run -p reqforge-cli -- examples/form-post-request.json

# Execute a complex API request
cargo run -p reqforge-cli -- examples/complex-request.json
```

### Using the CLI with Environment Variables

```bash
# Set environment variables for your requests
export BASE_URL=https://api.example.com
export API_KEY=your-api-key

# Create a request file that uses these variables
# (see data/collections/api-example.json for examples)

# Execute the request
cargo run -p reqforge-cli -- data/collections/api-example.json
```

## File Structure

```
gpui/
├── data/                          # Example data
│   ├── environments.json          # Environment definitions
│   └── collections/                # Example collections
│       ├── api-example.json       # API example collection
│       └── simple.json            # Simple collection
├── examples/                      # Example request files
│   ├── basic-get-request.json      # Basic GET example
│   ├── json-post-request.json      # JSON POST example
│   ├── form-post-request.json      # Form POST example
│   └── complex-request.json       # Complex API example
├── crates/
│   ├── reqforge-core/              # Core library
│   ├── reqforge-app/               # GPUI application (stubbed)
│   └── reqforge-cli/               # CLI tool
├── src/                           # Source files (legacy)
├── tests/                         # Test files
└── docs/                          # Documentation
    └── plan.md                    # Project plan
```

## Example Data

### Environments

The `data/environments.json` file contains three example environments:

- **Development** - Local development with debug mode
- **Staging** - Staging environment with API keys
- **Production** - Production environment with secure configuration

### Collections

- **API Example Collection** - Demonstrates typical API patterns with users and posts endpoints
- **Simple Collection** - Basic GET and POST requests for testing

### Example Requests

The `examples/` directory contains various request examples:

- Basic GET request to JSONPlaceholder
- POST request with JSON body
- Form data POST request
- Complex API request with authentication

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p reqforge-core
cargo test -p reqforge-cli

# Run with output formatting
cargo test -- --nocapture
```

## Configuration

### Environment Variables

- `BASE_URL` - Base URL for API requests
- `API_KEY` - API key for authentication
- `DEBUG` - Enable debug mode

### JSON Schema

Requests follow this JSON structure:

```json
{
  "name": "Request name",
  "method": "GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS",
  "url": "https://example.com/api/endpoint",
  "headers": [
    {
      "key": "Header name",
      "value": "Header value",
      "enabled": true,
      "description": "Header description"
    }
  ],
  "query_params": [
    {
      "key": "param",
      "value": "value",
      "enabled": true,
      "description": "Parameter description"
    }
  ],
  "body": "None" or {
    "type": "Raw|FormUrlEncoded",
    "content": "Body content",
    "content_type": "Json|Xml|Text|Html"
  }
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Future Plans

### Phase 2 - GPUI Application
Implement the full GUI application with:
- Native windows and controls
- Request editor with syntax highlighting
- Response viewer with formatted display
- Collection tree view
- Environment selector
- Theme support
- Keyboard shortcuts

### Phase 3 - Advanced Features
- Request history
- Test assertions
- Environment variables management
- Scripting support
- Plugins and extensions
- Collaboration features