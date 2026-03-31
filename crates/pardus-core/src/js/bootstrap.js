// bootstrap.js — Sets up document, window, Element, fetch for pardus-browser JS runtime.

// ---- Element wrapper ----
class Element {
  constructor(nodeId) {
    this.__nodeId = nodeId;
  }
  get tagName() { return Deno.core.ops.op_get_tag_name(this.__nodeId); }
  get id() { return Deno.core.ops.op_get_node_id_attr(this.__nodeId); }
  set id(v) { Deno.core.ops.op_set_node_id_attr(this.__nodeId, v); }
  get className() { return Deno.core.ops.op_get_class_name(this.__nodeId); }
  set className(v) { Deno.core.ops.op_set_class_name(this.__nodeId, v); }
  get innerHTML() { return Deno.core.ops.op_get_inner_html(this.__nodeId); }
  set innerHTML(v) { Deno.core.ops.op_set_inner_html(this.__nodeId, v); }
  get textContent() { return Deno.core.ops.op_get_text_content(this.__nodeId); }
  set textContent(v) { Deno.core.ops.op_set_text_content(this.__nodeId, v); }
  get children() {
    return Deno.core.ops.op_get_children(this.__nodeId).map(id => new Element(id));
  }
  get childElementCount() { return Deno.core.ops.op_get_children(this.__nodeId).length; }
  get firstChild() {
    const ids = Deno.core.ops.op_get_children(this.__nodeId);
    return ids.length > 0 ? new Element(ids[0]) : null;
  }
  get lastChild() {
    const ids = Deno.core.ops.op_get_children(this.__nodeId);
    return ids.length > 0 ? new Element(ids[ids.length - 1]) : null;
  }
  get parentElement() {
    const pid = Deno.core.ops.op_get_parent(this.__nodeId);
    return pid ? new Element(pid) : null;
  }
  get nextSibling() {
    const pid = Deno.core.ops.op_get_parent(this.__nodeId);
    if (!pid) return null;
    const siblings = Deno.core.ops.op_get_children(pid);
    const idx = siblings.indexOf(this.__nodeId);
    return idx >= 0 && idx < siblings.length - 1 ? new Element(siblings[idx + 1]) : null;
  }
  appendChild(child) { Deno.core.ops.op_append_child(this.__nodeId, child.__nodeId); return child; }
  removeChild(child) { Deno.core.ops.op_remove_child(this.__nodeId, child.__nodeId); return child; }
  setAttribute(name, value) { Deno.core.ops.op_set_attribute(this.__nodeId, name, String(value)); }
  getAttribute(name) { return Deno.core.ops.op_get_attribute(this.__nodeId, name); }
  removeAttribute(name) { Deno.core.ops.op_remove_attribute(this.__nodeId, name); }
  hasAttribute(name) { return Deno.core.ops.op_get_attribute(this.__nodeId, name) !== null; }
  querySelector(selector) {
    const id = Deno.core.ops.op_query_selector(this.__nodeId, selector);
    return id ? new Element(id) : null;
  }
  querySelectorAll(selector) {
    return Deno.core.ops.op_query_selector_all(this.__nodeId, selector).map(id => new Element(id));
  }
  addEventListener(event, handler) {
    if (event === "DOMContentLoaded" || event === "load") { handler(); }
  }
  get style() {
    const nodeId = this.__nodeId;
    return new Proxy({}, {
      set(_, prop, value) { Deno.core.ops.op_set_style(nodeId, prop, String(value)); return true; }
    });
  }
}

// ---- TextNode wrapper ----
class TextNode {
  constructor(nodeId) { this.__nodeId = nodeId; }
  get textContent() { return Deno.core.ops.op_get_text_content(this.__nodeId); }
  set textContent(v) { Deno.core.ops.op_set_text_content(this.__nodeId, v); }
}

// ---- DocumentFragment wrapper ----
class DocumentFragment {
  constructor(nodeId) { this.__nodeId = nodeId; }
  appendChild(child) { Deno.core.ops.op_append_child(this.__nodeId, child.__nodeId); return child; }
  get children() { return Deno.core.ops.op_get_children(this.__nodeId).map(id => new Element(id)); }
}

// ---- Document object ----
const document = {
  createElement(tag) { return new Element(Deno.core.ops.op_create_element(tag)); },
  createTextNode(text) { return new TextNode(Deno.core.ops.op_create_text_node(text)); },
  createDocumentFragment() { return new DocumentFragment(Deno.core.ops.op_create_document_fragment()); },
  getElementById(id) { const nid = Deno.core.ops.op_get_element_by_id(id); return nid ? new Element(nid) : null; },
  querySelector(selector) { const nid = Deno.core.ops.op_query_selector(0, selector); return nid ? new Element(nid) : null; },
  querySelectorAll(selector) { return Deno.core.ops.op_query_selector_all(0, selector).map(id => new Element(id)); },
  get documentElement() { return new Element(Deno.core.ops.op_get_document_element()); },
  get head() { return new Element(Deno.core.ops.op_get_head()); },
  get body() { return new Element(Deno.core.ops.op_get_body()); },
  addEventListener(event, handler) {
    if (event === "DOMContentLoaded" || event === "load") { handler(); }
  },
};

// ---- Fetch polyfill ----
async function fetch(input, init) {
  init = init || {};
  const url = typeof input === "string" ? input : (input.url || String(input));
  const method = init.method || "GET";
  const headers = {};
  if (init.headers) {
    if (init.headers instanceof Map) { init.headers.forEach((v, k) => headers[k] = v); }
    else if (typeof init.headers === "object") { Object.assign(headers, init.headers); }
  }
  const resp = await Deno.core.ops.op_fetch({ url, method, headers, body: init.body || null });
  return {
    ok: resp.ok, status: resp.status, statusText: resp.status_text, url,
    headers: new Map(Object.entries(resp.headers || {})),
    text: async () => resp.body,
    json: async () => JSON.parse(resp.body),
  };
}

// ---- Window object ----
const window = {
  document, fetch,
  addEventListener: document.addEventListener.bind(document),
  location: { href: "", origin: "", protocol: "https:", host: "", hostname: "", pathname: "/", search: "", hash: "" },
  navigator: { userAgent: "PardusBrowser/0.1.0" },
  console: {
    log(...a) {}, warn(...a) {}, error(...a) {}, info(...a) {}, debug(...a) {},
  },
  setTimeout(fn, ms) { if (typeof fn === "function") fn(); return 0; },
  setInterval(fn, ms) { return 0; },
  clearTimeout() {}, clearInterval() {},
  getComputedStyle() { return new Proxy({}, { get: () => "" }); },
  matchMedia() { return { matches: false, addListener() {}, removeListener() {} }; },
  innerWidth: 1280, innerHeight: 720,
};

// ---- Globals ----
globalThis.window = window;
globalThis.document = document;
globalThis.fetch = fetch;
globalThis.Element = Element;
globalThis.TextNode = TextNode;
globalThis.DocumentFragment = DocumentFragment;
globalThis.Node = { ELEMENT_NODE: 1, TEXT_NODE: 3, DOCUMENT_FRAGMENT_NODE: 11 };
globalThis.setTimeout = window.setTimeout;
globalThis.setInterval = window.setInterval;
globalThis.clearTimeout = window.clearTimeout;
globalThis.clearInterval = window.clearInterval;
globalThis.console = window.console;
globalThis.navigator = window.navigator;
globalThis.performance = { now: () => Date.now() };
globalThis.self = globalThis;
globalThis.top = globalThis;
globalThis.parent = globalThis;
globalThis.frames = globalThis;
