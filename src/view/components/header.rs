use gpui::{IntoElement, ParentElement, div};
use gpui_component::{TitleBar, WindowExt};

pub fn app_header() -> impl IntoElement {
    TitleBar::new()
        .on_close_window(|_, window, cx| {
            window.push_notification("Saving before close...", cx);
            window.remove_window();
        })
        .child(div().child("Custom Close Behavior"))
}
