use crate::DB;
use crate::sqlite::db::Config;
use crate::view::config::input::InputComponent;
use crate::view::config::select::SelectComponent;
use crate::{Page, Router};
use gpui::*;
use gpui::{Context, Window};
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};
use gpui_component::form::{Form, field, v_form};
use gpui_component::input::{Input, InputState};
use gpui_component::select::Select;
use uuid::Uuid;

pub struct FomrComponent {
    pub name_field: Entity<InputComponent>,
    pub value_field: Entity<InputComponent>,
    pub type_select_field: Entity<SelectComponent>,
    pub form_value: Option<Config>,
}

#[derive(Debug)]
pub struct FormValue {
    pub name: SharedString,
    pub value: SharedString,
    pub type_: SharedString,
}

impl FomrComponent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, value_id: Option<String>) -> Self {
        println!("is new");
        if let Some(id) = value_id {
            let _initial_data = DB.config_repo.find_by_id(id);
            println!("ini data:P{:?}", _initial_data);
        }

        let name_field = _cx.new(|cx| InputComponent::new(_window, cx, "enter name"));
        let value_field = _cx.new(|cx| InputComponent::new(_window, cx, "enter value"));
        let type_select_field = _cx.new(|cx| SelectComponent::new(_window, cx, vec!["app", "web"]));
        let mut form_value = None;
        Self {
            name_field,
            value_field,
            type_select_field,
            form_value,
        }
    }

    pub fn render_form(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Form {
        println!("two new");
        let name_field = self
            .name_field
            .update(_cx, |field, field_cx| Input::new(&field.input_state));
        let value_field = self
            .value_field
            .update(_cx, |field, _| Input::new(&field.input_state));
        let type_select_field = self
            .type_select_field
            .update(_cx, |field, _| Select::new(&field.select_state));
        let operate_button = ButtonGroup::new("form_operate")
            .flex()
            .justify_center()
            .gap_2()
            .child(
                Button::new("cancel")
                    .label("取消")
                    .on_click(move |_, _, cx| {
                        cx.update_global::<Router, _>(|router, _| {
                            router.current_page = Page::List;
                        });
                    }),
            )
            .child(
                Button::new("submit")
                    .label("提交")
                    .primary()
                    .on_click(_cx.listener(move |this, _, window, cx| {
                        // 这里可以调用 this 的方法
                        let all_value = this.get_form_value(cx);
                        let id = this
                            .form_value
                            .as_ref()
                            .map(|s| s.id.to_string())
                            .unwrap_or(Uuid::new_v4().to_string());
                        DB.config_repo
                            .save_config(&Config {
                                id,
                                name: all_value.name.to_string(),
                                value: all_value.value.to_string(),
                                command_type: all_value.type_.to_string(),
                                created_at: None,
                                updated_at: None,
                            })
                            .unwrap();
                        this.clear_form(window, cx);
                        cx.update_global::<Router, _>(|router, _| router.current_page = Page::List);
                    })),
            );

        let form_block = v_form()
            .child(field().required(true).label("Name").child(name_field))
            .child(field().required(true).label("Value").child(value_field))
            .child(
                field()
                    .required(true)
                    .label("Type")
                    .child(type_select_field),
            )
            .child(field().flex().justify_center().child(operate_button));
        form_block
    }

    pub fn get_name(&self, _cx: &mut Context<Self>) -> SharedString {
        self.name_field.update(_cx, |field, _| field.value.clone())
    }

    pub fn get_value(&self, _cx: &mut Context<Self>) -> SharedString {
        self.value_field.update(_cx, |field, _| field.value.clone())
    }

    pub fn get_type(&self, _cx: &mut Context<Self>) -> SharedString {
        self.type_select_field
            .update(_cx, |field, _| field.value.clone())
    }

    pub fn get_form_value(&self, _cx: &mut Context<Self>) -> FormValue {
        let result = FormValue {
            name: self.get_name(_cx),
            value: self.get_value(_cx),
            type_: self.get_type(_cx),
        };
        println!("result:{:?}", result);
        result
    }

    pub fn clear_form(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("清除");
        self.name_field.update(_cx, |field, field_cx| {
            field.set_value(SharedString::default(), _window, field_cx)
        });
        self.value_field.update(_cx, |field, field_cx| {
            field.set_value(SharedString::default(), _window, field_cx)
        });
        self.type_select_field.update(_cx, |field, field_cx| {
            field.set_value(SharedString::default(), _window, field_cx)
        });
        _cx.notify();
    }
    pub fn set_form_value(
        &mut self,
        form_value: Option<Config>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.form_value = form_value.clone();
        let name_value = SharedString::new(&form_value.clone().unwrap_or_default().name);
        let value_value = SharedString::new(&form_value.clone().unwrap_or_default().value);
        let type_value = SharedString::new(&form_value.clone().unwrap_or_default().command_type);
        self.name_field.update(_cx, |field, field_cx| {
            field.set_value(name_value, _window, field_cx)
        });
        self.value_field.update(_cx, |field, field_cx| {
            field.set_value(value_value, _window, field_cx)
        });
        self.type_select_field.update(_cx, |field, field_cx| {
            field.set_value(type_value, _window, field_cx)
        })
    }
}
