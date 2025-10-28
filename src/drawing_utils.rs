use ratatui::widgets::{Block, BorderType, Borders};

pub fn widget_block(border_type: BorderType) -> Block<'static> {
    Block::default()
        .border_type(border_type)
        .borders(Borders::all())
}
