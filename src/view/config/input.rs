use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;

pub struct InputComponent {
    pub input_state: Entity<InputState>,
    pub value: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl InputComponent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, placeholder: &str) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value = input_state.read(cx).value();
                    println!("value is {}", value);
                    this.value = value;
                    cx.notify()
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            value: SharedString::default(),
            _subscriptions,
        }
    }

    pub fn set_value(&mut self, value: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        self.value = value.clone();
        self.input_state
            .update(cx, move |state, _cx| state.set_value(value, window, _cx));
        cx.notify();
    }
}
