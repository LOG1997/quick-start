use crate::view::components::form::FormComponen;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
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
    // _subscriptions: Vec<Subscription>,
}

impl SelectComponent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, options: Vec<&'static str>) -> Self {
        let select_state = cx.new(|cx| SelectState::new(options, None, window, cx));
        // let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
        //     let input_state = input_state.clone();
        //     move |this, _, ev: &InputEvent, _window, cx| match ev {
        //         InputEvent::Change => {
        //             let value = input_state.read(cx).value();
        //             println!("value is {}", value);
        //             this.value = value;
        //             cx.notify()
        //         }
        //         _ => {}
        //     }
        // })];

        Self {
            select_state,
            value: SharedString::default(),
            // _subscriptions,
        }
    }
}
