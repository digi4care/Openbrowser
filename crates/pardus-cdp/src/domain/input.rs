use async_trait::async_trait;
use serde_json::Value;

use crate::domain::{method_not_found, CdpDomainHandler, DomainContext, HandleResult};
use crate::protocol::message::CdpErrorResponse;
use crate::protocol::target::CdpSession;

pub struct InputDomain;

fn resolve_target_id(session: &CdpSession) -> &str {
    session.target_id.as_deref().unwrap_or("default")
}

#[async_trait]
impl CdpDomainHandler for InputDomain {
    fn domain_name(&self) -> &'static str {
        "Input"
    }

    async fn handle(
        &self,
        method: &str,
        params: Value,
        session: &mut CdpSession,
        ctx: &DomainContext,
    ) -> HandleResult {
        let target_id = resolve_target_id(session);

        match method {
            "dispatchMouseEvent" => {
                let _mouse_type = params["type"].as_str().unwrap_or("");
                let _x = params["x"].as_f64();
                let _y = params["y"].as_f64();
                HandleResult::Ack
            }
            "dispatchKeyEvent" => {
                let _key = params["key"].as_str().unwrap_or("");
                HandleResult::Ack
            }
            "insertText" => {
                let text = params["text"].as_str().unwrap_or("");
                if !text.is_empty() {
                    if let (Some(html_str), Some(url)) = (ctx.get_html(target_id).await, ctx.get_url(target_id).await) {
                        let page = pardus_core::Page::from_html(&html_str, &url);
                        if let Some(el) = page.query("input[type='text'], input:not([type]), textarea") {
                            if pardus_core::interact::actions::type_text(&page, &el, text).is_err() {
                                return HandleResult::Error(CdpErrorResponse {
                                    id: 0,
                                    error: crate::error::CdpErrorBody {
                                        code: crate::error::SERVER_ERROR,
                                        message: "Input.insertText failed: no matching element".to_string(),
                                    },
                                    session_id: None,
                                });
                            }
                        }
                    }
                }
                HandleResult::Ack
            }
            _ => method_not_found("Input", method),
        }
    }
}
