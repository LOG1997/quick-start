use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, ParentElement, Styled, Window, div,
};
use gpui_component::Sizable;
use gpui_component::table::{DataTable, TableState};

use crate::view::list::table::ConfigTableDelegate;
use crate::view::list::table_toolbar::Toolbar;

pub struct TableView {
    table: Entity<TableState<ConfigTableDelegate>>,
    toolbar: Entity<Toolbar>,
}

impl TableView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let table = cx.new(|_cx| {
            TableState::new(ConfigTableDelegate::new(), window, _cx)
                .col_selectable(false)
                .col_movable(false)
        });
        let toolbar = cx.new(|_cx| Toolbar::new());
        Self { table, toolbar }
    }

    pub fn render_list(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let table_wrapper = self.table.update(cx, |table, _| {
            table.delegate_mut().update_config_list();
            DataTable::new(&self.table)
                .bordered(false)
                .stripe(true)
                .small()
        });

        let toolbar = self.toolbar.update(cx, |toolbar, tool_cx| {
            toolbar.render_toolbar(window, tool_cx)
        });
        div()
            .size_full()
            .child(toolbar)
            .child(table_wrapper)
            .into_any_element()
    }
}
