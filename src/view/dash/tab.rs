use gpui::{AnyElement, Context, IntoElement, Window, div};
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};
pub struct TabView;

impl TabView {
    pub fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn render_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        div().into_any_element()
    }
}
