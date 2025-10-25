use ratatui::widgets::{Block, BorderType, Borders};

/// Return a widget block, similar to bottom's widget_block function.
pub fn widget_block(border_type: BorderType) -> Block<'static> {
    Block::default()
        .border_type(border_type)
        .borders(Borders::all())
}
