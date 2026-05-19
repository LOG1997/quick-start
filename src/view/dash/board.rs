use gpui::{AnyElement, Context, IntoElement, ParentElement, Window, div};
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};
pub struct BoardView;

impl BoardView {
    pub fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn render_board(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        div().child("11212121").into_any_element()
    }
}
