mod background;
mod sqlite;
mod view;
use crate::background::create_db::AppDatabase;
use background::background::run_background;
use gpui::*;
use gpui_component::*;
use lazy_static::lazy_static;
use view::view::AppRoot;

lazy_static! {
    static ref DB: AppDatabase = {
        // 打开/创建数据库文件（app.db 会自动生成在项目根目录）
        let repo=AppDatabase::new("app.db");
        repo.unwrap()
    };
}
fn main() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    };
    run_background();
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
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
