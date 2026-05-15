use crate::view::components::form::FormComponen;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::IndexPath;
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::select::{
    SearchableVec, Select, SelectDelegate, SelectEvent, SelectGroup, SelectItem, SelectState,
};

pub struct SelectComponent {
    pub select_state: Entity<SelectState<Vec<&'static str>>>,
    pub value: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl SelectComponent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, options: Vec<&'static str>) -> Self {
        let select_state = cx.new(|cx| SelectState::new(options, None, window, cx));
        let _subscriptions = vec![cx.subscribe_in(&select_state, window, {
            move |this, _, ev: &SelectEvent<Vec<&'static str>>, _window, _cx| match ev {
                SelectEvent::Confirm(value) => {
                    if let Some(selected_value) = value {
                        this.value = selected_value.to_string().into();
                        println!("Selected: {:?}", selected_value);
                    } else {
                        println!("Selection cleared");
                    }
                    _cx.notify()
                }
                _ => {}
            }
        })];

        Self {
            select_state,
            value: SharedString::default(),
            _subscriptions,
        }
    }
    pub fn set_value(&mut self, value: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        println!("设置值");
        self.value = value.clone();
        self.select_state.update(cx, move |state, _cx| {
            state.set_selected_index(Some(IndexPath::new(1)), window, _cx)
        });
        cx.notify();
    }
}
