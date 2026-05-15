use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;

pub struct InputComponentBuilder {
    placeholder: Option<String>,
    validate: Option<Box<dyn Fn(&str, &mut Context<InputState>) -> bool + 'static>>,
}

impl Default for InputComponentBuilder {
    fn default() -> Self {
        Self {
            placeholder: None,
            validate: None,
        }
    }
}

impl InputComponentBuilder {
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &mut Context<InputState>) -> bool + 'static,
    {
        self.validate = Some(Box::new(f));
        self
    }

    pub fn build(self, window: &mut Window, cx: &mut Context<InputComponent>) -> InputComponent {
        // 在 cx.new 中根据配置构建 InputState
        let input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            if let Some(validate) = self.validate {
                state = state.validate(validate);
            }
            if let Some(placeholder) = self.placeholder {
                state = state.placeholder(&placeholder);
            }
            state
        });

        // 订阅输入变化事件（与原 new 中相同）
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

        InputComponent {
            input_state,
            value: SharedString::default(),
            _subscriptions,
        }
    }
}
pub struct InputComponent {
    pub input_state: Entity<InputState>,
    pub value: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl InputComponent {
    pub fn new() -> InputComponentBuilder {
        InputComponentBuilder::default()
    }

    pub fn set_value(&mut self, value: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        self.value = value.clone();
        self.input_state
            .update(cx, move |state, _cx| state.set_value(value, window, _cx));
        cx.notify();
    }
}
