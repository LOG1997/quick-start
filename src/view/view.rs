use crate::sqlite::db::Config;
use crate::view::components::header::app_header;
use crate::view::components::input::InputComponent;
use crate::view::config::form::FomrComponent;
use crate::view::list::table::ConfigTableDelegate;
use crate::{Page, Router};
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::table::{DataTable, TableState};
use gpui_component::*;

pub struct AppRoot {
    pub form: Entity<FomrComponent>,
    pub table: Entity<TableState<ConfigTableDelegate>>,
}

impl AppRoot {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let form = _cx.new(|cx| FomrComponent::new(_window, cx));
        let table = _cx.new(|cx| {
            TableState::new(ConfigTableDelegate::new(), _window, cx)
                .col_selectable(false)
                .col_movable(false)
        });
        Self { form, table }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let form_wrapper = self
            .form
            .update(_cx, |form_ex, cx| form_ex.render_form(_window, cx));
        let table_wrapper = self.table.update(_cx, |table_ex, cx| {
            table_ex.delegate_mut().update_config_list();
            DataTable::new(&self.table)
                .bordered(false)
                .stripe(true)
                .small()
        });
        let router = _cx.global::<Router>();
        let view = match router.current_page {
            Page::List => table_wrapper.into_any_element(),
            Page::Config { id: None } => form_wrapper.into_any_element(),
            Page::Config { id: Some(_) } => form_wrapper.into_any_element(),
        };
        div().size_full().v_flex().child(app_header()).child(view)
    }
}
