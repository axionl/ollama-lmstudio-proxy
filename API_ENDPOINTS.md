# API Endpoints Supported by ollama-lmstudio-proxy

This proxy server implements a comprehensive set of Ollama-compatible endpoints that translate to LM Studio native APIs.

## Core Ollama API Endpoints

### `/api/tags` (GET)
- **Purpose**: List all available models
- **Functionality**: Retrieves model information from LM Studio and formats it as Ollama tags response
- **Features**: 
  - Shows both actual LM Studio models and virtual model aliases
  - Includes size, digest, and metadata for each model

### `/api/chat` (POST)
- **Purpose**: Chat completions with streaming support  
- **Functionality**: Translates Ollama chat format to LM Studio native API calls
- **Features**:
  - Streaming response handling via SSE-to-NDJSON conversion
  - Tool call support for function calling
  - Reasoning content extraction and processing
  - Model resolution, parameter mapping, and keep-alive management

### `/api/generate` (POST)
- **Purpose**: Text generation with prompt-based input  
- **Functionality**: Converts Ollama generate requests to LM Studio format
- **Features**:
  - Streaming support for long-running generations
  - Support for image inputs via vision models
  - System prompt injection and parameter mapping
  - Raw mode handling

### `/api/embeddings` (POST)
- **Purpose**: Generate embeddings for text input
- **Functionality**: Maps to LM Studio embedding endpoint with proper response formatting  
- **Features**:
  - Dual support for both legacy and modern embedding formats
  - Proper chunking and error handling

### `/api/pull` (POST) 
- **Purpose**: Download models from remote sources or local storage
- **Functionality**: Initiates model download through LM Studio's native API with progress streaming
- **Features**:
  - Streamed status updates during download process
  - Support for quantization specification and source override
  - Retry logic on connection failures

### `/api/create` (POST)
- **Purpose**: Create virtual model aliases from existing models  
- **Functionality**: Creates a named alias that maps to an actual LM Studio model with optional custom metadata
- **Features**:
  - Streamed status updates for multi-step process
  - System prompt and template customization support
  - Parameter inheritance from source model

### `/api/copy` (POST)  
- **Purpose**: Copy existing models or virtual aliases to new names
- **Functionality**: Creates a new alias pointing to the same target model as an existing one
- **Features**:
  - Support for both real and virtual source models
  - Preserves all metadata from original

### `/api/delete` (POST)
- **Purpose**: Remove virtual model aliases  
- **Functionality**: Deletes stored virtual model entries from local cache
- **Features**:
  - Returns deletion confirmation response with timestamp details

### `/api/show` (POST)  
- **Purpose**: Get detailed information about a specific model
- **Functionality**: Returns comprehensive metadata including architecture, parameters and capabilities
- **Features**:
  - Shows both base model properties and virtual alias overrides
  - Includes system prompt, template and other customization fields

### `/api/ps` (GET)
- **Purpose**: List currently loaded models in the proxy environment  
- **Functionality**: Combines actual LM Studio loaded models with virtual model references
- **Features**:
  - Shows virtual model status alongside physical models
  - Includes context length, size and loading state information

### `/api/version` (GET)
- **Purpose**: Retrieve version information about this proxy server
- **Functionality**: Returns the current version string configured for the proxy  
- **Features**:
  - Supports custom version override via command-line argument

## Health & Status Endpoints  

### `/health` (GET) 
- **Purpose**: Monitor health and connectivity to LM Studio backend
- **Functionality**: Verifies connection to LM Studio by fetching model list
- **Features**:
  - Response includes model count, HTTP status codes, and response times
  - Detailed error messages for troubleshooting

### `/` (GET)
- **Purpose**: Root endpoint for service health check  
- **Functionality**: Returns simple "Ollama is running" message with 200 OK status

## Blob Storage Endpoints  

### `/api/blobs/*` 
- **Purpose**: Direct blob storage management
- **Functionality**: Supports HEAD and POST operations for model artifacts
- **Features**:
  - SHA256-based content addressing  
  - Secure temporary file handling during uploads
  - Atomic file moves for consistency

## Configuration & Management

### Command-line Arguments (via clap):
- `--listen`: Server bind address (default: 0.0.0.0:11434)
- `--lmstudio-url`: LM Studio backend URL (default: http://localhost:1234) 
- `--log-level`: Log verbosity level
- `--load-timeout-seconds`: Model loading wait timeout
- `--max-buffer-size`: Buffer size for streaming response assembly
- `--enable-chunk-recovery`: Enable partial chunk recovery during streaming
- `--update-check`: Check for updates to the proxy server

## Request/Response Format Compatibility

The proxy ensures full compatibility with Ollama's API specifications:
- Standard JSON request/response structures  
- Streaming responses using NDJSON format for compatibility
- Proper handling of all standard Ollama parameters and fields
- Error code translation between LM Studio and Ollama formats