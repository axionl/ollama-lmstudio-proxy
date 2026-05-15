# Summary of ollama-lmstudio-proxy

## Overview
This is a proxy server that bridges the Ollama API and LM Studio by translating requests from one format to another, allowing users to use LM Studio models through the familiar Ollama API.

Key features include:
- Full support for core Ollama endpoints like `/api/chat`, `/api/generate`, `/api/tags`
- Model aliasing system that allows creating virtual model names
- Streaming response handling with chunk recovery and cancellation support  
- SSE to NDJSON conversion for streaming compatibility
- Blob storage management for model artifacts

## Main Components 

### Core Architecture
- `src/main.rs`: Entry point with configuration parsing
- `src/server/proxy.rs`: Main server logic managing client, config, model resolver, and stores
- `src/server/routes.rs`: Route definitions mapping incoming requests to handlers
- `src/handlers/ollama/mod.rs`: All Ollama-specific endpoint handlers

### Model Resolution & Management  
- `src/model/resolver.rs`: Resolves Ollama model names to LM Studio IDs using caching
- `src/storage/virtual_models.rs`: Handles virtual model aliases (create/copy/delete operations)

### HTTP Layer
- `src/http/client.rs`: Wraps reqwest with cancellation support for LM Studio calls  
- `src/http/request.rs`: Transforms Ollama parameters to LM Studio format
- `src/streaming/sse.rs`: Processes SSE streams from LM Studio into NDJSON chunks

## Key Endpoints Supported

### Ollama Compliant Endpoints:
- `/api/tags` - List available models (includes virtual model aliases)
- `/api/chat` - Chat completions with streaming support  
- `/api/generate` - Text generation
- `/api/embeddings` - Generate embeddings
- `/api/pull` - Download models with status streaming
- `/api/create` - Create virtual model alias 
- `/api/copy` - Copy existing model to new alias
- `/api/delete` - Delete virtual model alias
- `/api/show` - Model metadata inspection
- `/api/ps` - List loaded models  
- `/api/version` - Proxy version information
- `/api/blobs/*` - Blob storage operations (HEAD, POST)

### Health & Status:
- `/health` - Health check endpoint for LM Studio connectivity
- `/` - Root endpoint returning "Ollama is running"

## Configuration Options

The proxy supports command-line arguments to configure behavior:
- `--listen`: Server bind address 
- `--lmstudio-url`: LM Studio backend URL
- `--log-level`: Log verbosity level  
- `--load-timeout-seconds`: Model loading wait timeout
- `--max-buffer-size`: Buffer size for streaming response assembly
- `--enable-chunk-recovery`: Enable partial chunk recovery during streaming

## Architecture Design

The proxy uses a layered architecture:
1. **Routing Layer**: Maps incoming requests to appropriate handlers
2. **Request Handling Layer**: Processes Ollama-specific request formats and translates them  
3. **Model Resolution Layer**: Translates between Ollama model names and LM Studio IDs
4. **Communication Layer**: Forwards requests to and from LM Studio backend with proper error handling
5. **Streaming Layer**: Handles streaming responses with chunk recovery and cancellation support

## Compatibility

The proxy provides compatibility with both:
- Ollama's API specification 
- LM Studio's native API endpoints for model management and generation