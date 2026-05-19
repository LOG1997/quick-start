use gpui::{AnyElement, Context, IntoElement, Window, div};
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};
pub struct SearchView;

impl SearchView {
    pub fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn render_search(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        div().into_any_element()
    }
}
