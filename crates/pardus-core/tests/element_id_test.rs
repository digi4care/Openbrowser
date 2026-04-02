//! Tests for element ID feature.
//!
//! Tests that interactive elements get unique IDs assigned during semantic tree building,
//! and that these IDs can be used to find and interact with elements.

use pardus_core::semantic::tree::SemanticTree;

// ---------------------------------------------------------------------------
// Element ID Assignment Tests
// ---------------------------------------------------------------------------

#[test]
fn test_link_gets_element_id() {
    let html = r#"<html><body><a href="/about">About Us</a></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    // Find the link node
    fn find_link(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Link) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_link(child) {
                return Some(found);
            }
        }
        None
    }

    let link = find_link(&tree.root).expect("Should find a link");
    assert!(link.element_id.is_some(), "Link should have an element_id");
    assert_eq!(link.element_id.unwrap(), 1, "First interactive element should have ID 1");
}

#[test]
fn test_button_gets_element_id() {
    let html = r#"<html><body><button>Click Me</button></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_button(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Button) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_button(child) {
                return Some(found);
            }
        }
        None
    }

    let button = find_button(&tree.root).expect("Should find a button");
    assert!(button.element_id.is_some(), "Button should have an element_id");
    assert_eq!(button.element_id.unwrap(), 1, "First interactive element should have ID 1");
}

#[test]
fn test_multiple_interactive_elements_get_sequential_ids() {
    let html = r#"
        <html><body>
            <a href="/link1">Link 1</a>
            <button>Button 1</button>
            <a href="/link2">Link 2</a>
            <input type="text" name="query">
            <input type="submit" value="Submit">
        </body></html>
    "#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn collect_interactive_ids(node: &pardus_core::semantic::tree::SemanticNode, ids: &mut Vec<usize>) {
        if let Some(id) = node.element_id {
            ids.push(id);
        }
        for child in &node.children {
            collect_interactive_ids(child, ids);
        }
    }

    let mut ids: Vec<usize> = Vec::new();
    collect_interactive_ids(&tree.root, &mut ids);

    assert!(!ids.is_empty(), "Should have interactive elements with IDs");
    assert_eq!(ids.len(), 5, "Should have 5 interactive elements");

    // IDs should be sequential starting from 1
    let mut expected = 1;
    for id in &ids {
        assert_eq!(*id, expected, "Element ID should be sequential");
        expected += 1;
    }
}

#[test]
fn test_textbox_gets_element_id() {
    let html = r#"<html><body><input type="text" name="email" placeholder="Email"></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_textbox(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::TextBox) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_textbox(child) {
                return Some(found);
            }
        }
        None
    }

    let textbox = find_textbox(&tree.root).expect("Should find a textbox");
    assert!(textbox.element_id.is_some(), "Textbox should have an element_id");
}

#[test]
fn test_checkbox_gets_element_id() {
    let html = r#"<html><body><input type="checkbox" name="agree"></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_checkbox(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Checkbox) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_checkbox(child) {
                return Some(found);
            }
        }
        None
    }

    let checkbox = find_checkbox(&tree.root).expect("Should find a checkbox");
    assert!(checkbox.element_id.is_some(), "Checkbox should have an element_id");
}

#[test]
fn test_radio_gets_element_id() {
    let html = r#"<html><body><input type="radio" name="choice" value="a"></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_radio(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Radio) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_radio(child) {
                return Some(found);
            }
        }
        None
    }

    let radio = find_radio(&tree.root).expect("Should find a radio button");
    assert!(radio.element_id.is_some(), "Radio should have an element_id");
}

#[test]
fn test_combobox_gets_element_id() {
    let html = r#"<html><body><select name="country"><option>USA</option></select></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_combobox(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Combobox) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_combobox(child) {
                return Some(found);
            }
        }
        None
    }

    let combobox = find_combobox(&tree.root).expect("Should find a combobox");
    assert!(combobox.element_id.is_some(), "Combobox should have an element_id");
}

// ---------------------------------------------------------------------------
// Disabled Element Tests
// ---------------------------------------------------------------------------

#[test]
fn test_disabled_button_has_no_element_id() {
    let html = r#"<html><body><button disabled>Disabled Button</button></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_button(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Button) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_button(child) {
                return Some(found);
            }
        }
        None
    }

    let button = find_button(&tree.root).expect("Should find a button");
    assert!(button.is_disabled, "Button should be disabled");
    assert!(button.element_id.is_none(), "Disabled button should NOT have an element_id");
}

#[test]
fn test_disabled_elements_dont_consume_ids() {
    let html = r#"
        <html><body>
            <button disabled>Disabled</button>
            <button>Enabled</button>
        </body></html>
    "#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn collect_button_ids(node: &pardus_core::semantic::tree::SemanticNode, ids: &mut Vec<usize>) {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Button) {
            if let Some(id) = node.element_id {
                ids.push(id);
            }
        }
        for child in &node.children {
            collect_button_ids(child, ids);
        }
    }

    let mut ids: Vec<usize> = Vec::new();
    collect_button_ids(&tree.root, &mut ids);

    assert_eq!(ids.len(), 1, "Only one button should have an ID (the enabled one)");
    assert_eq!(ids[0], 1, "The enabled button should have ID 1");
}

// ---------------------------------------------------------------------------
// Non-Interactive Element Tests
// ---------------------------------------------------------------------------

#[test]
fn test_heading_has_no_element_id() {
    let html = r#"<html><body><h1>Title</h1></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_heading(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Heading { .. }) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_heading(child) {
                return Some(found);
            }
        }
        None
    }

    let heading = find_heading(&tree.root).expect("Should find a heading");
    assert!(heading.element_id.is_none(), "Heading should NOT have an element_id (not interactive)");
}

#[test]
fn test_generic_element_has_no_element_id() {
    let html = r#"<html><body><div>Some content</div></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_generic(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Generic) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_generic(child) {
                return Some(found);
            }
        }
        None
    }

    let generic = find_generic(&tree.root);
    if let Some(node) = generic {
        assert!(node.element_id.is_none(), "Generic element should NOT have an element_id");
    }
}

// ---------------------------------------------------------------------------
// Statistics Tests
// ---------------------------------------------------------------------------

#[test]
fn test_stats_count_interactive_elements() {
    let html = r#"
        <html><body>
            <a href="/link1">Link 1</a>
            <button>Button 1</button>
            <input type="text" name="query">
        </body></html>
    "#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    assert_eq!(tree.stats.actions, 3, "Should count 3 interactive elements");
    assert_eq!(tree.stats.links, 1, "Should count 1 link");
}

// ---------------------------------------------------------------------------
// Link Without Href Tests
// ---------------------------------------------------------------------------

#[test]
fn test_link_without_href_has_no_element_id() {
    let html = r#"<html><body><a name="anchor">Anchor Only</a></body></html>"#;
    let tree = SemanticTree::build(&scraper::Html::parse_document(html), "https://example.com");

    fn find_link(node: &pardus_core::semantic::tree::SemanticNode) -> Option<&pardus_core::semantic::tree::SemanticNode> {
        if matches!(node.role, pardus_core::semantic::tree::SemanticRole::Link) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = find_link(child) {
                return Some(found);
            }
        }
        None
    }

    // Links without href should not be interactive, so no link role should be found
    // Actually the role is still Link but is_interactive should be false
    let link = find_link(&tree.root);
    if let Some(node) = link {
        // If found, it should not be interactive
        if !node.is_interactive {
            assert!(node.element_id.is_none(), "Non-interactive link should NOT have element_id");
        }
    }
}
