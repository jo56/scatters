use crate::drawing_utils::widget_block;
use crate::scatters::ScatteredWord;
use crate::styling::AppStyling;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub struct App {
    pub scattered_words: Vec<ScatteredWord>,
    pub word_count: usize,
    pub styling: AppStyling,
    pub selected_word_index: Option<usize>,
    pub highlighted_words: Vec<usize>,  // Track all highlighted words
    pub density: f32,  // Density multiplier for word generation (0.1 to 6.0)
    pub use_dimmed_current: bool,  // If true, current selection uses visited color instead of bright color
}

impl App {
    pub fn new(
        scattered_words: Vec<ScatteredWord>,
        word_count: usize,
        styling: AppStyling,
    ) -> Self {
        Self {
            scattered_words,
            word_count,
            styling,
            selected_word_index: Some(0),
            highlighted_words: vec![0],  // Start with first word highlighted
            density: 1.0,  // Start at default density
            use_dimmed_current: false,  // Start with bright current selection
        }
    }

    pub fn update_words(&mut self, scattered_words: Vec<ScatteredWord>) {
        self.scattered_words = scattered_words;
        self.selected_word_index = Some(0);
        self.highlighted_words = vec![0];  // Reset to single highlighted word on reroll
    }

    pub fn select_next_word(&mut self) {
        if let Some(index) = self.selected_word_index {
            let next_index = (index + 1) % self.scattered_words.len();
            self.selected_word_index = Some(next_index);
            // Add to highlighted words if not already there
            if !self.highlighted_words.contains(&next_index) {
                self.highlighted_words.push(next_index);
            }
        }
    }

    pub fn select_prev_word(&mut self) {
        if let Some(index) = self.selected_word_index {
            let prev_index = if index == 0 {
                self.scattered_words.len().saturating_sub(1)
            } else {
                index - 1
            };
            self.selected_word_index = Some(prev_index);
            // Add to highlighted words if not already there
            if !self.highlighted_words.contains(&prev_index) {
                self.highlighted_words.push(prev_index);
            }
        }
    }

    pub fn increase_density(&mut self) {
        self.density = ((self.density + 0.3) * 10.0).round() / 10.0;
        self.density = self.density.min(6.0);
    }

    pub fn decrease_density(&mut self) {
        self.density = ((self.density - 0.3) * 10.0).round() / 10.0;
        self.density = self.density.max(0.1);
    }

    pub fn toggle_current_highlight(&mut self) {
        // Toggle between bright current selection and dimmed (visited color) current selection
        self.use_dimmed_current = !self.use_dimmed_current;
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let frame_area = f.area();

    // Render unified white background for monochrome theme
    if app.styling.use_background_fill {
        let background = widget_block(app.styling.border_type)
            .style(app.styling.text_style)
            .borders(ratatui::widgets::Borders::empty());
        f.render_widget(background, frame_area);
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(frame_area);

    render_sidebar(f, main_layout[0], app);
    render_canvas(f, main_layout[1], app);
}

fn render_sidebar(f: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(3)])
        .margin(1)
        .split(area);

    // Controls section
    let mut controls_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Scatters ", app.styling.text_style)));

    if app.styling.use_background_fill {
        controls_block = controls_block.style(app.styling.text_style);
    }

    let count_text = format!("words {} / {}", app.scattered_words.len(), app.word_count);
    let highlighted_text = format!("selected {} / {}", app.highlighted_words.len(), app.scattered_words.len());

    let controls_text = vec![
        Line::from(vec![
            Span::styled("↑/↓", app.styling.text_style),
            Span::styled(" - density", app.styling.text_style),
        ]),
        Line::from(vec![
            Span::styled("←/→", app.styling.text_style),
            Span::styled(" - highlight", app.styling.text_style),
        ]),
        Line::from(vec![
            Span::styled("r", app.styling.text_style),
            Span::styled(" - reroll", app.styling.text_style),
        ]),
        Line::from(Span::styled(count_text, app.styling.text_style)),
        Line::from(Span::styled(highlighted_text, app.styling.text_style)),
    ];

    let controls = Paragraph::new(controls_text)
        .block(controls_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(controls, sections[0]);

    // Info section
    let mut info_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Density ", app.styling.text_style)));

    if app.styling.use_background_fill {
        info_block = info_block.style(app.styling.text_style);
    }

    // Create density bar visualization
    // Density ranges from 0.1 to 6.0, bar width is based on section width
    // Account for borders (2 chars) to span full width
    let available_width = sections[1].width.saturating_sub(2).max(8) as usize;
    let bar_width = available_width; // Use full available width
    let density_ratio = (app.density - 0.1) / (6.0 - 0.1); // Normalize to 0.0-1.0
    let filled_width = (density_ratio * bar_width as f32) as usize;
    let empty_width = bar_width - filled_width;

    let filled_bar = "█".repeat(filled_width);
    let empty_bar = " ".repeat(empty_width);

    let info_text = vec![
        Line::from(vec![
            Span::styled(filled_bar, app.styling.density_bar_style),
            Span::styled(empty_bar, app.styling.text_style),
        ]),
    ];

    let info = Paragraph::new(info_text)
        .block(info_block)
        .alignment(Alignment::Left);

    f.render_widget(info, sections[1]);
}

fn render_canvas(f: &mut Frame, area: Rect, app: &App) {
    // Create canvas block with border and background
    let mut canvas_block = widget_block(app.styling.border_type)
        .border_style(app.styling.highlighted_border_style);

    if app.styling.use_background_fill {
        canvas_block = canvas_block.style(app.styling.text_style);
    }

    let inner = canvas_block.inner(area);
    f.render_widget(canvas_block, area);

    // Render scattered words with highlight effect for selected word
    for (index, scattered) in app.scattered_words.iter().enumerate() {
        let x_pos = inner.x.saturating_add(scattered.x.min(inner.width.saturating_sub(1)));
        let y_pos = inner.y.saturating_add(scattered.y.min(inner.height.saturating_sub(1)));

        if x_pos >= inner.x
            && x_pos < inner.x + inner.width
            && y_pos >= inner.y
            && y_pos < inner.y + inner.height
        {
            let available_width = (inner.x + inner.width).saturating_sub(x_pos);

            if available_width > 0 {
                // Truncate word at character boundary if it exceeds available width
                let word = if scattered.word.chars().count() > available_width as usize {
                    scattered.word
                        .chars()
                        .take(available_width as usize)
                        .collect::<String>()
                } else {
                    scattered.word.clone()
                };

                let word_rect = Rect {
                    x: x_pos,
                    y: y_pos,
                    width: word.chars().count().min(available_width as usize) as u16,
                    height: 1,
                };

                // Apply three-tier styling: current selected, previously highlighted, or default
                let word_style = if app.selected_word_index == Some(index) {
                    if app.use_dimmed_current {
                        app.styling.selected_text_style  // Currently selected but dimmed (same as visited)
                    } else {
                        app.styling.current_selected_style  // Currently selected - brightest
                    }
                } else if app.highlighted_words.contains(&index) {
                    app.styling.selected_text_style  // Previously visited
                } else {
                    app.styling.text_style  // Not visited
                };

                let word_widget = Paragraph::new(Line::from(Span::styled(&word, word_style)));
                f.render_widget(word_widget, word_rect);
            }
        }
    }
}
