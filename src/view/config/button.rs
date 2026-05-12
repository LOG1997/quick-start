use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;

pub struct ButtonComponent {
    pub button: Button,
}

impl ButtonComponent {
    pub fn new(label: &'static str) -> Self {
        let button = Button::new(label).label(label).on_click(move |_, _, _| {
            println!("label:{}", label);
            // click_fn()
        });
        Self { button }
    }
}
