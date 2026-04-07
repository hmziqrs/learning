# Usage Guide for ReqForge CLI

This guide shows how to use the ReqForge CLI tool to execute HTTP requests.

## Quick Start

1. **Navigate to the project root:**
   ```bash
   cd /path/to/gpui
   ```

2. **Build the CLI tool:**
   ```bash
   cargo build -p reqforge-cli --release
   ```

3. **Run a request:**
   ```bash
   cargo run -p reqforge-cli -- examples/basic-get-request.json
   ```

## Basic Commands

### Execute a Simple GET Request

```bash
cargo run -p reqforge-cli -- examples/basic-get-request.json
```

Output:
```
Executing request: GET https://jsonplaceholder.typicode.com/posts/1

=== Response ===

Status: 200 OK
Size: 312 bytes
Elapsed: 123ms

--- Headers ---
Content-Type: application/json; charset=utf-8
Content-Length: 312
Date: Mon, 15 Jan 2024 10:00:00 GMT
Server: GitHub.com

--- Body ---
{
  "userId": 1,
  "id": 1,
  "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
  "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
}
```

### Execute a POST Request

```bash
cargo run -p reqforge-cli -- examples/json-post-request.json
```

### Execute a Form POST Request

```bash
cargo run -p reqforge-cli -- examples/form-post-request.json
```

### Execute a Complex Request

```bash
cargo run -p reqforge-cli -- examples/complex-request.json
```

## Using Environment Variables

Create a request that uses environment variables:

1. **Set environment variables:**
   ```bash
   export BASE_URL=https://api.example.com
   export API_KEY=your-secret-api-key
   ```

2. **Create a request file with variable interpolation:**
   ```json
   {
     "name": "API with Variables",
     "method": "GET",
     "url": "{{BASE_URL}}/users",
     "headers": [
       {
         "key": "Authorization",
         "value": "Bearer {{API_KEY}}",
         "enabled": true
       }
     ],
     "query_params": [],
     "body": "None"
   }
   ```

3. **Execute the request:**
   ```bash
   cargo run -p reqforge-cli -- your-request-file.json
   ```

## Custom Requests

You can create your own request JSON files with the following structure:

```json
{
  "name": "Your Request Name",
  "method": "GET",
  "url": "https://api.example.com/endpoint",
  "headers": [
    {
      "key": "Header-Name",
      "value": "Header-Value",
      "enabled": true,
      "description": "Optional description"
    }
  ],
  "query_params": [
    {
      "key": "param",
      "value": "value",
      "enabled": true,
      "description": "Optional description"
    }
  ],
  "body": "None"
}
```

For POST/PUT requests with JSON body:

```json
{
  "name": "JSON Request",
  "method": "POST",
  "url": "https://api.example.com/endpoint",
  "headers": [
    {
      "key": "Content-Type",
      "value": "application/json",
      "enabled": true
    }
  ],
  "query_params": [],
  "body": {
    "type": "Raw",
    "content": "{\"key\": \"value\"}",
    "content_type": "Json"
  }
}
```

For form data:

```json
{
  "name": "Form Request",
  "method": "POST",
  "url": "https://api.example.com/endpoint",
  "body": {
    "type": "FormUrlEncoded",
    "fields": [
      {
        "key": "username",
        "value": "user123",
        "enabled": true
      },
      {
        "key": "password",
        "value": "pass123",
        "enabled": true
      }
    ]
  }
}
```

## Available Methods

- `GET` - Retrieve data
- `POST` - Create data
- `PUT` - Update data
- `DELETE` - Delete data
- `PATCH` - Partial update
- `HEAD` - Get headers only
- `OPTIONS` - Get available methods

## Tips

1. **Use descriptive names** for your requests to keep them organized
2. **Enable/disable headers/params** by setting `enabled` to `true` or `false`
3. **Use environment variables** to avoid hardcoding sensitive information
4. **Add descriptions** to headers and parameters for documentation
5. **Test with simple requests** first before complex ones

## Troubleshooting

### Common Issues

1. **JSON parse errors**: Check your JSON syntax using a JSON validator
2. **Connection errors**: Verify the URL and network connectivity
3. **Authentication errors**: Check your API keys and headers
4. **Missing environment variables**: Ensure all required variables are set

### Debug Mode

Set the `DEBUG` environment variable to see more details:

```bash
export DEBUG=true
cargo run -p reqforge-cli -- examples/basic-get-request.json
```

## Next Steps

- Explore the `data/collections/` directory for more complex examples
- Try the API example collection to see environment variable usage
- Create your own collection of frequently used requests