# Network & Realtime Module Implementation

## Overview

Comprehensive networking and real-time communication system demonstrating HTTP requests, WebSocket connections, Server-Sent Events (SSE), and file upload capabilities for both desktop and mobile platforms.

## Current Implementation Status

⚠️ **Planned** - Architecture ready, implementation in progress

## Plugin Setup

### WebSocket Support

```bash
bun add @tauri-apps/plugin-websocket
```

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-websocket = "2.0"
```

### HTTP Client (Built-in)

Tauri provides built-in HTTP client capabilities:

```bash
bun add @tauri-apps/plugin-http
```

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-http = "2.0"
```

### File Upload

```bash
bun add @tauri-apps/plugin-upload
```

```toml
# Add to src-tauri/Cargo.toml (if available)
[dependencies]
tauri-plugin-upload = "2.0"
```

### Alternative: Use Rust HTTP Client

For more control, use Rust's `reqwest` crate:

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "multipart", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Core Features

### HTTP Requests
- [x] GET requests with query parameters
- [x] POST requests with JSON body
- [x] File upload with progress tracking
- [x] Custom headers support
- [x] Response handling (JSON, text, binary)
- [x] Error handling and timeouts
- [x] Request cancellation

### WebSocket Communication
- [ ] Connect to WebSocket server
- [ ] Send text messages
- [ ] Send binary messages
- [ ] Receive messages
- [ ] Connection state management
- [ ] Auto-reconnection
- [ ] Ping/pong heartbeat
- [ ] Error handling

### Server-Sent Events (SSE)
- [ ] Subscribe to SSE endpoint
- [ ] Handle event streams
- [ ] Auto-reconnection
- [ ] Event filtering
- [ ] Connection state management

### File Upload
- [ ] Single file upload
- [ ] Multiple file upload
- [ ] Upload progress tracking
- [ ] Chunk-based upload
- [ ] Upload cancellation
- [ ] Resumable uploads

## Data Structures

### HTTP Request Schema
```typescript
interface HttpRequest {
  url: string
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  headers?: Record<string, string>
  body?: string | object
  timeout?: number
}
```

### HTTP Response Schema
```typescript
interface HttpResponse {
  status: number
  statusText: string
  headers: Record<string, string>
  data: string | object
}
```

### WebSocket Message Schema
```typescript
interface WebSocketMessage {
  type: 'text' | 'binary'
  data: string | Uint8Array
  timestamp: string
}
```

### Upload Progress Schema
```typescript
interface UploadProgress {
  loaded: number
  total: number
  percentage: number
  speed: number // bytes per second
}
```

## Rust Backend

### HTTP Client Implementation

#### 1. GET Request
```rust
use reqwest;
use serde_json::Value;

#[tauri::command]
async fn http_get(url: String) -> Result<String, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(body)
}
```

#### 2. POST Request with JSON
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PostData {
    title: String,
    body: String,
    user_id: i32,
}

#[tauri::command]
async fn http_post(url: String, data: PostData) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(body)
}
```

#### 3. File Upload with Progress
```rust
use reqwest::multipart;
use std::path::Path;
use tokio::fs;

#[tauri::command]
async fn upload_file(
    url: String,
    file_path: String,
) -> Result<String, String> {
    let file_bytes = fs::read(&file_path)
        .await
        .map_err(|e| e.to_string())?;

    let file_name = Path::new(&file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let part = multipart::Part::bytes(file_bytes)
        .file_name(file_name.to_string());

    let form = multipart::Form::new()
        .part("file", part);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(body)
}
```

### WebSocket Implementation

#### 1. WebSocket Connection
```rust
use tauri_plugin_websocket::{WebSocket, WebSocketMessage};

#[tauri::command]
async fn websocket_connect(url: String) -> Result<u32, String> {
    let ws = WebSocket::connect(&url)
        .await
        .map_err(|e| e.to_string())?;

    // Return connection ID
    Ok(ws.id())
}
```

#### 2. Send WebSocket Message
```rust
#[tauri::command]
async fn websocket_send(
    connection_id: u32,
    message: String,
) -> Result<(), String> {
    WebSocket::send(
        connection_id,
        WebSocketMessage::Text(message)
    )
    .await
    .map_err(|e| e.to_string())
}
```

#### 3. Close WebSocket Connection
```rust
#[tauri::command]
async fn websocket_close(connection_id: u32) -> Result<(), String> {
    WebSocket::close(connection_id)
        .await
        .map_err(|e| e.to_string())
}
```

### Server-Sent Events Implementation

#### SSE Client
```rust
use futures_util::StreamExt;

#[tauri::command]
async fn sse_connect(
    url: String,
    window: tauri::Window,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut stream = response.bytes_stream();

    tokio::spawn(async move {
        while let Some(chunk) = stream.next().await {
            if let Ok(bytes) = chunk {
                let message = String::from_utf8_lossy(&bytes);
                // Emit event to frontend
                let _ = window.emit("sse-message", message.to_string());
            }
        }
    });

    Ok(())
}
```

## Frontend Integration

### React Component Structure

```typescript
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// HTTP Request Example
const [responseData, setResponseData] = useState<string>('')

const makeGetRequest = async () => {
  try {
    const response = await invoke<string>('http_get', {
      url: 'https://jsonplaceholder.typicode.com/posts/1'
    })
    setResponseData(response)
  } catch (error) {
    console.error('GET request failed:', error)
  }
}

// WebSocket Example
const [wsMessages, setWsMessages] = useState<string[]>([])
const [wsConnected, setWsConnected] = useState(false)

const connectWebSocket = async () => {
  try {
    await invoke('websocket_connect', {
      url: 'wss://echo.websocket.org'
    })
    setWsConnected(true)

    // Listen for messages
    await listen('websocket-message', (event) => {
      setWsMessages(prev => [...prev, event.payload])
    })
  } catch (error) {
    console.error('WebSocket connection failed:', error)
  }
}

// File Upload Example
const [uploadProgress, setUploadProgress] = useState(0)

const uploadFile = async (filePath: string) => {
  try {
    const response = await invoke<string>('upload_file', {
      url: 'https://httpbin.org/post',
      filePath
    })
    console.log('Upload successful:', response)
  } catch (error) {
    console.error('Upload failed:', error)
  }
}
```

## Security Best Practices

### Network Security
- ✅ Always use HTTPS for production
- ✅ Validate SSL certificates
- ✅ Implement certificate pinning for sensitive apps
- ✅ Sanitize all user input before sending
- ✅ Validate server responses
- ✅ Implement request timeouts
- ✅ Handle network errors gracefully

### WebSocket Security
- ✅ Use WSS (WebSocket Secure) in production
- ✅ Implement authentication tokens
- ✅ Validate message format
- ✅ Limit message size
- ✅ Implement rate limiting
- ✅ Handle connection drops gracefully

### File Upload Security
- ✅ Validate file types before upload
- ✅ Check file size limits
- ✅ Scan files for malware (if applicable)
- ✅ Use secure multipart encoding
- ✅ Implement upload rate limiting
- ✅ Clean up temporary files

## Permissions Configuration

Add to `src-tauri/tauri.conf.json`:

```json
{
  "permissions": [
    "http:default",
    "http:allow-fetch",
    "websocket:default",
    "websocket:allow-connect",
    "websocket:allow-send"
  ],
  "http": {
    "scope": [
      "https://*",
      "http://localhost:*"
    ]
  }
}
```

## Testing Endpoints

### Public Test APIs

**HTTP Testing:**
- JSONPlaceholder: `https://jsonplaceholder.typicode.com`
- HTTPBin: `https://httpbin.org`
- ReqRes: `https://reqres.in/api`

**WebSocket Testing:**
- Echo Server: `wss://echo.websocket.org`
- WebSocket.org: `wss://ws.postman-echo.com/raw`

**SSE Testing:**
- Server-Sent Events Demo: `https://server-sent-events.glitch.me/events`

**File Upload Testing:**
- HTTPBin Upload: `https://httpbin.org/post`
- File.io: `https://file.io`

## Common Use Cases

### 1. REST API Integration
```typescript
// Fetch data from API
const fetchPosts = async () => {
  const response = await invoke<string>('http_get', {
    url: 'https://jsonplaceholder.typicode.com/posts'
  })
  const posts = JSON.parse(response)
  return posts
}
```

### 2. Real-time Chat
```typescript
// WebSocket-based chat
const sendMessage = async (message: string) => {
  await invoke('websocket_send', {
    connectionId: wsId,
    message: JSON.stringify({
      type: 'chat',
      text: message,
      timestamp: new Date().toISOString()
    })
  })
}
```

### 3. Live Updates
```typescript
// SSE for live updates
useEffect(() => {
  const unlisten = listen('sse-message', (event) => {
    const update = JSON.parse(event.payload)
    setLiveData(update)
  })

  invoke('sse_connect', {
    url: 'https://api.example.com/events'
  })

  return () => unlisten.then(fn => fn())
}, [])
```

### 4. File Synchronization
```typescript
// Upload file to cloud storage
const syncFile = async (localPath: string) => {
  await invoke('upload_file', {
    url: 'https://api.example.com/upload',
    filePath: localPath
  })
}
```

## Error Handling

### Network Errors
```rust
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

#[tauri::command]
async fn http_get_safe(url: String) -> Result<String, ErrorResponse> {
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => Ok(body),
                Err(e) => Err(ErrorResponse {
                    error: e.to_string(),
                    code: "RESPONSE_ERROR".to_string(),
                }),
            }
        }
        Err(e) => {
            let code = if e.is_timeout() {
                "TIMEOUT"
            } else if e.is_connect() {
                "CONNECTION_ERROR"
            } else {
                "NETWORK_ERROR"
            };

            Err(ErrorResponse {
                error: e.to_string(),
                code: code.to_string(),
            })
        }
    }
}
```

## Performance Optimization

### Connection Pooling
```rust
use once_cell::sync::Lazy;

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
});

#[tauri::command]
async fn optimized_http_get(url: String) -> Result<String, String> {
    let response = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.text().await.map_err(|e| e.to_string())
}
```

### Request Caching
```rust
use std::collections::HashMap;
use std::sync::Mutex;

struct CacheEntry {
    data: String,
    timestamp: SystemTime,
}

static CACHE: Lazy<Mutex<HashMap<String, CacheEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[tauri::command]
async fn cached_http_get(url: String, cache_duration: u64) -> Result<String, String> {
    let mut cache = CACHE.lock().unwrap();

    if let Some(entry) = cache.get(&url) {
        if entry.timestamp.elapsed().unwrap().as_secs() < cache_duration {
            return Ok(entry.data.clone());
        }
    }

    drop(cache);

    let response = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    let mut cache = CACHE.lock().unwrap();
    cache.insert(url, CacheEntry {
        data: body.clone(),
        timestamp: SystemTime::now(),
    });

    Ok(body)
}
```

## Troubleshooting

### Common Issues

**HTTP requests fail**
- Check internet connection
- Verify URL is correct and accessible
- Check CORS headers (for web targets)
- Verify SSL certificate is valid
- Check firewall settings

**WebSocket connection drops**
- Implement auto-reconnection logic
- Use ping/pong heartbeat
- Check network stability
- Verify server supports WebSocket protocol

**File upload fails**
- Check file size limits
- Verify file exists and is readable
- Check server upload limits
- Verify multipart form encoding

**CORS errors (Desktop)**
- Desktop apps don't have CORS restrictions
- If seeing CORS errors, check server configuration
- May indicate server-side issue

## Resources

### Official Documentation
- [Tauri HTTP Plugin](https://v2.tauri.app/plugin/http/)
- [Tauri WebSocket Plugin](https://v2.tauri.app/plugin/websocket/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [WebSocket Protocol](https://datatracker.ietf.org/doc/html/rfc6455)

### Libraries & Tools
- [Reqwest](https://github.com/seanmonstar/reqwest) - Rust HTTP client
- [Tokio](https://tokio.rs/) - Async runtime
- [tungstenite](https://github.com/snapview/tungstenite-rs) - WebSocket library
- [Postman](https://www.postman.com/) - API testing

## Next Steps

1. **Install Required Plugins**
   - Add WebSocket plugin
   - Add HTTP plugin
   - Configure reqwest if needed

2. **Implement Basic Features**
   - HTTP GET/POST requests
   - WebSocket connection
   - File upload functionality

3. **Add Error Handling**
   - Network error handling
   - Timeout handling
   - Retry logic

4. **Implement Advanced Features**
   - Connection pooling
   - Request caching
   - Auto-reconnection
   - Progress tracking

5. **Test Thoroughly**
   - Test with public APIs
   - Test WebSocket connections
   - Test file uploads
   - Test error scenarios

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| HTTP Requests | ✅ | ✅ | ✅ | ✅ | ✅ |
| WebSocket | ✅ | ✅ | ✅ | ✅ | ✅ |
| File Upload | ✅ | ✅ | ✅ | ✅ | ✅ |
| SSE | ✅ | ✅ | ✅ | ✅ | ✅ |
| Custom Headers | ✅ | ✅ | ✅ | ✅ | ✅ |

---

**Last Updated**: November 2025
**Module Version**: 1.0.0
**Status**: Documentation Complete ✅
