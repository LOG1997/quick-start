use crate::view::components::header::app_header;
use crate::view::config::index::ConfigView;
use crate::view::list::index::TableView;
use crate::{Page, Router};
use gpui::*;
use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::*;

pub struct AppRoot {
    pub form: Entity<ConfigView>,
    pub table: Entity<TableView>,
    current_route: Option<Page>,
}

impl AppRoot {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let form = _cx.new(|cx| ConfigView::new(_window, cx));
        let table = _cx.new(|cx| TableView::new(_window, cx));
        Self {
            form,
            table,
            current_route: None,
        }
    }
}
impl AppRoot {
    fn on_route_change(&mut self, new_page: &Page, _window: &mut Window, cx: &mut Context<Self>) {
        match new_page {
            Page::Config { id } => {
                // 进入 Config 页面，执行一次初始化（如加载数据）
                let id_owned = id.clone();
                self.form.update(cx, move |form, form_cx| {
                    form.init_for_id(id_owned, form_cx); // 加载数据并更新表单
                });
            }
            Page::List => {
                // 进入 List 页面，如果需要初始化也可在此处理
            }
        }
    }
}
impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let router = _cx.global::<Router>();
        let new_page = router.current_page.clone();
        let current_page = router.current_page.clone();

        if Some(&new_page) != self.current_route.as_ref() {
            // 路由变化，执行进入页面的初始化
            println!("路由变化");
            self.on_route_change(&new_page, _window, _cx);
            self.current_route = Some(new_page.clone());
        }
        let view = match &current_page {
            Page::List => self
                .table
                .update(_cx, move |table, cx| table.render_list(_window, cx))
                .into_any_element(),
            Page::Config { id } => {
                // 进入 Config 页面，执行一次初始化（如加载数据）

                self.form
                    .update(_cx, |form_ex, cx| form_ex.render_config(_window, cx))
            }
        };
        div().size_full().v_flex().child(app_header()).child(view)
    }
}
