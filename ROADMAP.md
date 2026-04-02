# Pardus Browser Roadmap

## Project Status Summary

**Current Version:** 0.4.0-dev  
**Branch:** dev/interect  
**Last Updated:** April 2026

---

## ✅ Completed (Stable)

### Core Engine
- [x] **Semantic tree output** — ARIA roles, headings, landmarks, interactive elements
- [x] **Navigation graph** — Internal routes, external links, form descriptors with field metadata
- [x] **Multiple output formats** — Markdown (default), tree, JSON
- [x] **Interactive-only mode** — Strip static content, show only actionable elements
- [x] **Action annotations** — navigate, click, fill, toggle, select tags on elements
- [x] **Fast HTTP parsing** — GET + HTML parse typically under 200ms
- [x] **Zero Chrome dependencies** — Pure Rust, no browser binary needed

### Page Interaction
- [x] **Click actions** — Link navigation, button clicks with form auto-detection
- [x] **Form handling** — Text input, checkboxes, radio buttons, selects
- [x] **Form submission** — Automatic CSRF token collection, POST/GET submission
- [x] **Wait for selectors** — Polled re-fetch until element appears
- [x] **Scroll pagination** — Detects `?page=`, `?offset=`, `/page/N` patterns

### JavaScript Engine
- [x] **V8 integration** — deno_core with custom DOM operations
- [x] **35+ Rust DOM ops** — Bridging JavaScript ↔ Rust DOM (querySelector, Element API, classList, dataset, style proxies)
- [x] **Thread-based timeout** — Prevents infinite loops via execution limits
- [x] **Inline script execution** — Automatic execution of inline `<script>` tags
- [x] **Script filtering** — Problematic scripts automatically filtered to prevent hangs

### Security
- [x] **SSRF protection** — `UrlPolicy` validates all fetched URLs
  - Blocks private IPs (10.x, 172.16-31.x, 192.168.x), loopback, link-local (169.254.x), multicast
  - Blocks cloud metadata endpoints (AWS 169.254.169.254, GCP, Azure, Alibaba)
  - Blocks non-HTTP(S) schemes (file://, ftp://, data://, javascript:)
  - Configurable modes: default, permissive, allowlist
  - Wired into BrowserConfig and JS fetch API
  - IPv4 and IPv6 validation with bracket notation support

### Session & State Management
- [x] **Session persistence** — Cookies, localStorage, auth headers with size limits
- [x] **Custom headers** — Authentication and custom header support
- [x] **Cache management** — `clean` command for cookies/cache wiping
- [x] **Persistent REPL** — Interactive session with history and persistent browser state

### Tab Management
- [x] **Multi-tab support** — Multiple tabs with independent state
- [x] **History navigation** — Back/forward per tab
- [x] **Tab activation** — Switch between active tabs
- [x] **Tab info/list** — View open tabs and current state

### Network & Debugging
- [x] **Network debugger** — DevTools-style request table with subresource discovery
- [x] **Parallel fetching** — Stylesheets, scripts, images fetched concurrently
- [x] **Request recording** — Full HTTP request/response logging
- [x] **Subresource discovery** — Automatic parsing of CSS/JS/image references

### Server-Sent Events
- [x] **SSE parser** — Streaming parser per HTML Living Standard (BOM stripping, multi-line data, chunked input, 48 tests)
- [x] **SSE client** — Async client on dedicated 8-thread tokio background runtime with auto-reconnect
- [x] **SSE manager** — Thread-safe connection manager (DashMap) with open/close/poll/drain
- [x] **JS EventSource API** — Web-standard `EventSource` class with `MessageEvent`, event callbacks
- [x] **4 deno_core ops** — `op_sse_open`, `op_sse_close`, `op_sse_ready_state`, `op_sse_url`
- [x] **Auto-reconnect** — Exponential backoff (max 5 attempts, 30s cap), honors `retry:` field
- [x] **Last-Event-ID** — Sent on reconnect for gapless event streams
- [x] **SSRF protection** — UrlPolicy validates SSE URLs (blocks private IPs, loopback, metadata)
- [x] **88 unit tests** — Parser (48), client with local TCP servers (17), manager (20), url_policy (3)

### CDP (Chrome DevTools Protocol)
- [x] **WebSocket server** — CDP endpoint on ws://127.0.0.1:9222
- [x] **14 domain handlers:**
  - [x] Browser — Version info, permission management
  - [x] Target — Tab creation, attachment, destruction
  - [x] Page — Navigation, reload, screenshot hooks
  - [x] DOM — Node tree, query selectors, node highlighting
  - [x] Network — Request/response interception, events
  - [x] Runtime — Script execution, console API
  - [x] Input — Mouse/keyboard event dispatch
  - [x] CSS — Stylesheet inspection, computed styles
  - [x] Console — Console message capture
  - [x] Log — Log entry events
  - [x] Security — Certificate info, security state
  - [x] Emulation — Viewport, user agent, touch emulation
  - [x] Performance — Metrics collection
  - [x] Pardus — Custom domain for Pardus-specific features
- [x] **Event bus** — Real-time events to CDP clients
- [x] **Node mapping** — backendNodeId ↔ selector translation

### CLI & UX
- [x] **8 subcommands:** navigate, interact, serve, repl, tab, map, clean
- [x] **Rustyline integration** — History, line editing in REPL
- [x] **Shell-word parsing** — Proper argument handling in REPL
- [x] **Verbose logging** — Debug output via tracing

---

## 🔧 In Progress / Needs Polish

### CDP Integration
- [~] **CDP → Browser API migration** — Wiring CDP handlers through unified `Browser` type
  - Status: DomainContext holds App reference, needs Browser integration
  - Blocker: `!Send` constraint from scraper types in Page
  - Workaround: Storing raw HTML in TargetEntry instead of parsed Page

### JavaScript Interactions
- [x] **JS-level interaction** — Click/type/scroll/submit via deno_core DOM when JS enabled
  - All 4 interaction methods (`click`, `type_text`, `submit`, `scroll`) branch on `js_enabled`
  - Click dispatches click event, detects `window.location.href` navigation via Proxy setter
  - Type sets value attribute, dispatches `input` + `change` events
  - Submit dispatches `submit` event, respects `preventDefault`, falls back to HTTP if not prevented
  - Scroll dispatches `scroll` + `wheel` events for direction-aware handlers
  - Inline `on*` handlers (onclick, onchange, onsubmit, etc.) auto-registered before interaction
  - DOM mutations serialized back to `Page` after each interaction
  - Ephemeral per-interaction V8 runtime (no `!Send` constraint issues)
  - Unit tests for click dispatch, inline handlers, type+change events, scroll, navigation detection, submit preventDefault

### WebSocket
- [x] **WebSocket connection** — `WebSocketConnection` wrapping tokio-tungstenite for WS/WSS
  - `connect()`, `send_text()`, `send_binary()`, `recv()`, `close()` with TLS support
  - Automatic Ping/Pong handling, frame-level statistics
  - UrlPolicy validation on connect
- [x] **WebSocket manager** — `WebSocketManager` for connection pooling
  - Per-origin connection limits
  - CDP event emission
  - Lifecycle management

---

## 📋 Planned (Near-term)

### Proxy Support
- [ ] **HTTP proxy** — Basic CONNECT tunneling
- [ ] **SOCKS5 proxy** — Full SOCKS5 client support
- [ ] **Proxy authentication** — Username/password auth
- [ ] **Per-request proxy** — `--proxy` flag on navigate

### Screenshots (Optional)
- [ ] **HTML→PNG rendering** — For when pixels actually matter
- [ ] **Element screenshots** — Capture specific element bounds
- [ ] **Viewport clipping** — Configurable resolution
- [ ] **CDP screenshot API** — Page.captureScreenshot compliance

---

## 🚀 Future Roadmap (2026+)

### AI Agent Features
- [ ] **LLM-friendly output** — Optimized token formats for common LLM context windows
- [ ] **Action planning** — Suggested next actions based on page state
- [ ] **Auto-form filling** — AI-guided form completion with validation
- [ ] **Smart wait conditions** — Wait for "content loaded" not just selectors
- [ ] **Session recording** — Replayable action sequences

### Performance & Scale
- [x] **Connection pooling** — Reuse TCP connections across requests
- [x] **HTTP/2 push** — Client-side push simulation (early `<head>` scanning + speculative fetch) and optional h2 PUSH_PROMISE reception
- [x] **Caching layer** — HTTP cache compliance (ETag, Last-Modified)
  - RFC 7234 CachePolicy with Cache-Control, ETag, Last-Modified, Expires, Age, Date header parsing
  - Freshness lifetime: max-age, Expires, heuristic (10% LM-factor per RFC 7234 §4.2.2), immutable
  - Conditional requests: If-None-Match (ETag), If-Modified-Since (Last-Modified), 304 Not Modified handling
  - Cache-aware page loading, resource scheduler, JS fetch API (with cache modes: default/no-store/force-cache/only-if-cached), prefetcher
  - Disk cache with HTTP semantics (no-store eviction, stale entry priority eviction)
  - Shared HTTP client factory eliminating duplicate reqwest::Client construction
  - `from_cache` flag on NetworkRecord for observability
- [ ] **Request deduplication** — Avoid parallel fetches of same resource
- [x] **Memory limits** — Configurable per-tab memory caps

### Web Standards
- [x] **WebSocket support** — Full WS/WSS protocol handling with `WebSocketManager` and `WebSocketConnection`
  - `WebSocketConnection`: `connect()`, `send_text()`, `send_binary()`, `recv()`, `close()` methods
  - `WebSocketManager`: Connection pooling, per-origin limits, lifecycle management
  - SSRF protection: Blocks private IPs, loopback, link-local, cloud metadata endpoints
  - CDP events: `Network.webSocketCreated`, `Network.webSocketClosed`, `Network.webSocketFrameSent`, `Network.webSocketFrameReceived`
  - IPv6 support: Blocks loopback (`::1`), link-local (`fe80::/10`), unique local (`fc00::/7`)
  - 30 unit tests covering security, lifecycle, event bus, and URL validation
- [x] **EventSource/SSE** — Full SSE implementation: parser (48 tests), async client with auto-reconnect, thread-safe manager, JS EventSource Web API, 4 deno_core ops, SSRF protection
- [ ] **Shadow DOM** — Piercing shadow boundaries for web components
- [ ] **IFrame handling** — Recursive frame parsing and interaction
- [ ] **PDF viewing** — PDF.js-style rendering or extraction

### Security & Authentication
- [x] **SSRF protection** — URL policy blocking private IPs, metadata endpoints, non-HTTP schemes
- [ ] **Basic auth** — 401 response handling
- [ ] **OAuth flow** — OAuth 2.0 / OIDC automation helpers
- [ ] **Certificate pinning** — Custom CA/cert validation
- [ ] **CSP compliance** — Content Security Policy enforcement
- [ ] **Sandbox mode** — Restricted execution for untrusted content

### API & Integration
- [ ] **Python bindings** — PyO3 wrapper for Python agents
- [ ] **Node.js bindings** — N-API for JavaScript agents
- [ ] **Playwright adapter** — Drop-in replacement compatibility layer
- [ ] **Puppeteer adapter** — API compatibility for migration
- [ ] **Docker image** — Official container with health checks

### Developer Experience
- [ ] **HAR export** — HTTP Archive format for request logs
- [ ] **Coverage reporting** — CSS/JS usage statistics
- [ ] **Accessibility audit** — Automated a11y checks
- [ ] **Visual regression** — Diff screenshots for testing
- [ ] **REPL improvements** — Auto-completion, syntax highlighting

---

## 📊 Metrics & Targets

| Metric | Current | Target |
|--------|---------|--------|
| Cold start | ~50ms | ~30ms |
| Page parse (typical) | ~150ms | ~100ms |
| JS execution timeout | 3s | Configurable |
| CDP domains | 14/20 | 20/20 |
| Test coverage | ~60% | 85% |
| Binary size | ~15MB | <10MB |

---

## 🐛 Known Issues

| Issue | Status | Workaround |
|-------|--------|------------|
| External scripts not executed | By design | Only inline scripts supported |
| setTimeout/setInterval no-ops | By design | Prevents infinite loops |
| CDP DOM methods use raw HTML | Fixed | Parse on-demand from stored HTML |
| Complex SPA interactions | Partial | Use `--wait-ms` for async content |

---

## 📝 Changelog

### v0.4.0 — WebSocket Full Implementation

**WebSocket Support:**
- Added `WebSocketConnection` module (`crates/pardus-core/src/websocket/connection.rs`)
  - Async connect with configurable timeout
  - `send_text()`, `send_binary()` for outgoing messages
  - `recv()` returns `(WebSocketFrame, Vec<u8>)` for incoming messages
  - Automatic Ping/Pong handling
  - Connection statistics tracking (frames sent/received, bytes)
  - Unique connection ID generation via URL hashing

- Added `WebSocketManager` module (`crates/pardus-core/src/websocket/manager.rs`)
  - Connection pooling with per-origin limits (`max_per_origin`)
  - Configurable security policy (`block_private_ips`, `block_loopback`)
  - CDP event bus integration for real-time notifications
  - Event emission: `Network.webSocketCreated`, `Network.webSocketClosed`, `Network.webSocketFrameSent`, `Network.webSocketFrameReceived`

- Added `WebSocketConfig` for connection settings
  - `max_per_origin`: Maximum concurrent connections per origin (default: 6)
  - `connect_timeout_secs`: Connection timeout (default: 30s)
  - `max_message_size`: Maximum message size (default: 10MB)
  - `block_private_ips`: Block private IP addresses (default: true)
  - `block_loopback`: Block loopback addresses (default: true)

- SSRF Protection for WebSocket
  - Blocks private IPv4: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
  - Blocks IPv6 unique local: fc00::/7
  - Blocks IPv6 link-local: fe80::/10
  - Blocks IPv6 loopback: ::1
  - Blocks cloud metadata: metadata.google.internal, 169.254.169.254, 100.100.100.200
  - Blocks localhost hostname

- Added `ResourceType::WebSocket` to `pardus-debug` crate

- Dependencies added:
  - `tokio-tungstenite = "0.26"` — Async WebSocket client
  - `tungstenite = "0.26"` — WebSocket protocol

- Test coverage: 30 unit tests
  - Config tests (2)
  - Manager tests (15)
  - IPv6 tests (3)
  - Event bus tests (2)
  - URL validation tests (5)
  - Connection limit tests (2)
  - Permissive policy tests (1)

### v0.3.0 — SSE, WebSocket, SSRF Protection

**Server-Sent Events (EventSource):**
- Added `SseParser` — streaming SSE parser per HTML Living Standard (BOM stripping, multi-line data, chunked input, 48 tests)
- Added `SseClient` — async SSE connection with reqwest, runs on dedicated 8-thread tokio background runtime
- Added `SseManager` — thread-safe connection manager (DashMap) with `open`/`close`/`drain_events_js`
- Auto-reconnect with exponential backoff (max 5 attempts, 30s cap), honors server `retry:` field
- `Last-Event-ID` header sent on reconnect for gapless streams
- SSRF protection via `UrlPolicy` — blocks private IPs, loopback, metadata endpoints, file:// scheme
- 4 deno_core ops: `op_sse_open`, `op_sse_close`, `op_sse_ready_state`, `op_sse_url`
- `drain_events_js()` generates JS dispatch code consumed by runtime event loop
- `EventSource` Web API in bootstrap.js (CONNECTING/OPEN/CLOSED states, onopen/onmessage/onerror, addEventListener)
- `MessageEvent` class, `__sse_dispatch` global for Rust→JS event dispatch
- SSE event drain phase in `js/runtime.rs` after each event loop poll
- `spawn_sse_connection_on()` for testability (decouples runtime from connection spawning)
- 88 unit tests: 48 parser, 17 client (with local TCP test servers), 20 manager, 3 url_policy

**WebSocket (Full Implementation):**
- Added `WebSocketConnection` — wraps tokio-tungstenite for WS/WSS with TLS support
- `connect()`, `send_text()`, `send_binary()`, `recv()`, `recv_text()`, `close()` API
- Automatic Ping/Pong handling, frame-level statistics (`WebSocketStats`)
- UrlPolicy validation on connect, connection ID via blake3 hash
- Added `WebSocketManager` — connection pooling with per-origin limits
- CDP event emission: `Network.webSocketCreated`, `Network.webSocketClosed`, `Network.webSocketFrameSent`, `Network.webSocketFrameReceived`
- SSRF protection: Blocks private IPs (RFC 1918), loopback, link-local (169.254.x.x, fe80::/10), cloud metadata endpoints
- IPv6 support: Blocks loopback (::1), link-local (fe80::/10), unique local (fc00::/7)
- 30 unit tests covering security, lifecycle, event bus, URL validation

**SSRF Protection:**
- Added `UrlPolicy` — validates all URLs before fetching
- Blocks private IPs (10.x, 172.16-31.x, 192.168.x), loopback, link-local (169.254.x), multicast
- Blocks cloud metadata endpoints (AWS 169.254.169.254, GCP, Azure, Alibaba 100.100.100.200)
- Blocks non-HTTP(S) schemes (file://, ftp://, data://, javascript:)
- Three modes: `default()` (strict), `permissive()` (localhost allowed), `allowlist()`
- Wired into `BrowserConfig` and JS `fetch()` API (15 unit tests)

**Interceptor Pipeline:**
- Fixed `run_before_request` to compose Redirect/Mock across all interceptors (previously returned on first match)

**Preload Scanner Rewrite:**
- Replaced single `RegexSet` with 6 per-tag-type classifiers (`classify_link`, `classify_script`)
- Proper CORS attribute extraction (`crossorigin`, `use-credentials`, `anonymous`)
- `modulepreload` recognition, better priority classification (preload/modulepreload → High, async/defer → Low)

**Streaming Parser Simplification:**
- Removed `lol_html` dependency; now uses regex-based `PreloadScanner` + `LazyDom`

**Cache:**
- `CachePolicy::is_fresh()` returns `true` for `immutable` resources (bypasses freshness lifetime calculation)

**Navigation Graph:**
- Added `Clone` + `Deserialize` derives to `NavigationGraph`, `Route`, `FormDescriptor`, `FieldDescriptor`

**Parser:**
- `LazyDom`: added `from_bytes()` (infallible), `Default` impl, fixed `select()` lifetime, removed `Send + Sync` bound
- Early scanner: relaxed image prefetch to `priority <= High` (was Critical only)

**Resource Module:**
- `FetchResult::error()` simplified to take `String`
- `CachedFetcher` wrapped in `Arc`, `PriorityQueue::peek()` returns references

**JS:**
- Fixed DOM tree bug where `child_id` return value was unused in `js/dom.rs`
- Conditional JS compilation: `#[cfg(feature = "js")]` guards on interaction methods in `browser.rs`

**Prefetcher:**
- Now takes shared `client: reqwest::Client` + `cache: Arc<ResourceCache>` (no duplicate client creation)

**Toolchain:**
- Added `rust-toolchain.toml` pinning to nightly

### v0.2.0 — CDP & Cookie Optimizations

**HTTP Caching Layer (RFC 7234):**
- Added `CachePolicy` type parsing Cache-Control (max-age, no-store, no-cache, must-revalidate, immutable), ETag, Last-Modified, Expires, Age, Date headers
- Implemented heuristic freshness: 10% of Last-Modified age (min 1s, max 24h) per RFC 7234 §4.2.2
- Conditional requests: If-None-Match / If-Modified-Since sent on stale cache entries; 304 Not Modified handled with cache header update
- Cache-aware page loading: fresh hits return immediately, stale entries revalidated, misses cached with policy
- Cache-aware resource scheduler: `CachedFetcher` with `Send`-safe async design wraps all subresource fetches
- JS fetch API cache integration: supports `cache` parameter (default, no-store, force-cache, only-if-cached); adds `x-cache` response header
- Prefetcher stores results in shared `ResourceCache`, checks freshness before network requests
- Disk cache enhanced with HTTP semantics: `CacheMeta` metadata, no-store/fast-expiry priority eviction, `insert_with_meta()`
- Shared HTTP client factory (`http/client.rs`) eliminating 5 duplicate `reqwest::Client` builders
- `CacheManager` wired into `App` and `Browser` with `resource_cache()` accessors
- `NetworkRecord` gains `from_cache: Option<bool>` field for observability
- `chrono` added as workspace dependency for HTTP date parsing

**CDP Server Hardening:**
- Fixed async safety: replaced all `blocking_lock()` calls with `.lock().await`; session lock no longer held across `.await` during command routing
- Fixed protocol compliance: error responses now carry correct request IDs; `querySelectorAll` returns unique IDs per element; `getOuterHTML` returns proper errors
- Added connection limit (default 16, configurable via `with_max_connections()`) with graceful rejection logging
- Added graceful shutdown via `CdpServer::shutdown()` method
- Added per-command timeout (30s default) with timeout error responses
- Wired HTTP discovery endpoints (`/json/version`, `/json/list`) for non-WebSocket HTTP connections
- Added target lifecycle events: `Target.targetCreated`, `Target.targetDestroyed`, `Target.attachedToTarget`, `Target.detachedFromTarget`
- Implemented `Target.closeTarget` with proper cleanup and destruction event
- Added event replay buffer (64 events) for lagged connection recovery via `EventBus::replay_events()`
- Improved NodeMap with `invalidate_on_navigation()` for safe ID reset and `get_or_assign_indexed()` for unique per-element IDs

**CDP Network (Cookies):**
- Implemented `Network.getCookies` / `Network.getAllCookies` — extracts cookies from network log Set-Cookie headers with full attribute parsing (domain, path, httpOnly, secure, sameSite, size)
- Implemented `Network.setCookie`, `Network.deleteCookies`, `Network.clearBrowserCookies`
- Added `url` crate dependency to pardus-cdp for URL parsing in cookie operations

**Cookie System (SessionStore):**
- Fixed cookie parsing bug: removed incorrect `split(';')` on Set-Cookie header values
- Switched to RFC 6265 compliant domain matching via `cookie_store::get_request_values`
- Added atomic save (temp file + rename) for session persistence
- Added `delete_cookie(name, domain, path)` method to SessionStore
- Added `session_dir()` public accessor to SessionStore

**Performance:**
- Removed unnecessary HTML re-parsing in Pardus domain click handler (reuse `page_data` result)
- Removed dead HTML clone in `RuntimeDomain::evaluate_expression`
- Fixed tab loading to use browser's actual `BrowserConfig` instead of hardcoded default
- POST form submissions now recorded in NetworkLog

**Architecture:**
- `DomainContext.get_html/get_url/get_title` converted from sync `blocking_lock()` to async `.lock().await` (safe for multi-threaded tokio runtime)
- Added `HandleResult::with_request_id()` utility for threading request IDs through error responses
- Router now injects correct `request.id` into all error responses, even from domain handlers

### v0.1.0-dev (current)
- Initial release with full feature set
- Unified Browser API
- CDP server with 14 domains
- JavaScript execution via deno_core
- Configurable per-tab memory limits
- Persistent REPL and tab management

---

*For contributing to the roadmap, open an issue with the `roadmap` label.*
