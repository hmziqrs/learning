# Plan for Building a Todo‑App with Axum, SeaORM, Redis, QuickWit and Prometheus (2025)

## Purpose and Scope

This document outlines a detailed plan for an AI agent to implement a simple **todo application** using modern Rust libraries and observability tooling available as of **1 November 2025**. The stack uses **Axum** for the web framework, **SeaORM** with **PostgreSQL** for persistence, **Redis** for caching, **QuickWit** for log and trace storage, and **Prometheus** for metrics collection. All versions referenced here are the latest stable releases found through official documentation and release notes.

The goal is to provide comprehensive guidance without code. An agent following this plan will:

- Define models and database interactions with SeaORM and Postgres.
- Implement RESTful endpoints with Axum that create, fetch, update and delete todos.
- Use Redis as a cache layer to speed up reads.
- Instrument the service with OpenTelemetry for **logs**, **traces** and **metrics**, then export logs and traces to QuickWit and metrics to Prometheus.
- Expose additional endpoints to query QuickWit and Prometheus for health or diagnostic information.

## Latest Versions (as of November 1 2025)

Selecting versions that are actively maintained is essential for security and stability. The following versions should be used in Cargo.toml and Docker manifests:

| Component | Latest version (2025‑11‑01) | Evidence | Notes |
| --- | --- | --- | --- |
| **Axum (crate)** | 0.8.6 | The documentation for the axum crate lists version 0.8.6 as the current release[\[1\]](https://docs.rs/axum/latest/axum/#:~:text=Docs). | Use this for the HTTP server and routing. |
| **SeaORM (crate)** | 1.1.17 | The SeaORM documentation shows version 1.1.17 as the latest[\[2\]](https://docs.rs/sea-orm/latest/sea_orm/#:~:text=%2A%20sea). | Use with the sqlx‑postgres feature enabled. |
| **PostgreSQL** | 18  | The PostgreSQL official site notes that **PostgreSQL 18** was released on 25 September 2025[\[3\]](https://www.postgresql.org/#:~:text=2025,Released). | Use the official postgres:18 Docker image. |
| **redis (crate)** | 0.32.7 | The docs.rs page for the redis crate lists version 0.32.7[\[4\]](https://docs.rs/redis/latest/redis/#:~:text=Docs). | Provides a Redis client for Rust. |
| **Redis (server)** | 8.2.2 | The Redis open‑source release page marks **8.2.2** as the latest stable release[\[5\]](https://github.com/redis/redis/releases#:~:text=8). | Use the redis:8.2.2 Docker image. |
| **QuickWit** | 0.8.2 | QuickWit's release page labels version 0.8.2 as the latest release[\[6\]](https://github.com/quickwit-oss/quickwit/releases#:~:text=v0). | This version supports OTLP ingestion for logs and traces. |
| **Prometheus** | 3.7.3 | The Prometheus download page lists release 3.7.3 (dated 2025‑10‑29) as the latest[\[7\]](https://prometheus.io/download/#:~:text=3.7.3%20%2F%202025). | Use the prom/prometheus:v3.7.3 image and the Prometheus Rust client for metrics. |
| **OpenTelemetry (crate)** | 0.31.0 | The OpenTelemetry Rust crate version is shown as 0.31.0[\[8\]](https://docs.rs/opentelemetry/latest/opentelemetry/#:~:text=%2A%20opentelemetry). | Provides APIs for traces, metrics and logging. |

Use these versions explicitly in your configuration to avoid mismatched dependencies. For example, Cargo.toml should depend on axum=0.8.6, sea-orm=1.1.17, redis=0.32.7, and opentelemetry=0.31.0.

## High‑Level Architecture

The application will expose a REST API to create, read, update and delete todo items. The flow between components is summarised below:

- **Client request to create a todo** → Axum handler calls SeaORM to insert a row in PostgreSQL → concurrently writes a cached representation to Redis.
- **Fetch a todo** → Axum checks Redis first. If the entry exists, return it from cache; otherwise fetch from PostgreSQL, return to client and write it into Redis.
- **Update or delete a todo** → Axum updates or deletes the row in PostgreSQL → ensures the Redis cache entry is synchronised (update or deletion).
- **Observability instrumentation** → each handler and database/cache operation is instrumented with OpenTelemetry spans, events and metrics. Logs and traces are exported to QuickWit through an OTLP exporter, while metrics are exposed to Prometheus via a /metrics endpoint.
- **Health and diagnostic endpoints** → additional routes query QuickWit and Prometheus to confirm they are reachable and return basic index or scrape status. Another endpoint allows manual deletion of a Redis cache entry for testing the instrumentation.

This design ensures strong consistency between cache and database, provides detailed observability, and uses only hard‑coded configuration for easier reproducibility.

## Environment and Tooling Setup

- **Containerised services** - Use Docker Compose to run supporting services. Pin the versions shown above:
- postgres:18 for the PostgreSQL database.
- redis:8.2.2 for caching.
- quickwit/quickwit:0.8.2 for log and trace storage. Expose ports 7280 (HTTP API and UI) and 7281 (OTLP gRPC ingestion). QuickWit will automatically create otel-trace-v0 and otel-log-v0 indices when data arrives.
- prom/prometheus:v3.7.3 for metrics scraping. Configure Prometheus to scrape the Axum service on the /metrics endpoint.
- **Rust project** - Initialise a new Rust workspace. Add dependencies at their latest versions (see table above), including tokio, tracing, tracing-subscriber, opentelemetry, opentelemetry-otlp, axum-extra for JSON extraction, serde and serde_json for data structures, and metrics crate if you prefer using the metrics API. All dependencies should be pinned to exact versions to avoid breaking changes.
- **No environment variables** - All configuration details (database URL, Redis address, QuickWit endpoint, Prometheus scrape interval, etc.) must be stored in a Rust module config.rs. Hard‑code the hostnames and ports that match your Docker Compose file. For example, PostgreSQL might live at postgres://postgres:password@localhost:5432/todos, Redis at redis://127.0.0.1:6379, QuickWit OTLP at <http://127.0.0.1:7281> and UI at <http://127.0.0.1:7280>, and the service will listen on 0.0.0.0:3000.

## Database Design with SeaORM and PostgreSQL

- **Model definition** - In SeaORM, define an entity for the todos table with fields:
- **id**: Primary key (e.g., Uuid or auto‑increment integer). SeaORM supports UUID via uuid crate if desired.
- **title**: Short text describing the task.
- **description**: Longer text explaining the task (optional).
- **status**: Enum or boolean indicating whether the todo is complete.
- **created_at** / **updated_at**: Timestamps for record creation and last update.
- **Migration** - Use SeaORM's sea-orm-cli to generate migrations. The migration creates the table with appropriate column types for PostgreSQL. The AI agent should create a migration folder and set up the necessary tables before starting the service.
- **Connection handling** - The Axum server will use a SeaORM DatabaseConnection configured with the Postgres URL defined in config.rs. Configure a connection pool (e.g., using sqlx with max_connections) to handle concurrent queries.

## Caching Strategy with Redis

Caching is used to reduce load on PostgreSQL and improve response times. The logic flows are as follows:

- **Create todo** - When a new todo is created:
- Persist the todo in PostgreSQL via SeaORM.
- Write the same todo into Redis under a key derived from its ID (e.g., todo:{id}).
- Optionally set an expiration time (TTL) to avoid stale data lingering indefinitely.
- **Fetch todo** - When retrieving a todo by ID:
- First query Redis for the key todo:{id}. If it exists, return the cached object and record a cache hit metric.
- If not found in Redis, query PostgreSQL. On success, cache the result in Redis for future requests and record a cache miss metric.
- **Update todo** - For updates (title, description or status):
- Apply the update in PostgreSQL first. Ensure the operation is committed before modifying the cache.
- After the database update succeeds, update the corresponding Redis key with the new data to maintain consistency.
- **Delete todo** - When deleting a todo:
- Delete the row in PostgreSQL.
- Delete the associated Redis cache entry (DEL todo:{id}).
- **Explicit cache deletion** - Provide a dedicated route (e.g., DELETE /cache/todo/{id}) that removes the Redis entry without affecting the database. This route is useful for verifying OpenTelemetry instrumentation and testing cache invalidation independently.

Redis operations should use a client from the redis crate. Use a shared connection or connection pool to minimise overhead. Consider serialising the todo as JSON for storage in Redis; this makes it easy to rehydrate into Rust structs.

## OpenTelemetry Instrumentation (Tracing, Logs and Metrics)

### Traces and Logs

- **Setup** - Use the opentelemetry and tracing crates to instrument the application. Install an OTLP exporter (opentelemetry-otlp) configured to send data via gRPC to QuickWit's OTLP endpoint (e.g., <http://localhost:7281>). Attach this exporter to a tracing_subscriber pipeline so that spans and events are automatically exported.
- **Span design** - Create spans for each high‑level operation:
- HTTP handlers (create_todo, get_todo, update_todo, delete_todo, delete_cache)
- Database calls (SeaORM operations)
- Redis calls (cache reads/writes/deletes)
- QuickWit and Prometheus health checks Each span should include attributes such as HTTP method, path, status code, record ID, and success/failure indicators.
- **Logs** - Use tracing::info!, tracing::error! and similar macros to record logs within spans. Each log entry will be associated with the current span and exported to QuickWit. Avoid using environment variables; logs should include explicit context (e.g., todo_id, cache_hit) passed via attributes.
- **Context propagation** - Ensure that parent-child relationships between spans are preserved. For example, the span created by an HTTP handler should be the parent of spans for database queries or Redis commands.

### Metrics

- **Collecting metrics** - Use the opentelemetry crate's metrics API or the metrics facade. Track counts such as:
- Number of todos created, updated, deleted.
- Cache hits and misses.
- HTTP request durations and sizes.
- Database and Redis latency histograms.
- **Exposing metrics** - Integrate a Prometheus exporter. The opentelemetry-prometheus crate can convert OpenTelemetry metrics into a format that Prometheus understands. Expose these metrics on a route (e.g., GET /metrics), which the Prometheus server will scrape periodically.
- **Prometheus configuration** - In the Prometheus configuration file (e.g., prometheus.yml), add a scrape job targeting the service's host and port. The default scrape interval can be configured to 15 seconds or as appropriate.

## Integrating QuickWit for Logs and Traces

QuickWit is a Rust‑native search engine that can ingest OpenTelemetry data directly via OTLP. To set up QuickWit:

- **Deployment** - Use the quickwit/quickwit:0.8.2 container. Map local volumes for storage and expose the HTTP API on port 7280 and the OTLP gRPC endpoint on port 7281.
- **Indices** - When the first trace or log is received, QuickWit automatically creates indices named otel-trace-v0 and otel-log-v0. These store spans and logs respectively.
- **Exporter configuration** - Configure the OpenTelemetry OTLP exporter in the Rust application to send both traces and logs to <http://localhost:7281>. Use insecure mode (insecure=true) since communications are local.
- **Health endpoint** - Provide an Axum route (e.g., GET /health/quickwit) that queries QuickWit's HTTP API to ensure it is running. A simple check is to call GET /api/v1/catalog or GET / on port 7280 and verify that a JSON response is returned. Capture the result in a span labelled quickwit_health_check and record success or failure as log events.
- **Diagnostics** - Optionally, implement a route that forwards a search request to QuickWit's /api/v1/otel-trace-v0/search or /api/v1/otel-log-v0/search endpoints. This allows users to test queries via the app and verify that data is stored correctly.

## Integrating Prometheus for Metrics

- **Prometheus service** - Launch the prom/prometheus:v3.7.3 container. Mount a configuration file specifying a scrape job for your Axum service. The file might define a job named todo-app with a target todo_app:3000 or localhost:3000 depending on the network setup.
- **Metrics exporter** - Include the opentelemetry-prometheus or metrics-exporter-prometheus crate in your dependencies. Register a Prometheus exporter during the service initialisation and expose a /metrics route that Prometheus can scrape. The exporter should convert counters and histograms into the Prometheus exposition format.
- **Health endpoint** - Implement a route (e.g., GET /health/prometheus) that returns static information about the metrics exporter (for example, the number of metrics currently registered or the last scrape time). Alternatively, you could fetch the metrics endpoint yourself and return a summary (e.g., HTTP 200 if metrics are accessible). Instrument this route with a span named prometheus_health_check.

## Additional Routes for Diagnostics and Cache Management

The application should provide a few extra endpoints for operational visibility:

- **GET /health** - Returns basic service status (e.g., version, uptime). Useful for infrastructure liveness checks.
- **GET /health/quickwit** - Calls QuickWit's API as described above and returns success or failure.
- **GET /health/prometheus** - Performs the Prometheus health check.
- **DELETE /cache/todo/{id}** - Removes the Redis entry for the specified todo ID. This does not delete the record from PostgreSQL. Use this route to observe how the cache deletion appears in logs and traces. Instrument it with an appropriate span and log whether the key existed.
- **GET /metrics** - Exposes Prometheus metrics in the appropriate format. While not considered a health endpoint, it is crucial for Prometheus scraping.
- **GET /quickwit/search** (optional) - Accepts query parameters to search logs or traces directly via QuickWit. This route acts as a thin proxy, making it easier to query telemetry without using QuickWit's UI.

Each of these routes should return structured JSON responses and include OpenTelemetry instrumentation so that requests to them generate traces and logs.

## Instrumentation Testing and Validation

To ensure the observability pipeline is functioning:

- **Manual tests** - After starting all services, create, fetch, update and delete a few todo items via HTTP requests (e.g., using curl or Postman). Watch the QuickWit UI at <http://localhost:7280/ui> to confirm that traces and logs appear in otel-trace-v0 and otel-log-v0 indices.
- **Prometheus metrics** - Navigate to the Prometheus UI at <http://localhost:9090>. Use PromQL queries to check counters such as http_requests_total{handler="create_todo"} or histograms like db_query_duration_seconds_bucket. Ensure metrics from your service are being scraped.
- **Redis cache deletion** - Use the DELETE /cache/todo/{id} endpoint and verify that a corresponding trace appears in QuickWit. The trace should include spans for reading and deleting from Redis.
- **Health checks** - Call the QuickWit and Prometheus health endpoints; confirm that the results are recorded in QuickWit and that metrics include their latency.

## Conclusion

By following this plan, an AI agent can build a maintainable todo application that leverages modern Rust frameworks and a best‑of‑breed observability stack. The design ensures:

- **Reliable persistence** with PostgreSQL and SeaORM.
- **Efficient caching** using Redis and a clear strategy for synchronising data between the cache and the database.
- **Comprehensive observability** through OpenTelemetry instrumentation, with logs and traces stored in QuickWit[\[6\]](https://github.com/quickwit-oss/quickwit/releases#:~:text=v0) and metrics collected by Prometheus[\[7\]](https://prometheus.io/download/#:~:text=3.7.3%20%2F%202025).
- **Operational insight** via health and diagnostic routes that make it easy to monitor the system.

Always verify versions before starting new projects, as later releases may exist after November 1 2025. The citations provided above should be updated if a newer stable version becomes available.

[\[1\]](https://docs.rs/axum/latest/axum/#:~:text=Docs) axum - Rust

<https://docs.rs/axum/latest/axum/>

[\[2\]](https://docs.rs/sea-orm/latest/sea_orm/#:~:text=%2A%20sea) sea_orm - Rust

<https://docs.rs/sea-orm/latest/sea_orm/>

[\[3\]](https://www.postgresql.org/#:~:text=2025,Released) PostgreSQL: The world's most advanced open source database

<https://www.postgresql.org/>

[\[4\]](https://docs.rs/redis/latest/redis/#:~:text=Docs) redis - Rust

<https://docs.rs/redis/latest/redis/>

[\[5\]](https://github.com/redis/redis/releases#:~:text=8) Releases · redis/redis

<https://github.com/redis/redis/releases>

[\[6\]](https://github.com/quickwit-oss/quickwit/releases#:~:text=v0) Releases · quickwit-oss/quickwit

<https://github.com/quickwit-oss/quickwit/releases>

[\[7\]](https://prometheus.io/download/#:~:text=3.7.3%20%2F%202025) Download | Prometheus

<https://prometheus.io/download/>

[\[8\]](https://docs.rs/opentelemetry/latest/opentelemetry/#:~:text=%2A%20opentelemetry) opentelemetry - Rust

<https://docs.rs/opentelemetry/latest/opentelemetry/>
