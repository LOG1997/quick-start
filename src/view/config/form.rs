use crate::DB;
use crate::background::create_db::Config;
use crate::view::config::button::ButtonComponent;
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
    pub button: Entity<ButtonComponent>,
}

#[derive(Debug)]
pub struct FormValue {
    pub name: SharedString,
    pub value: SharedString,
    pub type_: SharedString,
}

impl FomrComponent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let name_field = _cx.new(|cx| InputComponent::new(_window, cx, "enter name"));
        let value_field = _cx.new(|cx| InputComponent::new(_window, cx, "enter value"));
        let type_select_field = _cx.new(|cx| SelectComponent::new(_window, cx, vec!["app", "web"]));
        let submit_button = _cx.new(|_| ButtonComponent::new("Submit"));
        Self {
            name_field,
            value_field,
            type_select_field,
            button: submit_button,
        }
    }

    pub fn render_form(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Form {
        let name_field = self.name_field.update(_cx, |field, _| {
            Input::new(&field.input_state) // 正确传递两个参数
        });
        let value_field = self.value_field.update(_cx, |field, _| {
            Input::new(&field.input_state) // 正确传递两个参数
        });
        let type_select_field = self
            .type_select_field
            .update(_cx, |field, _| Select::new(&field.select_state));
        let submit_button = Button::new("submit").label("submit").on_click(_cx.listener(
            |this, event, window, cx| {
                // 这里可以调用 this 的方法
                let all_value = this.get_form_value(cx);
                println!("this is u:{:?}", all_value);
                let db = DB.config_repo.find_all();
                // DB.config_repo
                //     .save(&Config {
                //         id: 11,
                //         name: all_value.name.to_string(),
                //         value: all_value.value.to_string(),
                //     })
                //     .unwrap()
                println!("db:{:?}", db);
            },
        ));
        // let submit_button = self.button.update(_cx, |_button, _| {
        //     Button::new("submit").label("submit").on_click(cx.listener(
        //         |this, event, window, cx1| {
        //             // 这里可以调用 this 的方法
        //             this.button.;
        //         },
        //     ))
        // });
        let form_block = v_form()
            .child(field().label("Name").child(name_field))
            .child(field().label("Value").child(value_field))
            .child(field().label("Type").child(type_select_field))
            .child(field().child(submit_button));
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
}
