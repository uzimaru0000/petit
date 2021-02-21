use tui::layout::Rect;

mod tweet;

pub trait Widget<W: tui::widgets::Widget> {
    fn view(&self, area: &Rect) -> W;
}

pub trait StatefulWidget<W: tui::widgets::Widget> {
    type State;
    fn view(&self, area: &Rect, state: Self::State) -> W;
}
