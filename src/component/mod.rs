use tui::layout::Rect;

pub mod input;
pub mod tweet;

pub trait Component<W: tui::widgets::Widget> {
    fn view(&self, area: &Rect) -> W;
}

pub trait ComponentWithContext<W: tui::widgets::Widget, Ctx> {
    fn view(&self, area: &Rect, ctx: Ctx) -> W;
}
