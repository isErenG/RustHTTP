# rust-http

An HTTP/1.1 server built from scratch in Rust. I'm building this to learn how HTTP servers work.

## What's implemented

- TCP accept loop on `127.0.0.1:7878`
- Custom buffered async reader (shows you what `BufReader` does under the hood)
- HTTP/1.1 parser for request line, headers, and body via `Content-Length`
- `Request`, `Response`, `StatusCode`, `RequestMethod` types
- Per-route handler registration via `Server::attach_handler`
- Middleware support (async closures that run before the handler)

## Running

```bash
cargo run
curl -v http://localhost:7878/api
```
