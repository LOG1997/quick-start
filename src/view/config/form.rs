use crate::view::components::form::FormComponen;
use crate::view::config::input::InputComponent;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
pub struct FomrComponent {
    pub name_field: Entity<InputComponent>,
    pub value_filed: Entity<InputComponent>,
}

impl FomrComponent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let name_field = _cx.new(|cx| InputComponent::new(_window, cx));
        let value_filed = _cx.new(|cx| InputComponent::new(_window, cx));
        Self {
            name_field,
            value_filed,
        }
    }

    pub fn render_input(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Input {
        self.name_field.update(_cx, |field, _| {
            Input::new(&field.input_state) // 正确传递两个参数
        })
        // self.value_field.update(_cx, |field, _| {
        //     Input::new(&field.input_state) // 正确传递两个参数
        // })
    }
}
