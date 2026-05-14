use crate::{Page, Router};
use gpui::*;
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};

/// 工具栏组件（只负责渲染，交互通过回调注入）
pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }

    pub fn render_toolbar(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .justify_end()
            .my_2()
            .child(
                ButtonGroup::new("tool_bar-group").child(
                    Button::new("toolbar-new")
                        .label("新建")
                        .primary()
                        .cursor_pointer()
                        .on_click(move |_, _, cx| {
                            cx.update_global::<Router, _>(|router, _| {
                                router.current_page = Page::Config { id: None };
                            });
                        }),
                ),
            )
            .into_any_element()
    }
}
