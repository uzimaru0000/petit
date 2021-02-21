use crate::component::ComponentWithContext;
use tui::widgets::Paragraph;
use tui::{backend::Backend, layout::Rect, Frame};
use unicode_width::UnicodeWidthStr;

pub struct Input {
    pub value: String,
}

impl<'a, B: 'a> ComponentWithContext<Paragraph<'a>, &mut Frame<'a, B>> for Input
where
    B: Backend,
{
    fn view(&self, area: &Rect, ctx: &mut Frame<'a, B>) -> Paragraph<'a> {
        ctx.set_cursor(area.x + self.value.width() as u16 + 1, area.y + 1);
        Paragraph::new(self.value.clone())
    }
}
