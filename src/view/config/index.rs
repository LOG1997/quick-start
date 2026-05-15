use crate::DB;
use crate::sqlite::db::Config;
use crate::view::config::form::FomrComponent;
use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, ParentElement, Styled, Window, div,
};

pub struct ConfigView {
    form: Entity<FomrComponent>,
    form_value: Option<Config>,
}

impl ConfigView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let form = cx.new(|form_cx| FomrComponent::new(window, form_cx, None));
        Self {
            form,
            form_value: None,
        }
    }

    pub fn render_config(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let form_wrapper = self.form.update(cx, |form_com, form_cx| {
            form_com.render_form(window, form_cx)
        });
        div().size_full().child(form_wrapper).into_any_element()
    }
    pub fn init_for_id(&mut self, id: Option<String>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(id) = id {
            // 查询数据（只执行一次）
            let data = DB.config_repo.find_by_id(id.to_string());
            println!("data:{:?}", data);
            self.form_value = data.unwrap();
            self.form.update(cx, |form_com, form_cx| {
                form_com.set_form_value(self.form_value.clone(), window, form_cx)
            });
            cx.notify(); // 通知视图刷新
        } else {
            // 新建模式，清空表单
            println!("没有id");
            self.form.update(cx, |form_com, form_cx| {
                form_com.clear_form(window, form_cx);
            });
            cx.notify();
        }
    }
}
