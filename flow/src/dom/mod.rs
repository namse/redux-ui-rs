mod any_clone_partial_eq;
pub mod li;
mod start;
pub mod style;

use crate::{render, Element, IntoElement};
use any_clone_partial_eq::*;
pub use li::*;
pub use start::*;
pub use style::*;
use web_sys::HtmlElement;

impl IntoElement for &str {
    fn into_element(self) -> Element {
        text(self)
    }
}
impl IntoElement for &String {
    fn into_element(self) -> Element {
        text(self)
    }
}
impl IntoElement for String {
    fn into_element(self) -> Element {
        text(self)
    }
}

fn text(text: impl ToString) -> Element {
    println!("text: {:?}", text.to_string());
    render(())
}

pub struct OnClick {
    event: Box<dyn AnyClonePartialEq>,
}

impl Clone for OnClick {
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone_box(),
        }
    }
}
impl PartialEq for OnClick {
    fn eq(&self, other: &Self) -> bool {
        self.event.equals(other.event.as_ref())
    }
}

pub fn on_click(event: impl std::any::Any + Clone + PartialEq) -> OnClick {
    OnClick {
        event: Box::new(event),
    }
}
