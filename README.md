# RustHTTP

A from-scratch HTTP/1.1 server written in Rust. No web frameworks — just TCP sockets, manual byte-level parsing, and the standard library + `tokio` for async I/O.

The point isn't to ship yet another web server; it's to understand what `axum`, `actix`, or Go's `net/http` are actually doing underneath. Every layer (request line parsing, header decoding, response serialization, the read buffer, the routing table) is hand-written so the bytes on the wire are never abstracted away.

## What's in here

- **TCP accept loop** — `tokio::net::TcpListener` on `127.0.0.1:7878`.
- **Custom buffered reader** — a hand-rolled `AsyncRead` + `AsyncBufRead` implementation (`reader/reader.rs`) that owns its own 1 KiB buffer and drives `poll_read` directly. This is the part that teaches you what `BufReader` is doing.
- **HTTP/1.1 parser** — reads the request line, parses headers into a `HashMap`, and pulls the body using `Content-Length`.
- **Typed request/response** — `Request`, `Response`, `RequestMethod`, and `StatusCode` types serialize back to bytes.
- **Pluggable handlers** — register a handler per `(method, path)` with `Server::attach_handler`.
- **Middleware hooks** — async closures that run before the matched handler.

## Running

```bash
cd rust_http
cargo run
```

```bash
curl -v http://localhost:7878/api
```

## Layout

```
rust_http/src/
├── main.rs          # wires up the server, registers handlers/middleware
├── server/          # accept loop + dispatch
├── handler/         # Handler struct (method + path + closure)
├── http/parser.rs   # request line, header, and body parsing
├── reader/reader.rs # custom AsyncRead/AsyncBufRead wrapper
├── schemas/         # Request, Response, StatusCode, RequestMethod
└── utils/           # debug printing helpers
```
