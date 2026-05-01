# RustHTTP

A from-scratch HTTP/1.1 server written in Rust. No frameworks — just raw TCP sockets, manual byte parsing, and the standard library.

The goal is to understand how HTTP works at the byte level, and to get hands-on with Rust's ownership model and type system.

---

## Roadmap

### V1 — Bare TCP to HTTP
- Open a TCP listener on a port
- Read raw bytes from the socket
- Manually parse request line and headers
- Send a hardcoded HTTP response back

### V2 — Routing + Responses
- Simple router matching method + path to a handler
- Structured `HttpRequest` and `HttpResponse` types
- Proper status codes (200, 404, 405)
- Parse request body for POST requests

### V3 — Static File Serving
- Serve files from a `/public` directory
- Detect content type from file extension
- Return 404 if file not found

### V4 — Concurrency
- Introduce `tokio` for async I/O
- Handle multiple simultaneous connections

---

## Running

```bash
cargo run
```

```bash
curl -v http://localhost:7878
```