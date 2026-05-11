use crate::view::config::input::InputComponent;
use crate::view::config::select::SelectComponent;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::select::{
    SearchableVec, Select, SelectDelegate, SelectEvent, SelectGroup, SelectItem, SelectState,
};

pub struct FomrComponent {
    pub name_field: Entity<InputComponent>,
    pub value_field: Entity<InputComponent>,
    pub type_select_field: Entity<SelectComponent>,
}

impl FomrComponent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let name_field = _cx.new(|cx| InputComponent::new(_window, cx));
        let value_field = _cx.new(|cx| InputComponent::new(_window, cx));
        let type_select_field = _cx.new(|cx| SelectComponent::new(_window, cx, vec!["app", "web"]));
        Self {
            name_field,
            value_field,
            type_select_field,
        }
    }

    pub fn render_form(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Form {
        let name_field = self.name_field.update(_cx, |example, _| {
            Input::new(&example.input_state) // 正确传递两个参数
        });
        let value_field = self.value_field.update(_cx, |example, _| {
            Input::new(&example.input_state) // 正确传递两个参数
        });
        let type_select_field = self
            .type_select_field
            .update(_cx, |example, _| Select::new(&example.select_state));

        let form_block = v_form()
            .child(field().label("Name").child(name_field))
            .child(field().label("Value").child(value_field))
            .child(field().label("Type").child(type_select_field));
        form_block
    }
}
