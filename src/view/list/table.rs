use crate::DB;
use crate::sqlite::db::Config;
use crate::{Page, Router};
use gpui::*;
use gpui_component::button::{Button, ButtonGroup, ButtonVariants};
use gpui_component::{
    ActiveTheme, Sizable,
    progress::Progress,
    tab::{Tab, TabBar},
    table::{Column, ColumnSort, DataTable, TableDelegate, TableState},
    v_flex,
};

pub struct ConfigTableDelegate {
    pub consfig_list: Vec<Config>,
    pub columns: Vec<Column>,
}

impl ConfigTableDelegate {
    pub fn new() -> Self {
        Self {
            consfig_list: Vec::new(),
            columns: vec![
                Column::new("name", "Name").width(70.),
                Column::new("value", "Value").width(120.),
                Column::new("command_type", "Type").width(80.),
                Column::new("operate", "Operate").width(80.),
            ],
        }
    }

    pub fn update_config_list(&mut self) {
        let config_list = DB.config_repo.find_all();
        self.consfig_list = config_list.unwrap_or_default();
    }
}

impl TableDelegate for ConfigTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.consfig_list.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> Column {
        self.columns[col_ix].clone()
    }
    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let Some(config_data) = self.consfig_list.get(row_ix) else {
            return div().into_any_element();
        };
        let config_data_edit = config_data.clone();
        let config_data_delete = config_data.clone();
        let operate_button = ButtonGroup::new("btn-group")
            .flex()
            .gap_2()
            .child(
                Button::new("编辑")
                    .link()
                    .text_color(cx.theme().blue)
                    .label("编辑")
                    .on_click(move |_, _, cx| {
                        cx.update_global::<Router, _>(|router, _| {
                            router.current_page = Page::Config {
                                id: Some(config_data_edit.id.clone()),
                            };
                        });
                    }),
            )
            .child(
                Button::new("删除")
                    .link()
                    .text_color(cx.theme().red)
                    .label("删除")
                    .on_click(move |_, _, cx| {
                        DB.config_repo.delete(config_data_delete.id.clone());
                    }),
            );
        match col_ix {
            0 => div()
                .child(format!("{}", config_data.name))
                .into_any_element(),
            1 => div()
                .child(format!("{}", config_data.value))
                .into_any_element(),
            2 => div()
                .child(format!("{}", config_data.command_type))
                .into_any_element(),
            3 => div().child(div().child(operate_button)).into_any_element(),
            _ => div().into_any_element(),
        }
    }

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) {
    }
}
