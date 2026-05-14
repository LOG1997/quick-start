mod background;
mod sqlite;
mod view;
use background::background::run_background;
use background::create_db::DB;
use gpui::*;
use gpui_component::*;
use view::view::AppRoot;

// 1. 定义路由状态（全局单例）
pub struct Router {
    current_page: Page,
    // 可选：历史栈
}

impl Global for Router {} // 假设 gpui_platform 中 Global trait 存在
impl Global for AppRoot {}
// 2. 页面枚举
#[derive(Clone, PartialEq)]
pub enum Page {
    Config { id: Option<String> },
    List,
}
fn main() {
    run_background();
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.set_global(Router {
            current_page: Page::List,
        });
        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: None,
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    window.set_window_title("123");

                    // 创建 AppRoot 视图
                    let app_root = cx.new(|cx| AppRoot::new(window, cx));
                    // 用 Root 包裹它
                    let root = cx.new(|cx| Root::new(app_root, window, cx));
                    root
                },
            )
            .expect("Failed to open window");
        })
        .detach();
    });
}
