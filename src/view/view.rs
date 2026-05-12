use crate::view::components::header::app_header;
use crate::view::components::input::InputComponent;
use crate::view::config::form::FomrComponent;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;

pub struct AppRoot {
    pub form: Entity<FomrComponent>,
}

impl AppRoot {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let form = _cx.new(|cx| FomrComponent::new(_window, cx));
        Self { form }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let form_wrapper = self
            .form
            .update(_cx, |form_ex, cx| form_ex.render_form(_window, cx));
        div()
            .size_full()
            .v_flex()
            .child(app_header())
            .child(form_wrapper)
    }
}
