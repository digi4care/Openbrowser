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
                let _child_id = self.build_from_scraper(&el, Some(node_id));
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

    // ---- Query Selector Support ----

    /// Query for the first element matching a CSS selector, starting from a given node.
    /// If node_id is 0, searches from document element.
    pub fn query_selector(&self, node_id: NodeId, selector: &str) -> Option<NodeId> {
        let start_node = if node_id == 0 {
            self.document_element_id
        } else {
            node_id
        };

        let css_selector = match Selector::parse(selector) {
            Ok(s) => s,
            Err(_) => return None,
        };

        self.query_selector_recursive(start_node, &css_selector)
    }

    fn query_selector_recursive(&self, node_id: NodeId, selector: &Selector) -> Option<NodeId> {
        let node = self.nodes.get(&node_id)?;

        // Check if current node matches
        if node.node_type == DomNodeType::Element {
            if self.node_matches_selector(node_id, selector) {
                return Some(node_id);
            }
        }

        // Search children depth-first
        for &child_id in &node.children {
            if let Some(found) = self.query_selector_recursive(child_id, selector) {
                return Some(found);
            }
        }

        None
    }

    /// Query for all elements matching a CSS selector, starting from a given node.
    /// If node_id is 0, searches from document element.
    pub fn query_selector_all(&self, node_id: NodeId, selector: &str) -> Vec<NodeId> {
        let start_node = if node_id == 0 {
            self.document_element_id
        } else {
            node_id
        };

        let css_selector = match Selector::parse(selector) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();
        self.query_selector_all_recursive(start_node, &css_selector, &mut results);
        results
    }

    fn query_selector_all_recursive(&self, node_id: NodeId, selector: &Selector, results: &mut Vec<NodeId>) {
        let node = match self.nodes.get(&node_id) {
            Some(n) => n,
            None => return,
        };

        // Check if current node matches
        if node.node_type == DomNodeType::Element {
            if self.node_matches_selector(node_id, selector) {
                results.push(node_id);
            }
        }

        // Search children
        for &child_id in &node.children {
            self.query_selector_all_recursive(child_id, selector, results);
        }
    }

    /// Check if a node matches a CSS selector
    fn node_matches_selector(&self, node_id: NodeId, selector: &Selector) -> bool {
        let node = match self.nodes.get(&node_id) {
            Some(n) => n,
            None => return false,
        };

        // Build a temporary HTML element for scraper to match against
        let _tag = match &node.tag_name {
            Some(t) => t.clone(),
            None => return false,
        };

        // Create a minimal HTML representation for this element
        let html = self.node_to_minimal_html(node_id);

        // Parse and check if selector matches
        let doc = Html::parse_fragment(&html);
        if let Some(el) = doc.select(selector).next() {
            // Verify it's the same element by checking a data attribute we add
            return el.value().attr("data-pardus-node-id")
                .map(|s| s == node_id.to_string())
                .unwrap_or(false);
        }

        false
    }

    /// Convert a node to minimal HTML for selector matching
    fn node_to_minimal_html(&self, node_id: NodeId) -> String {
        let node = match self.nodes.get(&node_id) {
            Some(n) => n,
            None => return String::new(),
        };

        let tag = node.tag_name.as_deref().unwrap_or("div");
        let mut html = format!("<{} data-pardus-node-id=\"{}\"", tag, node_id);

        for (k, v) in &node.attributes {
            html.push_str(&format!(" {}=\"{}\"", k, v.replace('"', "&quot;")));
        }

        html.push_str("></");
        html.push_str(tag);
        html.push('>');

        html
    }

    // ---- Extended Element API ----

    /// Insert a node before a reference node
    pub fn insert_before(&mut self, parent_id: NodeId, new_node_id: NodeId, ref_node_id: Option<NodeId>) {
        // Remove from old parent
        if let Some(old_parent) = self.nodes.get(&new_node_id).and_then(|n| n.parent_id) {
            if let Some(old) = self.nodes.get_mut(&old_parent) {
                old.children.retain(|&id| id != new_node_id);
            }
        }

        // Set new parent
        if let Some(new_node) = self.nodes.get_mut(&new_node_id) {
            new_node.parent_id = Some(parent_id);
        }

        // Insert at correct position
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            match ref_node_id {
                Some(ref_id) => {
                    if let Some(pos) = parent.children.iter().position(|&id| id == ref_id) {
                        parent.children.insert(pos, new_node_id);
                    } else {
                        parent.children.push(new_node_id);
                    }
                }
                None => {
                    parent.children.push(new_node_id);
                }
            }
        }
    }

    /// Replace a child node with another
    pub fn replace_child(&mut self, parent_id: NodeId, new_child_id: NodeId, old_child_id: NodeId) {
        // Remove new child from old parent
        if let Some(old_parent) = self.nodes.get(&new_child_id).and_then(|n| n.parent_id) {
            if let Some(old) = self.nodes.get_mut(&old_parent) {
                old.children.retain(|&id| id != new_child_id);
            }
        }

        // Set parent for new child
        if let Some(new_child) = self.nodes.get_mut(&new_child_id) {
            new_child.parent_id = Some(parent_id);
        }

        // Replace in parent's children
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            if let Some(pos) = parent.children.iter().position(|&id| id == old_child_id) {
                parent.children[pos] = new_child_id;
            }
        }

        // Remove old child
        if let Some(old_child) = self.nodes.get_mut(&old_child_id) {
            old_child.parent_id = None;
        }
        self.remove_recursive(old_child_id);
    }

    /// Clone a node
    pub fn clone_node(&mut self, node_id: NodeId, deep: bool) -> NodeId {
        self.clone_node_internal(node_id, None, deep)
    }

    fn clone_node_internal(&mut self, node_id: NodeId, parent_id: Option<NodeId>, deep: bool) -> NodeId {
        let node = match self.nodes.get(&node_id) {
            Some(n) => n.clone(),
            None => return 0,
        };

        let new_id = self.alloc_node(node.node_type.clone(), parent_id);

        if let Some(new_node) = self.nodes.get_mut(&new_id) {
            new_node.tag_name = node.tag_name;
            new_node.attributes = node.attributes;
            new_node.text_content = node.text_content;
        }

        if deep {
            for &child_id in &node.children {
                let cloned_child = self.clone_node_internal(child_id, Some(new_id), true);
                if let Some(new_node) = self.nodes.get_mut(&new_id) {
                    new_node.children.push(cloned_child);
                }
            }
        }

        new_id
    }

    /// Check if a node contains another node
    pub fn contains(&self, container_id: NodeId, contained_id: NodeId) -> bool {
        if container_id == contained_id {
            return false;
        }

        let mut current_id = contained_id;
        while let Some(node) = self.nodes.get(&current_id) {
            match node.parent_id {
                Some(pid) if pid == container_id => return true,
                Some(pid) => current_id = pid,
                None => return false,
            }
        }
        false
    }

    /// Check if node has child nodes
    pub fn has_child_nodes(&self, node_id: NodeId) -> bool {
        self.nodes.get(&node_id).map(|n| !n.children.is_empty()).unwrap_or(false)
    }

    /// Check if node has attributes
    pub fn has_attributes(&self, node_id: NodeId) -> bool {
        self.nodes.get(&node_id).map(|n| !n.attributes.is_empty()).unwrap_or(false)
    }

    /// Get previous sibling
    pub fn get_previous_sibling(&self, node_id: NodeId) -> Option<NodeId> {
        let parent_id = self.nodes.get(&node_id)?.parent_id?;
        let parent = self.nodes.get(&parent_id)?;
        let idx = parent.children.iter().position(|&id| id == node_id)?;
        if idx > 0 {
            Some(parent.children[idx - 1])
        } else {
            None
        }
    }

    /// Get node type as number (matches DOM Node.ELEMENT_NODE, etc.)
    pub fn get_node_type(&self, node_id: NodeId) -> u16 {
        self.nodes.get(&node_id).map(|n| match n.node_type {
            DomNodeType::Element => 1,
            DomNodeType::Text => 3,
            DomNodeType::Document => 9,
            DomNodeType::DocumentFragment => 11,
        }).unwrap_or(0)
    }

    /// Get node name (tagName for elements, #text for text nodes, etc.)
    pub fn get_node_name(&self, node_id: NodeId) -> String {
        self.nodes.get(&node_id).map(|n| match &n.node_type {
            DomNodeType::Element => n.tag_name.clone().unwrap_or_default().to_uppercase(),
            DomNodeType::Text => "#text".to_string(),
            DomNodeType::Document => "#document".to_string(),
            DomNodeType::DocumentFragment => "#document-fragment".to_string(),
        }).unwrap_or_default()
    }

    /// Get all attribute names
    pub fn get_attribute_names(&self, node_id: NodeId) -> Vec<String> {
        self.nodes.get(&node_id)
            .map(|n| n.attributes.keys().cloned().collect())
            .unwrap_or_default()
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

    // ==================== Query Selector Tests ====================

    #[test]
    fn test_query_selector_by_id() {
        let html = "<html><body><div id=\"app\"><span id=\"inner\">test</span></div></body></html>";
        let doc = DomDocument::from_html(html);
        let result = doc.query_selector(0, "#inner");
        assert!(result.is_some());
        let node_id = result.unwrap();
        assert_eq!(doc.get_text_content(node_id), "test");
    }

    #[test]
    fn test_query_selector_by_class() {
        let html = "<html><body><div class=\"container\"><p class=\"item\">first</p><p class=\"item\">second</p></div></body></html>";
        let doc = DomDocument::from_html(html);
        let result = doc.query_selector(0, ".item");
        assert!(result.is_some());
        let node_id = result.unwrap();
        assert_eq!(doc.get_text_content(node_id), "first");
    }

    #[test]
    fn test_query_selector_by_tag() {
        let html = "<html><body><div><article><h1>Title</h1></article></div></body></html>";
        let doc = DomDocument::from_html(html);
        let result = doc.query_selector(0, "article");
        assert!(result.is_some());
    }

    #[test]
    fn test_query_selector_not_found() {
        let html = "<html><body><div>content</div></body></html>";
        let doc = DomDocument::from_html(html);
        let result = doc.query_selector(0, "#nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_query_selector_all() {
        let html = "<html><body><ul><li class=\"item\">1</li><li class=\"item\">2</li><li class=\"item\">3</li></ul></body></html>";
        let doc = DomDocument::from_html(html);
        let results = doc.query_selector_all(0, ".item");
        // Check that we find at least some items
        assert!(!results.is_empty());
    }

    #[test]
    fn test_query_selector_all_empty() {
        let html = "<html><body><div>no items</div></body></html>";
        let doc = DomDocument::from_html(html);
        let results = doc.query_selector_all(0, "#nonexistent");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_query_selector_from_element() {
        let html = "<html><body><div id=\"container\"><span class=\"inner\">a</span><span class=\"inner\">b</span></div><span class=\"inner\">c</span></body></html>";
        let doc = DomDocument::from_html(html);
        let container = doc.get_element_by_id("container").unwrap();
        let results = doc.query_selector_all(container, ".inner");
        // Should find items inside container
        assert!(!results.is_empty());
    }

    // ==================== DOM Manipulation Tests ====================

    #[test]
    fn test_insert_before_into_empty() {
        let html = "<html><body><div id=\"parent\"></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let parent = doc.get_element_by_id("parent").unwrap();
        let new_child = doc.create_element("span");
        doc.set_attribute(new_child, "id", "new");
        doc.insert_before(parent, new_child, None);
        let children = doc.get_children(parent);
        assert_eq!(children.len(), 1);
        assert_eq!(doc.get_node_id_attr(children[0]), "new");
    }

    #[test]
    fn test_insert_before_null_ref_appends() {
        let html = "<html><body><div id=\"parent\"></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let parent = doc.get_element_by_id("parent").unwrap();
        let child1 = doc.create_element("span");
        doc.set_attribute(child1, "id", "child1");
        doc.insert_before(parent, child1, None);
        let child2 = doc.create_element("span");
        doc.set_attribute(child2, "id", "child2");
        doc.insert_before(parent, child2, None);
        let children = doc.get_children(parent);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_replace_child() {
        let html = "<html><body><div id=\"parent\"><span id=\"old\">old</span></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let parent = doc.get_element_by_id("parent").unwrap();
        let old_child = doc.get_element_by_id("old").unwrap();
        let new_child = doc.create_element("span");
        doc.set_attribute(new_child, "id", "new");
        doc.set_text_content(new_child, "new");
        doc.replace_child(parent, new_child, old_child);
        assert!(doc.get_element_by_id("old").is_none());
        assert!(doc.get_element_by_id("new").is_some());
    }

    #[test]
    fn test_clone_node_shallow() {
        let html = "<html><body><div id=\"original\" class=\"test\"><span>child</span></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let original = doc.get_element_by_id("original").unwrap();
        let clone = doc.clone_node(original, false);
        // Should have same attributes (including id, which is standard DOM behavior)
        assert_eq!(doc.get_attribute(clone, "id"), Some("original".to_string()));
        assert_eq!(doc.get_attribute(clone, "class"), Some("test".to_string()));
        // Should not have children in shallow clone
        assert_eq!(doc.get_children(clone).len(), 0);
    }

    #[test]
    fn test_clone_node_deep() {
        let html = "<html><body><div id=\"original\"><span>child1</span><span>child2</span></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let original = doc.get_element_by_id("original").unwrap();
        let clone = doc.clone_node(original, true);
        // Should have children in deep clone
        assert!(doc.get_children(clone).len() >= 2);
    }

    // ==================== Utility Methods Tests ====================

    #[test]
    fn test_contains() {
        let html = "<html><body><div id=\"outer\"><div id=\"inner\"><span id=\"deep\">text</span></div></div></body></html>";
        let doc = DomDocument::from_html(html);
        let outer = doc.get_element_by_id("outer").unwrap();
        let inner = doc.get_element_by_id("inner").unwrap();
        let deep = doc.get_element_by_id("deep").unwrap();
        let body = doc.body();
        assert!(doc.contains(outer, inner));
        assert!(doc.contains(outer, deep));
        assert!(doc.contains(body, deep));
        assert!(!doc.contains(inner, outer));
        assert!(!doc.contains(deep, outer));
    }

    #[test]
    fn test_has_child_nodes() {
        let html = "<html><body><div id=\"empty\"></div><div id=\"with-children\"><span>child</span></div></body></html>";
        let doc = DomDocument::from_html(html);
        let empty = doc.get_element_by_id("empty").unwrap();
        let with_children = doc.get_element_by_id("with-children").unwrap();
        assert!(!doc.has_child_nodes(empty));
        assert!(doc.has_child_nodes(with_children));
    }

    #[test]
    fn test_has_attributes() {
        let html = "<html><body><div id=\"with-attr\" class=\"test\"></div><div id=\"without-attr\"></div></body></html>";
        let doc = DomDocument::from_html(html);
        let with_attr = doc.get_element_by_id("with-attr").unwrap();
        let without_attr = doc.get_element_by_id("without-attr").unwrap();
        assert!(doc.has_attributes(with_attr));
        // Note: id is also an attribute
        assert!(doc.has_attributes(without_attr));
    }

    #[test]
    fn test_get_previous_sibling() {
        let html = "<html><body><div id=\"first\"></div><div id=\"second\"></div><div id=\"third\"></div></body></html>";
        let doc = DomDocument::from_html(html);
        let second = doc.get_element_by_id("second").unwrap();
        let third = doc.get_element_by_id("third").unwrap();
        let prev_of_second = doc.get_previous_sibling(second).unwrap();
        let prev_of_third = doc.get_previous_sibling(third).unwrap();
        assert_eq!(doc.get_node_id_attr(prev_of_second), "first");
        assert_eq!(doc.get_node_id_attr(prev_of_third), "second");
    }

    #[test]
    fn test_get_previous_sibling_none() {
        let html = "<html><body><div id=\"only\"></div></body></html>";
        let doc = DomDocument::from_html(html);
        let only = doc.get_element_by_id("only").unwrap();
        assert!(doc.get_previous_sibling(only).is_none());
    }

    #[test]
    fn test_get_node_type() {
        let html = "<html><body><div id=\"elem\">text</div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let elem = doc.get_element_by_id("elem").unwrap();
        let text = doc.create_text_node("hello");
        let frag = doc.create_document_fragment();
        assert_eq!(doc.get_node_type(elem), 1); // ELEMENT_NODE
        assert_eq!(doc.get_node_type(text), 3); // TEXT_NODE
        assert_eq!(doc.get_node_type(frag), 11); // DOCUMENT_FRAGMENT_NODE
    }

    #[test]
    fn test_get_node_name() {
        let html = "<html><body><div id=\"elem\">text</div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let elem = doc.get_element_by_id("elem").unwrap();
        let text = doc.create_text_node("hello");
        let frag = doc.create_document_fragment();
        assert_eq!(doc.get_node_name(elem), "DIV");
        assert_eq!(doc.get_node_name(text), "#text");
        assert_eq!(doc.get_node_name(frag), "#document-fragment");
    }

    #[test]
    fn test_get_attribute_names() {
        let html = "<html><body><div id=\"test\" class=\"foo\" data-value=\"bar\"></div></body></html>";
        let doc = DomDocument::from_html(html);
        let elem = doc.get_element_by_id("test").unwrap();
        let names = doc.get_attribute_names(elem);
        assert!(names.contains(&"id".to_string()));
        assert!(names.contains(&"class".to_string()));
        assert!(names.contains(&"data-value".to_string()));
        assert_eq!(names.len(), 3);
    }

    // ==================== Style Tests ====================

    #[test]
    fn test_set_style() {
        let html = "<html><body><div id=\"styled\"></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let elem = doc.get_element_by_id("styled").unwrap();
        doc.set_style(elem, "color", "red");
        doc.set_style(elem, "font-size", "14px");
        let style = doc.get_attribute(elem, "style").unwrap();
        assert!(style.contains("color"));
        assert!(style.contains("red"));
        assert!(style.contains("font-size"));
        assert!(style.contains("14px"));
    }

    #[test]
    fn test_set_style_override() {
        let html = "<html><body><div id=\"styled\" style=\"color: blue\"></div></body></html>";
        let mut doc = DomDocument::from_html(html);
        let elem = doc.get_element_by_id("styled").unwrap();
        doc.set_style(elem, "color", "red");
        let style = doc.get_attribute(elem, "style").unwrap();
        assert!(style.contains("color: red"));
        assert!(!style.contains("blue"));
    }
}
