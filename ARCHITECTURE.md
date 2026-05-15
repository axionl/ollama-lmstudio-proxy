# ollama-lmstudio-proxy Architecture Overview

This proxy server bridges the Ollama API and LM Studio by acting as an intermediary that translates requests from one format to another.

## Key Components

### 1. Main Server Structure
- `src/main.rs`: Entry point with configuration parsing and main loop
- `src/server/proxy.rs`: Core server logic managing client, config, model resolver, and stores 
- `src/server/routes.rs`: Route definitions that map incoming requests to handlers
- `src/server/mod.rs`: Module exports for proxy components

### 2. Request Handling & Routing
- `src/handlers/mod.rs`: Exposes all handler modules for different API endpoints
- `src/handlers/ollama/mod.rs`: All Ollama-specific handlers like chat, generate, pull etc.
- `src/handlers/lmstudio.rs`: Forwards requests to LM Studio backend with parameter translation

### 3. Model Resolution & Management  
- `src/model/resolver.rs`: Resolves Ollama model names to LM Studio IDs using caching
- `src/model/types.rs`: Defines data structures for models and their metadata
- `src/storage/virtual_models.rs`: Handles virtual model aliases (create/copy/delete operations)
- `src/handlers/ollama/model_resolution.rs`: Model resolution context building

### 4. HTTP Layer
- `src/http/client.rs`: Wraps reqwest with cancellation support for LM Studio calls
- `src/http/parsing.rs`: Parses JSON request bodies and content types
- `src/http/request.rs`: Transforms Ollama parameters to LM Studio format  
- `src/http/response.rs`: Handles response formatting and forwarding

### 5. Streaming & Chunking
- `src/streaming/mod.rs`: Contains streaming handling modules
- `src/streaming/sse.rs`: Processes SSE streams from LM Studio into NDJSON chunks
- `src/streaming/chunks.rs`: Manages chunk processing, serialization, and error handling for streaming responses

### 6. Storage & Caching
- `src/storage/blob.rs`: Blob storage for model artifacts using SHA256 digests  
- `src/storage/virtual_models.rs`: Virtual model management (aliases)

## Key Features Implemented

1. **Ollama API Compatibility**: Full support for `/api/chat`, `/api/generate`, `/api/tags`, `/api/pull` etc.
2. **LM Studio Integration**: Forwarding to LM Studio native endpoints
3. **Model Aliasing**: Create virtual models with custom metadata 
4. **Streaming Support**: Proper handling of streaming responses from LM Studio
5. **Chunk Recovery**: Reconstructs broken chunks for robust streaming
6. **Keep-Alive Handling**: Model unloading based on TTL values
7. **Caching**: Model resolution caching to reduce API calls

## Key Endpoints Supported

### Ollama Compliant Endpoints:
- `/api/tags` - List available models  
- `/api/chat` - Chat completions (supports streaming)
- `/api/generate` - Text generation 
- `/api/embeddings` - Generate embeddings
- `/api/pull` - Download model with status streaming
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