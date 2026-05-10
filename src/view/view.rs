use crate::view::components::header::app_header;
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::*;

pub struct AppRoot;
impl Render for AppRoot {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let helloworld = div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            );
        div()
            .size_full()
            .v_flex()
            // 👇 直接放你的两个组件
            .child(app_header()) // 组件1
            .child(helloworld) // 组件2
    }
}
