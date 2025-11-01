# API Testing Guide

This document contains a series of curl commands to test the todo application functionality and observability features.

## Prerequisites

Start all services before running these tests:

```bash
# Start supporting services (PostgreSQL, Redis, QuickWit, Prometheus)
docker-compose up -d

# Wait for services to be ready (30-60 seconds)
# Then start the Rust application
cargo run
```

The application will be available at `http://localhost:3000`

## 1. Health Check Tests

### Basic Health Check
```bash
curl -i http://localhost:3000/health
```
*Expected:* HTTP 200 with JSON response containing service status and version

### QuickWit Health Check
```bash
curl -i http://localhost:3000/health/quickwit
```
*Expected:* HTTP 200 with QuickWit service status

### Prometheus Health Check
```bash
curl -i http://localhost:3000/health/prometheus
```
*Expected:* HTTP 200 with Prometheus service status

### Metrics Endpoint
```bash
curl -i http://localhost:3000/metrics
```
*Expected:* HTTP 200 with Prometheus-formatted metrics

## 2. CRUD Operations Tests

### Create Todo #1
```bash
curl -i -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust",
    "description": "Complete the Rust book and build projects"
  }'
```
*Expected:* HTTP 200 with created todo object including UUID and timestamps

### Create Todo #2
```bash
curl -i -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Setup Observability",
    "description": "Configure OpenTelemetry, QuickWit, and Prometheus"
  }'
```
*Expected:* HTTP 200 with second todo object

### Get Todo #1 (Cache Miss)
```bash
# Replace {ID_1} with the actual UUID from first todo creation
curl -i http://localhost:3000/todos/{ID_1}
```
*Expected:* HTTP 200 with todo details (first request = cache miss)

### Get Todo #1 (Cache Hit)
```bash
# Run the same request again
curl -i http://localhost:3000/todos/{ID_1}
```
*Expected:* HTTP 200 with todo details (second request = cache hit)

### Get Todo #2 (Cache Miss)
```bash
# Replace {ID_2} with the actual UUID from second todo creation
curl -i http://localhost:3000/todos/{ID_2}
```
*Expected:* HTTP 200 with second todo details

### Update Todo #1
```bash
curl -i -X PUT http://localhost:3000/todos/{ID_1} \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rust (Updated)",
    "completed": true
  }'
```
*Expected:* HTTP 200 with updated todo object

### Get Non-existent Todo
```bash
curl -i http://localhost:3000/todos/00000000-0000-0000-0000-000000000000
```
*Expected:* HTTP 404 Not Found

### Delete Todo #2
```bash
curl -i -X DELETE http://localhost:3000/todos/{ID_2}
```
*Expected:* HTTP 204 No Content

### Verify Todo #2 Deleted
```bash
curl -i http://localhost:3000/todos/{ID_2}
```
*Expected:* HTTP 404 Not Found

## 3. Cache Management Tests

### Delete Cache Entry for Todo #1
```bash
curl -i -X DELETE http://localhost:3000/cache/todo/{ID_1}
```
*Expected:* HTTP 200 with cache deletion confirmation

### Get Todo #1 (Cache Miss After Deletion)
```bash
curl -i http://localhost:3000/todos/{ID_1}
```
*Expected:* HTTP 200 with todo details (cache miss, fetched from database)

### Delete Non-existent Cache Entry
```bash
curl -i -X DELETE http://localhost:3000/cache/todo/00000000-0000-0000-0000-000000000000
```
*Expected:* HTTP 200 with "not_found" status

## 4. Observability Tests

### QuickWit Search (Logs)
```bash
curl -i -X POST http://localhost:3000/quickwit/search \
  -H "Content-Type: application/json" \
  -d '{
    "q": "service.name: todo-app",
    "index": "otel-log-v0"
  }'
```
*Expected:* HTTP 200 with search results from logs

### QuickWit Search (Traces)
```bash
curl -i -X POST http://localhost:3000/quickwit/search \
  -H "Content-Type: application/json" \
  -d '{
    "q": "operation_name: create_todo",
    "index": "otel-trace-v0"
  }'
```
*Expected:* HTTP 200 with search results from traces

## 5. Error Handling Tests

### Create Invalid Todo (Missing Title)
```bash
curl -i -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Missing title field"
  }'
```
*Expected:* HTTP 400/500 (validation error)

### Update Non-existent Todo
```bash
curl -i -X PUT http://localhost:3000/todos/00000000-0000-0000-0000-000000000000 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "This should fail"
  }'
```
*Expected:* HTTP 404 Not Found

### Delete Non-existent Todo
```bash
curl -i -X DELETE http://localhost:3000/todos/00000000-0000-0000-0000-000000000000
```
*Expected:* HTTP 404 Not Found

## 6. Load Testing (Optional)

### Create Multiple Todos Rapidly
```bash
for i in {1..10}; do
  curl -s -X POST http://localhost:3000/todos \
    -H "Content-Type: application/json" \
    -d "{
      \"title\": \"Load Test Todo $i\",
      \"description\": \"Generated during load testing\"
    }" | jq '.id'
done
```

### Concurrent Cache Hits
```bash
# Replace {ID} with an actual todo ID
for i in {1..5}; do
  curl -s -w "Time: %{time_total}s, Status: %{http_code}\n" \
    -o /dev/null \
    http://localhost:3000/todos/{ID} &
done
wait
```

## 7. Validation Steps

After running the tests, validate the observability stack:

### Check QuickWit UI
```bash
# Open in browser
open http://localhost:7280/ui
```
*Look for:*
- Logs in `otel-log-v0` index
- Traces in `otel-trace-v0` index
- Spans for HTTP handlers, database operations, Redis calls

### Check Prometheus UI
```bash
# Open in browser
open http://localhost:9090
```
*Look for metrics like:*
- `http_requests_total{handler="create_todo"}`
- `http_requests_total{handler="get_todo"}`
- Cache hit/miss counters
- Database query latency histograms

### Verify Metrics Collection
```bash
curl -s http://localhost:3000/metrics | grep -E "(http_requests|cache|db_query)"
```

## 8. Cleanup Commands

### Stop All Services
```bash
# Stop application (Ctrl+C) then:
docker-compose down
```

### Remove Docker Volumes (Optional)
```bash
docker-compose down -v
```

## Notes

1. **Replace Placeholders**: Replace `{ID_1}` and `{ID_2}` with actual UUIDs returned from creation requests
2. **JSON Formatting**: All JSON payloads are pretty-printed for readability; remove newlines for actual curl commands if needed
3. **Timing**: Some commands include timing information (`-w` flag) to observe cache vs database performance
4. **jq Dependency**: Some commands use `jq` for JSON parsing; install with `brew install jq` on macOS
5. **Port Conflicts**: Ensure ports 3000, 5432, 6379, 7280, 7281, and 9090 are available

## Expected Observability Signals

Successful execution should generate:
- **Traces**: HTTP request spans → Database query spans → Redis operation spans
- **Logs**: Structured logs with todo IDs, cache hit/miss status, operation results
- **Metrics**: Request counters, response time histograms, cache performance metrics
- **Health Monitoring**: Periodic health check spans for QuickWit and Prometheus