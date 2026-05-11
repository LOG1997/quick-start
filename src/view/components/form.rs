use crate::view::components::header::app_header;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;

enum FieldType {
    Input,
    Select,
}
pub struct FormComponen {
    pub field_type: FieldType,
}
