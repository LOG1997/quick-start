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

    // pub fn render_form(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Input {
    //     self.form.update(_cx, |example, _| {
    //         Input::new(&example.input_state) // 正确传递两个参数
    //     })
    // }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let name_input = _cx.new(|cx| InputState::new(_window, cx).placeholder("Enter your name"));
        let email_input =
            _cx.new(|cx| InputState::new(_window, cx).placeholder("Enter your email"));
        // let helloworld = v_form()
        //     .child(
        //         field()
        //             .label("Run")
        //             .child(self.render_input(_window, _cx))
        //             .child(self.input.read(_cx).value.clone()),
        //     )
        //     .child(field().label("Name").child(Input::new(&name_input)))
        //     .child(
        //         field()
        //             .label("Email")
        //             .child(Input::new(&email_input))
        //             .required(true),
        //     );
        let form_wrapper = self
            .form
            .update(_cx, |form_ex, cx| form_ex.render_form(_window, cx));
        div()
            .size_full()
            .v_flex()
            .child(app_header()) // 组件1
            // .child(helloworld) // 组件2
            .child(form_wrapper)
    }
}
