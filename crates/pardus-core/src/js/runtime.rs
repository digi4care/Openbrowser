use std::cell::RefCell;
use std::rc::Rc;
use deno_core::*;
use super::dom::DomDocument;

/// Execute all scripts in the given HTML and return the modified HTML.
///
/// This uses deno_core (V8) to execute JavaScript. We provide a minimal
/// `document` and `window` shim via ops that interact with the DOM.
/// Note: Only inline scripts are executed - external scripts are skipped.
pub async fn execute_js(
    html: &str,
    _base_url: &str,
    _wait_ms: u32,
) -> anyhow::Result<String> {
    // For now, just return the HTML without any JS processing
    // to avoid hanging issues with complex sites
    Ok(html.to_string())
}

// ==================== DOM Ops ====================

// Document methods
#[op2(fast)]
fn op_create_element(state: &mut OpState, #[string] tag: &str) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().create_element(tag)
}

#[op2(fast)]
fn op_create_text_node(state: &mut OpState, #[string] text: &str) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().create_text_node(text)
}

#[op2(fast)]
fn op_create_document_fragment(state: &mut OpState) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().create_document_fragment()
}

#[op2(fast)]
fn op_get_element_by_id(state: &mut OpState, #[string] id: &str) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_element_by_id(id).unwrap_or(0)
}

#[op2(fast)]
fn op_query_selector(state: &mut OpState, node_id: u32, #[string] selector: &str) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().query_selector(node_id, selector).unwrap_or(0)
}

#[op2]
#[serde]
fn op_query_selector_all(state: &mut OpState, node_id: u32, #[string] selector: &str) -> Vec<u32> {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().query_selector_all(node_id, selector)
}

#[op2(fast)]
fn op_get_document_element(state: &mut OpState) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().document_element()
}

#[op2(fast)]
fn op_get_head(state: &mut OpState) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().head()
}

#[op2(fast)]
fn op_get_body(state: &mut OpState) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().body()
}

// Node/Element methods
#[op2(fast)]
fn op_append_child(state: &mut OpState, parent_id: u32, child_id: u32) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().append_child(parent_id, child_id);
}

#[op2(fast)]
fn op_remove_child(state: &mut OpState, parent_id: u32, child_id: u32) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().remove_child(parent_id, child_id);
}

#[op2(fast)]
fn op_insert_before(state: &mut OpState, parent_id: u32, new_node_id: u32, ref_node_id: u32) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    let ref_id = if ref_node_id == 0 { None } else { Some(ref_node_id) };
    dom.borrow_mut().insert_before(parent_id, new_node_id, ref_id);
}

#[op2(fast)]
fn op_replace_child(state: &mut OpState, parent_id: u32, new_child_id: u32, old_child_id: u32) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().replace_child(parent_id, new_child_id, old_child_id);
}

#[op2(fast)]
fn op_clone_node(state: &mut OpState, node_id: u32, deep: bool) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().clone_node(node_id, deep)
}

// Attribute methods
#[op2(fast)]
fn op_set_attribute(state: &mut OpState, node_id: u32, #[string] name: &str, #[string] value: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_attribute(node_id, name, value);
}

#[op2]
#[string]
fn op_get_attribute(state: &mut OpState, node_id: u32, #[string] name: &str) -> Option<String> {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_attribute(node_id, name)
}

#[op2(fast)]
fn op_remove_attribute(state: &mut OpState, node_id: u32, #[string] name: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().remove_attribute(node_id, name);
}

// Property getters
#[op2]
#[string]
fn op_get_tag_name(state: &mut OpState, node_id: u32) -> Option<String> {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_tag_name(node_id)
}

#[op2]
#[string]
fn op_get_node_id_attr(state: &mut OpState, node_id: u32) -> String {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_node_id_attr(node_id)
}

#[op2(fast)]
fn op_set_node_id_attr(state: &mut OpState, node_id: u32, #[string] id: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_node_id_attr(node_id, id);
}

#[op2]
#[string]
fn op_get_class_name(state: &mut OpState, node_id: u32) -> String {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_class_name(node_id)
}

#[op2(fast)]
fn op_set_class_name(state: &mut OpState, node_id: u32, #[string] class_name: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_class_name(node_id, class_name);
}

#[op2]
#[string]
fn op_get_inner_html(state: &mut OpState, node_id: u32) -> String {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_inner_html(node_id)
}

#[op2(fast)]
fn op_set_inner_html(state: &mut OpState, node_id: u32, #[string] html: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_inner_html(node_id, html);
}

#[op2]
#[string]
fn op_get_text_content(state: &mut OpState, node_id: u32) -> String {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_text_content(node_id)
}

#[op2(fast)]
fn op_set_text_content(state: &mut OpState, node_id: u32, #[string] text: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_text_content(node_id, text);
}

#[op2(fast)]
fn op_get_parent(state: &mut OpState, node_id: u32) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_parent(node_id).unwrap_or(0)
}

#[op2]
#[serde]
fn op_get_children(state: &mut OpState, node_id: u32) -> Vec<u32> {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_children(node_id)
}

#[op2(fast)]
fn op_get_previous_sibling(state: &mut OpState, node_id: u32) -> u32 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_previous_sibling(node_id).unwrap_or(0)
}

// Style
#[op2(fast)]
fn op_set_style(state: &mut OpState, node_id: u32, #[string] property: &str, #[string] value: &str) {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow_mut().set_style(node_id, property, value);
}

// Utility methods
#[op2(fast)]
fn op_contains(state: &mut OpState, container_id: u32, contained_id: u32) -> bool {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().contains(container_id, contained_id)
}

#[op2(fast)]
fn op_has_child_nodes(state: &mut OpState, node_id: u32) -> bool {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().has_child_nodes(node_id)
}

#[op2(fast)]
fn op_has_attributes(state: &mut OpState, node_id: u32) -> bool {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().has_attributes(node_id)
}

#[op2(fast)]
fn op_get_node_type(state: &mut OpState, node_id: u32) -> u16 {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_node_type(node_id)
}

#[op2]
#[string]
fn op_get_node_name(state: &mut OpState, node_id: u32) -> String {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_node_name(node_id)
}

#[op2]
#[serde]
fn op_get_attribute_names(state: &mut OpState, node_id: u32) -> Vec<String> {
    let dom = state.borrow::<Rc<RefCell<DomDocument>>>().clone();
    dom.borrow().get_attribute_names(node_id)
}

// Fetch op (existing)
#[op2]
#[serde]
async fn op_fetch(#[serde] args: FetchArgs) -> FetchResult {
    let client = reqwest::Client::new();
    let mut req = match args.method.as_str() {
        "POST" => client.post(&args.url),
        "PUT" => client.put(&args.url),
        "DELETE" => client.delete(&args.url),
        "PATCH" => client.patch(&args.url),
        _ => client.get(&args.url),
    };

    for (k, v) in &args.headers {
        req = req.header(k, v);
    }

    if let Some(body) = &args.body {
        req = req.body(body.clone());
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let status_text = resp.status().canonical_reason().unwrap_or("").to_string();
            let headers: std::collections::HashMap<String, String> = resp
                .headers()
                .iter()
                .filter_map(|(k, v)| Some((k.to_string(), v.to_str().ok()?.to_string())))
                .collect();
            let body = resp.text().await.unwrap_or_default();

            FetchResult {
                ok: status >= 200 && status < 300,
                status,
                status_text,
                headers,
                body,
            }
        }
        Err(_) => FetchResult {
            ok: false,
            status: 0,
            status_text: "Network Error".to_string(),
            headers: std::collections::HashMap::new(),
            body: String::new(),
        }
    }
}

#[derive(serde::Deserialize)]
struct FetchArgs {
    url: String,
    method: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
}

#[derive(serde::Serialize)]
struct FetchResult {
    ok: bool,
    status: u16,
    status_text: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

deno_core::extension!(
    pardus_dom,
    ops = [
        op_create_element,
        op_create_text_node,
        op_create_document_fragment,
        op_get_element_by_id,
        op_query_selector,
        op_query_selector_all,
        op_get_document_element,
        op_get_head,
        op_get_body,
        op_append_child,
        op_remove_child,
        op_insert_before,
        op_replace_child,
        op_clone_node,
        op_set_attribute,
        op_get_attribute,
        op_remove_attribute,
        op_get_tag_name,
        op_get_node_id_attr,
        op_set_node_id_attr,
        op_get_class_name,
        op_set_class_name,
        op_get_inner_html,
        op_set_inner_html,
        op_get_text_content,
        op_set_text_content,
        op_get_parent,
        op_get_children,
        op_get_previous_sibling,
        op_set_style,
        op_contains,
        op_has_child_nodes,
        op_has_attributes,
        op_get_node_type,
        op_get_node_name,
        op_get_attribute_names,
        op_fetch,
    ],
);
