use crate::view::dash::{board::BoardView, search::SearchView, tab::TabView};
use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    div,
};
use gpui_component::Sizable;
use gpui_component::table::{DataTable, TableState};

pub struct DashView {
    pub searchView: Entity<SearchView>,
    pub tabView: Entity<TabView>,
    pub boardView: Entity<BoardView>,
}

impl DashView {
    pub fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        let searchView = cx.new(|_cx| SearchView::new(window, _cx));
        let tabView = cx.new(|_cx| TabView::new(window, _cx));
        let boardView = cx.new(|_cx| BoardView::new(window, _cx));
        Self {
            searchView,
            tabView,
            boardView,
        }
    }

    pub fn render_dashboard(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let searchView = self
            .searchView
            .update(cx, |view, cx| view.render_search(window, cx));
        let tabView = self
            .tabView
            .update(cx, |view, cx| view.render_tab(window, cx));
        let boardView = self
            .boardView
            .update(cx, |view, cx| view.render_board(window, cx));
        div()
            .child(searchView)
            .child(tabView)
            .child(boardView)
            .into_any_element()
    }
}
impl Render for DashView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        self.render_dashboard(window, cx)
    }
}
