use crate::view::dash::index::DashView;
use crate::{Page, Router};
use gpui::*;
use gpui_component::Root;
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
                ButtonGroup::new("tool_bar-group")
                    .child(
                        Button::new("toolbar-new")
                            .label("新建")
                            .primary()
                            .cursor_pointer()
                            .on_click(move |_, _, cx| {
                                cx.update_global::<Router, _>(|router, _| {
                                    router.current_page = Page::Config { id: None };
                                });
                            }),
                    )
                    .child(
                        Button::new("toolbar-dash")
                            .label("完成")
                            .primary()
                            .cursor_pointer()
                            .on_click(move |_, _, cx| {
                                cx.open_window(
                                    WindowOptions {
                                        // 在这里配置新窗口的属性，例如：
                                        titlebar: None, // 创建一个无边框窗口
                                        ..Default::default()
                                    },
                                    |window, cx| {
                                        window.set_window_title("123");
                                        // 创建 AppRoot 视图
                                        let app_root = cx.new(|cx| DashView::new(window, cx));
                                        // 用 Root 包裹它
                                        let root = cx.new(|cx| Root::new(app_root, window, cx));
                                        root
                                    },
                                );
                            }),
                    ),
            )
            .into_any_element()
    }
}
