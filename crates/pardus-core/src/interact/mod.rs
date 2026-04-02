pub mod element;
pub mod actions;
pub mod form;
pub mod wait;
pub mod scroll;
#[cfg(feature = "js")]
pub mod js_interact;

pub use element::ElementHandle;
pub use actions::InteractionResult;
pub use form::FormState;
pub use scroll::ScrollDirection;
#[cfg(feature = "js")]
pub use js_interact::{js_click, js_type, js_scroll, js_submit};
