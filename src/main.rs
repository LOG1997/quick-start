mod background;
mod view;
use background::background::run_background;
use gpui::*;
use gpui_component::button::ButtonVariants;
use gpui_component::*;
use gpui_component::{TitleBar, badge::Badge, button::Button, menu::AppMenuBar};
use gtk;
use tray_icon::{Icon, TrayIconBuilder};
use view::view::AppRoot;

fn main() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    };
    run_background();
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        let _ = gtk::init(); // 主动初始化 GTK
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("system-tray - tray icon library!")
            // .with_icon(Icon)
            .build()
            .unwrap();
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
                    let app_root = cx.new(|_| AppRoot);
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
