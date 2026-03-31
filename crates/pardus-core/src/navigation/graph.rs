use scraper::{Html, Selector, ElementRef};
use url::Url;
use serde::Serialize;

/// Navigation graph extracted from a page — all reachable routes and forms.
#[derive(Debug, Serialize)]
pub struct NavigationGraph {
    pub current_url: String,
    pub internal_links: Vec<Route>,
    pub external_links: Vec<String>,
    pub forms: Vec<FormDescriptor>,
}

#[derive(Debug, Serialize)]
pub struct Route {
    pub url: String,
    pub label: Option<String>,
    pub rel: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FormDescriptor {
    pub id: Option<String>,
    pub action: Option<String>,
    pub method: String,
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Debug, Serialize)]
pub struct FieldDescriptor {
    pub name: Option<String>,
    pub field_type: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
}

impl NavigationGraph {
    pub fn build(html: &Html, current_url: &str) -> Self {
        let current_origin = Url::parse(current_url)
            .ok()
            .map(|u| u.origin().ascii_serialization());

        let mut internal_links = Vec::new();
        let mut external_links = Vec::new();
        let mut forms = Vec::new();

        // Collect links
        if let Ok(selector) = Selector::parse("a[href]") {
            for el in html.select(&selector) {
                let href = el.value().attr("href").unwrap_or("");
                let label: String = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                let rel = el.value().attr("rel").map(|s| s.to_string());

                let resolved = Url::parse(current_url)
                    .and_then(|base| base.join(href))
                    .ok();

                if let Some(resolved) = resolved {
                    let is_same_origin = current_origin.as_ref()
                        .map(|o| resolved.origin().ascii_serialization() == *o)
                        .unwrap_or(false);

                    if is_same_origin {
                        let route = Route {
                            url: resolved.to_string(),
                            label: if label.is_empty() { None } else { Some(label) },
                            rel,
                        };
                        // Deduplicate
                        if !internal_links.iter().any(|r: &Route| r.url == route.url) {
                            internal_links.push(route);
                        }
                    } else {
                        if !external_links.contains(&resolved.to_string()) {
                            external_links.push(resolved.to_string());
                        }
                    }
                }
            }
        }

        // Collect forms
        if let Ok(selector) = Selector::parse("form") {
            for form_el in html.select(&selector) {
                let action = form_el.value().attr("action")
                    .and_then(|a| Url::parse(current_url).ok()?.join(a).ok())
                    .map(|u| u.to_string());

                let method = form_el.value()
                    .attr("method")
                    .unwrap_or("GET")
                    .to_uppercase();

                let id = form_el.value().attr("id").map(|s| s.to_string());

                let mut fields = Vec::new();
                if let Ok(input_sel) = Selector::parse("input, select, textarea") {
                    for field_el in form_el.select(&input_sel) {
                        let field_name = field_el.value().attr("name").map(|s| s.to_string());
                        let field_type = field_el.value()
                            .attr("type")
                            .unwrap_or(field_el.value().name())
                            .to_string();

                        // Find associated label
                        let label = find_label_for(&form_el, &field_el, field_name.as_deref());

                        fields.push(FieldDescriptor {
                            name: field_name,
                            field_type,
                            label,
                            placeholder: field_el.value().attr("placeholder").map(|s| s.to_string()),
                            required: field_el.value().attr("required").is_some(),
                        });
                    }
                }

                forms.push(FormDescriptor {
                    id,
                    action,
                    method,
                    fields,
                });
            }
        }

        NavigationGraph {
            current_url: current_url.to_string(),
            internal_links,
            external_links,
            forms,
        }
    }
}

/// Try to find a <label> associated with a form field.
fn find_label_for(form: &ElementRef, _field: &ElementRef, field_name: Option<&str>) -> Option<String> {
    if let Ok(label_sel) = Selector::parse("label") {
        for label_el in form.select(&label_sel) {
            // Check for attribute: for="field_name"
            if let Some(for_attr) = label_el.value().attr("for") {
                if let Some(name) = field_name {
                    if for_attr == name {
                        let text: String = label_el.text().collect();
                        let trimmed = text.trim().to_string();
                        if !trimmed.is_empty() {
                            return Some(trimmed);
                        }
                    }
                }
            }
        }
    }
    None
}
