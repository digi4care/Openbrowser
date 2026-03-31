use std::cell::RefCell;
use std::rc::Rc;
use deno_core::*;

/// Execute all scripts in the given HTML and return the modified HTML.
///
/// This uses deno_core (V8) to execute JavaScript. We provide a minimal
/// `document` and `window` shim via a single op that stores the final HTML.
pub async fn execute_js(
    html: &str,
    base_url: &str,
    wait_ms: u32,
) -> anyhow::Result<String> {
    let final_html: Rc<RefCell<String>> = Rc::new(RefCell::new(html.to_string()));
    let scripts = extract_scripts(html, base_url).await?;

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![pardus_dom::init()],
        ..Default::default()
    });

    // Store shared state
    {
        let op_state = runtime.op_state();
        let mut state = op_state.borrow_mut();
        state.put(final_html.clone());
    }

    // Bootstrap JS globals (document, window, fetch shim)
    let bootstrap = include_str!("bootstrap.js");
    runtime.execute_script("<bootstrap>", ModuleCodeString::from(bootstrap.to_string()))?;

    // Execute each script
    for script in scripts {
        let result = match script {
            ScriptSource::Inline(code) => runtime.execute_script("<inline>", ModuleCodeString::from(code)),
            ScriptSource::External(url, code) => runtime.execute_script(url.clone(), ModuleCodeString::from(code)),
        };
        if let Err(e) = result {
            tracing::debug!("JS execution error: {e}");
        }
    }

    // Run event loop with timeout to settle async operations
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(wait_ms as u64),
        runtime.run_event_loop(PollEventLoopOptions::default()),
    ).await;

    Ok(final_html.borrow().clone())
}

enum ScriptSource {
    Inline(String),
    External(String, String),
}

async fn extract_scripts(html: &str, base_url: &str) -> anyhow::Result<Vec<ScriptSource>> {
    use scraper::{Html, Selector};
    let parsed = Html::parse_document(html);
    let mut scripts = Vec::new();
    let client = reqwest::Client::new();

    if let Ok(sel) = Selector::parse("script") {
        for el in parsed.select(&sel) {
            if let Some(typ) = el.value().attr("type") {
                let t = typ.to_lowercase();
                if !t.is_empty()
                    && t != "text/javascript"
                    && t != "module"
                    && t != "application/javascript"
                {
                    continue;
                }
            }
            if let Some(src) = el.value().attr("src") {
                let resolved = url::Url::parse(base_url)
                    .and_then(|b| b.join(src))
                    .map(|u| u.to_string())
                    .unwrap_or_else(|_| src.to_string());
                match client.get(&resolved).send().await {
                    Ok(resp) => {
                        if let Ok(code) = resp.text().await {
                            scripts.push(ScriptSource::External(resolved, code));
                        }
                    }
                    Err(e) => tracing::debug!("Failed to fetch script {resolved}: {e}"),
                }
            } else {
                let code: String = el.text().collect();
                if !code.trim().is_empty() {
                    scripts.push(ScriptSource::Inline(code));
                }
            }
        }
    }
    Ok(scripts)
}

// Minimal op: allows JS to set the final HTML result
#[op2(fast)]
fn op_set_result_html(state: &mut OpState, #[string] html: &str) {
    let buf = state.borrow::<Rc<RefCell<String>>>().clone();
    *buf.borrow_mut() = html.to_string();
}

deno_core::extension!(
    pardus_dom,
    ops = [op_set_result_html],
);
