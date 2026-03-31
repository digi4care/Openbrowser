use std::collections::HashMap;
use scraper::{Html, Selector, ElementRef};

/// Unique ID for a DOM node.
pub type NodeId = u32;

/// A minimal DOM document backed by a flat HashMap.
#[derive(Debug)]
pub struct DomDocument {
    nodes: HashMap<NodeId, DomNode>,
    next_id: NodeId,
    document_element_id: NodeId,
    head_id: NodeId,
    body_id: NodeId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DomNodeType {
    Element,
    Text,
    Document,
    DocumentFragment,
}

#[derive(Debug, Clone)]
pub struct DomNode {
    pub id: NodeId,
    pub node_type: DomNodeType,
    pub tag_name: Option<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<NodeId>,
    pub parent_id: Option<NodeId>,
    pub text_content: Option<String>,
}

impl DomDocument {
    /// Build a DomDocument from an HTML string.
    pub fn from_html(html: &str) -> Self {
        let parsed = Html::parse_document(html);
        let mut doc = Self {
            nodes: HashMap::new(),
            next_id: 1,
            document_element_id: 0,
            head_id: 0,
            body_id: 0,
        };

        // Create document root
        let _doc_id = doc.alloc_node(DomNodeType::Document, None);

        // Walk the parsed tree
        if let Some(html_el) = parsed.select(&Selector::parse("html").unwrap()).next() {
            let html_id = doc.build_from_scraper(&html_el, None);
            doc.document_element_id = html_id;

            // Find head and body among html's direct children
            for &child_id in &doc.nodes.get(&html_id).unwrap().children.clone() {
                if let Some(node) = doc.nodes.get(&child_id) {
                    match node.tag_name.as_deref() {
                        Some("head") => doc.head_id = child_id,
                        Some("body") => doc.body_id = child_id,
                        _ => {}
                    }
                }
            }
        } else if let Some(body_el) = parsed.select(&Selector::parse("body").unwrap()).next() {
            let body_id = doc.build_from_scraper(&body_el, None);
            doc.body_id = body_id;
            doc.document_element_id = body_id;
        }

        doc
    }

    fn alloc_node(&mut self, node_type: DomNodeType, parent_id: Option<NodeId>) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, DomNode {
            id,
            node_type,
            tag_name: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            parent_id,
            text_content: None,
        });
        id
    }

    fn alloc_element(&mut self, tag: &str, parent_id: Option<NodeId>) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, DomNode {
            id,
            node_type: DomNodeType::Element,
            tag_name: Some(tag.to_lowercase()),
            attributes: HashMap::new(),
            children: Vec::new(),
            parent_id,
            text_content: None,
        });
        id
    }

    fn alloc_text(&mut self, text: &str, parent_id: Option<NodeId>) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, DomNode {
            id,
            node_type: DomNodeType::Text,
            tag_name: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            parent_id,
            text_content: Some(text.to_string()),
        });
        id
    }

    /// Recursively build DOM from a scraper element.
    fn build_from_scraper(&mut self, el: &ElementRef, parent_id: Option<NodeId>) -> NodeId {
        let tag = el.value().name().to_lowercase();
        let id = self.alloc_element(&tag, parent_id);

        // Copy attributes
        for (k, v) in el.value().attrs() {
            self.nodes.get_mut(&id).unwrap().attributes.insert(k.to_string(), v.to_string());
        }

        // Walk children — skip script/style subtrees
        for child_node in el.children() {
            if let Some(child_el) = ElementRef::wrap(child_node) {
                let child_tag = child_el.value().name().to_lowercase();
                if matches!(child_tag.as_str(), "script" | "style") {
                    continue;
                }
                let child_id = self.build_from_scraper(&child_el, Some(id));
                self.nodes.get_mut(&id).unwrap().children.push(child_id);
            } else if let Some(text) = child_node.value().as_text() {
                if !text.text.trim().is_empty() {
                    let text_id = self.alloc_text(&text.text, Some(id));
                    self.nodes.get_mut(&id).unwrap().children.push(text_id);
                }
            }
        }

        if let Some(pid) = parent_id {
            self.nodes.get_mut(&pid).unwrap().children.push(id);
        }

        id
    }

    // ---- Serialization ----

    pub fn to_html(&self) -> String {
        let mut output = String::new();
        if self.document_element_id != 0 {
            self.serialize_node(self.document_element_id, &mut output);
        }
        output
    }

    fn serialize_node(&self, id: NodeId, output: &mut String) {
        let node = match self.nodes.get(&id) {
            Some(n) => n,
            None => return,
        };

        match node.node_type {
            DomNodeType::Text => {
                if let Some(text) = &node.text_content {
                    output.push_str(text);
                }
            }
            DomNodeType::Element => {
                let tag = node.tag_name.as_deref().unwrap_or("div");
                let void = matches!(tag, "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "param" | "source" | "track" | "wbr");

                output.push('<');
                output.push_str(tag);
                for (k, v) in &node.attributes {
                    output.push(' ');
                    output.push_str(k);
                    output.push_str("=\"");
                    output.push_str(&v.replace('&', "&amp;").replace('"', "&quot;"));
                    output.push('"');
                }
                output.push('>');

                if !void {
                    for &child_id in &node.children {
                        self.serialize_node(child_id, output);
                    }
                    output.push_str("</");
                    output.push_str(tag);
                    output.push('>');
                }
            }
            DomNodeType::Document | DomNodeType::DocumentFragment => {
                for &child_id in &node.children {
                    self.serialize_node(child_id, output);
                }
            }
        }
    }

    // ---- DOM manipulation ----

    pub fn create_element(&mut self, tag: &str) -> NodeId {
        self.alloc_element(tag, None)
    }

    pub fn create_text_node(&mut self, text: &str) -> NodeId {
        self.alloc_text(text, None)
    }

    pub fn create_document_fragment(&mut self) -> NodeId {
        self.alloc_node(DomNodeType::DocumentFragment, None)
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        // Remove from old parent
        if let Some(old_parent) = self.nodes.get(&child_id).and_then(|n| n.parent_id) {
            if let Some(old) = self.nodes.get_mut(&old_parent) {
                old.children.retain(|&id| id != child_id);
            }
        }
        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent_id = Some(parent_id);
        }
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
        }
    }

    pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.retain(|&id| id != child_id);
        }
        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent_id = None;
        }
        self.remove_recursive(child_id);
    }

    fn remove_recursive(&mut self, node_id: NodeId) {
        if let Some(node) = self.nodes.get(&node_id) {
            let children: Vec<NodeId> = node.children.clone();
            for cid in children {
                self.remove_recursive(cid);
            }
        }
        self.nodes.remove(&node_id);
    }

    pub fn set_attribute(&mut self, node_id: NodeId, name: &str, value: &str) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.attributes.insert(name.to_string(), value.to_string());
        }
    }

    pub fn get_attribute(&self, node_id: NodeId, name: &str) -> Option<String> {
        self.nodes.get(&node_id).and_then(|n| n.attributes.get(name).cloned())
    }

    pub fn remove_attribute(&mut self, node_id: NodeId, name: &str) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.attributes.remove(name);
        }
    }

    pub fn set_inner_html(&mut self, node_id: NodeId, html: &str) {
        // Remove existing children
        let old_children: Vec<NodeId> = self.nodes.get(&node_id)
            .map(|n| n.children.clone()).unwrap_or_default();
        for old_id in old_children {
            self.remove_recursive(old_id);
        }
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.children.clear();
        }
        // Parse and add new children
        let fragment = Html::parse_fragment(html);
        for node_ref in fragment.tree.nodes() {
            if let Some(el) = ElementRef::wrap(node_ref) {
                let child_id = self.build_from_scraper(&el, Some(node_id));
                // build_from_scraper already adds to parent
            } else if let Some(text) = node_ref.value().as_text() {
                if !text.text.trim().is_empty() {
                    let text_id = self.alloc_text(&text.text, Some(node_id));
                    if let Some(parent) = self.nodes.get_mut(&node_id) {
                        parent.children.push(text_id);
                    }
                }
            }
        }
    }

    pub fn get_inner_html(&self, node_id: NodeId) -> String {
        let mut output = String::new();
        if let Some(node) = self.nodes.get(&node_id) {
            for &child_id in &node.children {
                self.serialize_node(child_id, &mut output);
            }
        }
        output
    }

    pub fn get_text_content(&self, node_id: NodeId) -> String {
        let node = match self.nodes.get(&node_id) {
            Some(n) => n,
            None => return String::new(),
        };
        match node.node_type {
            DomNodeType::Text => node.text_content.clone().unwrap_or_default(),
            _ => {
                let mut text = String::new();
                for &child_id in &node.children {
                    text.push_str(&self.get_text_content(child_id));
                }
                text
            }
        }
    }

    pub fn set_text_content(&mut self, node_id: NodeId, text: &str) {
        let old_children: Vec<NodeId> = self.nodes.get(&node_id)
            .map(|n| n.children.clone()).unwrap_or_default();
        for old_id in old_children {
            self.remove_recursive(old_id);
        }
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.children.clear();
        }
        let text_id = self.alloc_text(text, Some(node_id));
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.children.push(text_id);
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<NodeId> {
        for (&node_id, node) in &self.nodes {
            if let Some(attr_id) = node.attributes.get("id") {
                if attr_id == id {
                    return Some(node_id);
                }
            }
        }
        None
    }

    pub fn get_parent(&self, node_id: NodeId) -> Option<NodeId> {
        self.nodes.get(&node_id).and_then(|n| n.parent_id)
    }

    // ---- Accessors ----

    pub fn document_element(&self) -> NodeId { self.document_element_id }
    pub fn head(&self) -> NodeId { self.head_id }
    pub fn body(&self) -> NodeId { self.body_id }

    pub fn get_tag_name(&self, node_id: NodeId) -> Option<String> {
        self.nodes.get(&node_id).and_then(|n| n.tag_name.clone())
    }

    pub fn get_children(&self, node_id: NodeId) -> Vec<NodeId> {
        self.nodes.get(&node_id).map(|n| n.children.clone()).unwrap_or_default()
    }

    pub fn get_class_name(&self, node_id: NodeId) -> String {
        self.get_attribute(node_id, "class").unwrap_or_default()
    }

    pub fn set_class_name(&mut self, node_id: NodeId, class_name: &str) {
        self.set_attribute(node_id, "class", class_name);
    }

    pub fn get_node_id_attr(&self, node_id: NodeId) -> String {
        self.get_attribute(node_id, "id").unwrap_or_default()
    }

    pub fn set_node_id_attr(&mut self, node_id: NodeId, id: &str) {
        self.set_attribute(node_id, "id", id);
    }

    pub fn set_style(&mut self, node_id: NodeId, property: &str, value: &str) {
        let existing = self.get_attribute(node_id, "style").unwrap_or_default();
        let style = format_style_property(&existing, property, value);
        self.set_attribute(node_id, "style", &style);
    }
}

fn format_style_property(existing: &str, property: &str, value: &str) -> String {
    let target = format!("{}:", property);
    let mut found = false;
    let parts: Vec<String> = existing.split(';')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() { return None; }
            if p.starts_with(&target) {
                found = true;
                Some(format!("{}: {}", property, value))
            } else {
                Some(p.to_string())
            }
        })
        .collect();
    let mut result = parts.join("; ");
    if !found {
        if !result.is_empty() { result.push_str("; "); }
        result.push_str(&format!("{}: {}", property, value));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let html = "<html><head></head><body><h1>Hello</h1><p class=\"intro\">World</p></body></html>";
        let doc = DomDocument::from_html(html);
        let output = doc.to_html();
        assert!(output.contains("<h1>"));
        assert!(output.contains("Hello"));
        assert!(output.contains("<p"));
        assert!(output.contains("class=\"intro\""));
    }

    #[test]
    fn test_create_and_append() {
        let html = "<html><head></head><body></body></html>";
        let mut doc = DomDocument::from_html(html);
        let body = doc.body();
        let div = doc.create_element("div");
        doc.set_attribute(div, "id", "app");
        doc.append_child(body, div);
        let output = doc.to_html();
        assert!(output.contains("<div id=\"app\">"));
    }

    #[test]
    fn test_set_inner_html() {
        let html = "<html><head></head><body><div id=\"app\"></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let app = doc.get_element_by_id("app").unwrap();
        doc.set_inner_html(app, "<p>Rendered!</p>");
        let output = doc.to_html();
        assert!(output.contains("Rendered!"));
    }

    #[test]
    fn test_get_element_by_id() {
        let html = "<html><body><div id=\"foo\">bar</div></body></html>";
        let doc = DomDocument::from_html(html);
        let id = doc.get_element_by_id("foo").unwrap();
        assert_eq!(doc.get_text_content(id), "bar");
    }

    #[test]
    fn test_get_parent() {
        let html = "<html><body><div id=\"child\">x</div></body></html>";
        let doc = DomDocument::from_html(html);
        let child = doc.get_element_by_id("child").unwrap();
        let parent = doc.get_parent(child);
        assert!(parent.is_some());
    }
}
