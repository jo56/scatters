use crate::scatters::ScatteredWord;
use crate::styling::AppStyling;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Wrap, Block, BorderType, Borders},
    Frame,
};
use std::path::PathBuf;

pub struct App {
    pub scattered_words: Vec<ScatteredWord>,
    pub word_count: usize,
    pub styling: AppStyling,
    pub selected_word_index: Option<usize>,
    pub highlighted_words: Vec<usize>,  // Track all highlighted words
    pub density: f32,  // Density multiplier for word generation (0.1 to 6.0)
    pub use_dimmed_current: bool,  // If true, current selection uses visited color instead of bright color
    pub fullscreen_mode: bool,
    pub directory: PathBuf,  // Current directory being used
}

impl App {
    pub fn new(
        scattered_words: Vec<ScatteredWord>,
        word_count: usize,
        styling: AppStyling,
        directory: PathBuf,
    ) -> Self {
        Self {
            scattered_words,
            word_count,
            styling,
            selected_word_index: Some(0),
            highlighted_words: vec![0],  // Start with first word highlighted
            density: 1.0,  // Start at default density
            use_dimmed_current: false,  // Start with bright current selection
            fullscreen_mode: false,
            directory,
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

    pub fn increase_density(&mut self, bar_width: u16) {
        let density_per_pixel = (6.0 - 0.1) / bar_width.max(1) as f32;
        self.density = (self.density + density_per_pixel).min(6.0);
    }

    pub fn decrease_density(&mut self, bar_width: u16) {
        let density_per_pixel = (6.0 - 0.1) / bar_width.max(1) as f32;
        self.density = (self.density - density_per_pixel).max(0.1);
    }

    pub fn toggle_current_highlight(&mut self) {
        // Toggle between bright current selection and dimmed (visited color) current selection
        self.use_dimmed_current = !self.use_dimmed_current;
    }
}

pub fn get_density_bar_width(terminal_width: u16) -> u16 {
    let sidebar_width = terminal_width * 25 / 100;
    let section_width = sidebar_width.saturating_sub(2);
    section_width.saturating_sub(2).max(8)
}

pub fn calculate_sidebar_width_for_app(app: &App) -> u16 {
    // Calculate the longest text line in each section
    let count_text = format!("words {} / {}", app.scattered_words.len(), app.word_count);
    let highlighted_text = format!("selected {} / {}", app.highlighted_words.len(), app.scattered_words.len());

    // Scatters section: compare both lines
    let scatters_width = count_text.len().max(highlighted_text.len());

    // Density section: the bar + title
    let density_width = 20; // A reasonable default for the density bar

    // Controls section: find longest control line
    let controls_lines = [
        "↑/↓ - density",
        "←/→ - highlight",
        "r - reroll",
        "v - view",
        "q - quit",
    ];
    let controls_width = controls_lines.iter().map(|s| s.len()).max().unwrap_or(0);

    // Word and File sections: calculate width if a word is selected
    let (word_width, file_width) = if let Some(index) = app.selected_word_index {
        if let Some(scattered_word) = app.scattered_words.get(index) {
            let word_line = format!("{}", scattered_word.word);
            let file_line = format!("{}", scattered_word.source_file);
            (word_line.len(), file_line.len())
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    // Path section: calculate width based on wrapped path lines
    let path_str = app.directory.display().to_string();
    // Use a conservative estimate for max_width (content width minus borders/padding)
    let estimated_max_width = 14; // 20 (sidebar cap) - 6 (padding) = 14
    let wrapped_path_lines = wrap_path_smart(&path_str, estimated_max_width);
    let truncated_path_lines = truncate_path_if_needed(wrapped_path_lines, 3, estimated_max_width);
    let path_width = truncated_path_lines.iter().map(|s| s.len()).max().unwrap_or(0);

    // Take the maximum of all sections
    let content_width = scatters_width.max(density_width).max(controls_width).max(word_width).max(file_width).max(path_width);

    // Add padding for borders (2) and internal padding (2) and a bit extra (2)
    (content_width as u16 + 6).min(20) // Cap sidebar width to 20
}

fn widget_block(border_type: BorderType) -> Block<'static> {
    Block::default()
        .border_type(border_type)
        .borders(Borders::all())
}

/// Wraps a path string smartly by preferring to break at path separators
fn wrap_path_smart(path_str: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    // Split by both / and \ to handle cross-platform paths
    let components: Vec<&str> = path_str.split(|c| c == '/' || c == '\\').collect();

    for (i, component) in components.iter().enumerate() {
        // Reconstruct the separator (use the original if possible, or default to /)
        let separator = if i > 0 { "/" } else { "" };
        let piece = format!("{}{}", separator, component);

        // Check if adding this piece would exceed max width
        if !current_line.is_empty() && current_line.len() + piece.len() > max_width {
            // If the piece itself is longer than max_width, we need character-level wrapping
            if piece.len() > max_width {
                // Flush current line if not empty
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }

                // Break the long piece into chunks
                let mut remaining = piece.as_str();
                while remaining.len() > max_width {
                    lines.push(remaining[..max_width].to_string());
                    remaining = &remaining[max_width..];
                }
                current_line = remaining.to_string();
            } else {
                // Start a new line with this piece
                lines.push(current_line.clone());
                current_line = piece;
            }
        } else {
            current_line.push_str(&piece);
        }
    }

    // Add the last line if not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Truncates wrapped path lines if they exceed max_lines by adding ellipsis
fn truncate_path_if_needed(lines: Vec<String>, max_lines: usize, max_width: usize) -> Vec<String> {
    if lines.len() <= max_lines {
        lines
    } else {
        // Special case: if only 1 line available, show the last line
        // rather than just "..."
        if max_lines == 1 {
            vec![lines.last().unwrap().clone()]
        } else {
            // Take the last max_lines lines
            let start_index = lines.len() - max_lines;
            let remaining_lines: Vec<String> = lines[start_index..].to_vec();

            // Combine "..." with the first remaining line
            let first_line = &remaining_lines[0];
            let combined_first = format!("...{}", first_line);

            // If combined line exceeds max_width, truncate it intelligently
            let final_first = if combined_first.len() > max_width {
                if max_width > 3 {
                    // Keep "..." and truncate the directory part
                    let available_for_dir = max_width - 3;
                    format!("...{}", &first_line[..available_for_dir.min(first_line.len())])
                } else {
                    "...".to_string()
                }
            } else {
                combined_first
            };

            // Build result with modified first line and remaining lines
            let mut result = vec![final_first];
            result.extend(remaining_lines[1..].iter().cloned());
            result
        }
    }
}

/// Wraps a single text line at character boundaries if it exceeds max_width
fn wrap_text_line(text: &str, max_width: usize) -> Vec<String> {
    if text.len() <= max_width {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_pos = 0;

    while current_pos < text.len() {
        let remaining = text.len() - current_pos;
        let mut chunk_size = remaining.min(max_width);

        // Ensure we're slicing at a character boundary
        while current_pos + chunk_size < text.len() && !text.is_char_boundary(current_pos + chunk_size) {
            chunk_size -= 1;
        }

        let chunk = &text[current_pos..current_pos + chunk_size];
        lines.push(chunk.to_string());
        current_pos += chunk_size;
    }

    lines
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

    if app.fullscreen_mode {
        // Fullscreen mode: canvas takes entire screen
        render_canvas(f, frame_area, app);
    } else {
        // Normal mode: sidebar + canvas layout
        // Calculate sidebar width based on content
        let sidebar_width = calculate_sidebar_width_for_app(app);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(sidebar_width), Constraint::Min(0)])
            .split(frame_area);

        render_sidebar(f, main_layout[0], app);
        render_canvas(f, main_layout[1], app);
    }
}

fn render_sidebar(f: &mut Frame, area: Rect, app: &App) {
    // Conditionally add info box section if a word is selected
    let has_selection = app.selected_word_index.is_some();

    let available_width = area.width.saturating_sub(4) as usize; // Subtract borders and padding
    let max_width = available_width.max(10); // Minimum width of 10 chars

    // Calculate word box and file box heights dynamically if a word is selected
    let (word_box_height, file_box_height) = if has_selection {
        if let Some(index) = app.selected_word_index {
            if let Some(scattered_word) = app.scattered_words.get(index) {
                let word_text = format!("{}", scattered_word.word);
                let file_text = format!("File: {}", scattered_word.source_file);

                // Wrap both lines separately
                let word_wrapped = wrap_text_line(&word_text, max_width);
                let file_wrapped = wrap_text_line(&file_text, max_width);

                let word_height = (word_wrapped.len() + 2) as u16; // Add 2 for borders
                let file_height = (file_wrapped.len() + 2) as u16; // Add 2 for borders

                (word_height, file_height)
            } else {
                (3, 3) // Default heights
            }
        } else {
            (3, 3) // Default heights
        }
    } else {
        (0, 0) // Not used when no selection
    };

    // Calculate fixed sections height first to ensure they have priority
    let fixed_height = if has_selection {
        4 + 3 + 7 + word_box_height + file_box_height  // Scatters + Density + Controls + Word + File (dynamic)
    } else {
        4 + 3 + 7  // Scatters + Density + Controls
    };

    // Calculate path box height dynamically based on wrapped content
    // But cap it to remaining available space
    let path_str = app.directory.display().to_string();
    let wrapped_path_lines = wrap_path_smart(&path_str, max_width);
    let ideal_path_content_lines = wrapped_path_lines.len().max(1);
    let ideal_path_box_height = (ideal_path_content_lines + 2) as u16; // Add 2 for borders

    // Cap path height to remaining space (with minimum of 3 lines)
    let max_path_height = area.height.saturating_sub(fixed_height).max(3);
    let path_box_height = ideal_path_box_height.min(max_path_height);

    let sections = if has_selection {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),                  // Scatters - fixed
                Constraint::Length(3),                  // Density - fixed
                Constraint::Length(7),                  // Controls - fixed (priority)
                Constraint::Length(word_box_height),    // Word - dynamically sized to wrapped content
                Constraint::Length(file_box_height),    // File - dynamically sized to wrapped content
                Constraint::Length(path_box_height),    // Path - sized to content, capped to available space
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),                  // Scatters - fixed
                Constraint::Length(3),                  // Density - fixed
                Constraint::Length(7),                  // Controls - fixed (priority)
                Constraint::Length(path_box_height),    // Path - sized to content, capped to available space
            ])
            .split(area)
    };

    // Calculate container area dynamically based on actual section positions
    let container_area = if has_selection {
        let last_section = &sections[5]; // Path section is last
        Rect {
            x: area.x,
            y: sections[0].y,
            width: area.width,
            height: (last_section.y + last_section.height).saturating_sub(sections[0].y),
        }
    } else {
        let last_section = &sections[3]; // Path section is last (no Word/File sections)
        Rect {
            x: area.x,
            y: sections[0].y,
            width: area.width,
            height: (last_section.y + last_section.height).saturating_sub(sections[0].y),
        }
    };

    let mut sidebar_container = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style);

    if app.styling.use_background_fill {
        sidebar_container = sidebar_container.style(app.styling.text_style);
    }

    f.render_widget(sidebar_container, container_area);

    let mut scatters_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Text Scatters ", app.styling.text_style)));

    if app.styling.use_background_fill {
        scatters_block = scatters_block.style(app.styling.text_style);
    }

    let count_text = format!("words {} / {}", app.scattered_words.len(), app.word_count);
    let highlighted_text = format!("selected {} / {}", app.highlighted_words.len(), app.scattered_words.len());

    let scatters_text = vec![
        Line::from(Span::styled(count_text, app.styling.text_style)),
        Line::from(Span::styled(highlighted_text, app.styling.text_style)),
    ];

    let scatters = Paragraph::new(scatters_text)
        .block(scatters_block)
        .alignment(Alignment::Left);

    f.render_widget(scatters, sections[0]);

    let mut density_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Density ", app.styling.text_style)));

    if app.styling.use_background_fill {
        density_block = density_block.style(app.styling.text_style);
    }

    let available_width = sections[1].width.saturating_sub(2).max(8) as usize;
    let bar_width = available_width;
    let density_ratio = (app.density - 0.1) / (6.0 - 0.1);
    let filled_width = (density_ratio * bar_width as f32) as usize;
    let empty_width = bar_width - filled_width;

    let filled_bar = "█".repeat(filled_width);
    let empty_bar = " ".repeat(empty_width);

    let density_text = vec![
        Line::from(vec![
            Span::styled(filled_bar, app.styling.density_bar_style),
            Span::styled(empty_bar, app.styling.text_style),
        ]),
    ];

    let density = Paragraph::new(density_text)
        .block(density_block)
        .alignment(Alignment::Left);

    f.render_widget(density, sections[1]);

    let mut controls_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Controls ", app.styling.text_style)));

    if app.styling.use_background_fill {
        controls_block = controls_block.style(app.styling.text_style);
    }

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
        Line::from(vec![
            Span::styled("v", app.styling.text_style),
            Span::styled(" - view", app.styling.text_style),
        ]),
        Line::from(vec![
            Span::styled("q", app.styling.text_style),
            Span::styled(" - quit", app.styling.text_style),
        ]),
    ];

    let controls = Paragraph::new(controls_text)
        .block(controls_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(controls, sections[2]);

    // Render word and file boxes if a word is selected
    if has_selection {
        render_word_box(f, sections[3], app);
        render_file_box(f, sections[4], app);
        render_path_box(f, sections[5], app);
    } else {
        render_path_box(f, sections[3], app);
    }
}

fn render_word_box(f: &mut Frame, area: Rect, app: &App) {
    let mut word_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Current ", app.styling.text_style)));

    if app.styling.use_background_fill {
        word_block = word_block.style(app.styling.text_style);
    }

    // Get the selected word
    let word_text = if let Some(index) = app.selected_word_index {
        if let Some(scattered_word) = app.scattered_words.get(index) {
            format!("{}", scattered_word.word)
        } else {
            "(none)".to_string()
        }
    } else {
        "(none)".to_string()
    };

    // Wrap the text line
    let available_width = area.width.saturating_sub(4) as usize; // Subtract borders and padding
    let max_width = available_width.max(10); // Minimum width of 10 chars

    let word_wrapped = wrap_text_line(&word_text, max_width);

    // Build the display text
    let mut word_text_lines: Vec<Line> = Vec::new();
    for line in word_wrapped {
        word_text_lines.push(Line::from(Span::styled(line, app.styling.text_style)));
    }

    let word_paragraph = Paragraph::new(word_text_lines)
        .block(word_block)
        .alignment(Alignment::Left);

    f.render_widget(word_paragraph, area);
}

fn render_file_box(f: &mut Frame, area: Rect, app: &App) {
    let mut file_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" File ", app.styling.text_style)));

    if app.styling.use_background_fill {
        file_block = file_block.style(app.styling.text_style);
    }

    // Get the source file
    let file_text = if let Some(index) = app.selected_word_index {
        if let Some(scattered_word) = app.scattered_words.get(index) {
            format!("{}", scattered_word.source_file)
        } else {
            "(none)".to_string()
        }
    } else {
        "(none)".to_string()
    };

    // Wrap the text line
    let available_width = area.width.saturating_sub(4) as usize; // Subtract borders and padding
    let max_width = available_width.max(10); // Minimum width of 10 chars

    let file_wrapped = wrap_text_line(&file_text, max_width);

    // Build the display text
    let mut file_text_lines: Vec<Line> = Vec::new();
    for line in file_wrapped {
        file_text_lines.push(Line::from(Span::styled(line, app.styling.text_style)));
    }

    let file_paragraph = Paragraph::new(file_text_lines)
        .block(file_block)
        .alignment(Alignment::Left);

    f.render_widget(file_paragraph, area);
}

fn render_path_box(f: &mut Frame, area: Rect, app: &App) {
    let mut path_block = widget_block(app.styling.border_type)
        .border_style(app.styling.border_style)
        .title_top(Line::from(Span::styled(" Path ", app.styling.text_style)));

    if app.styling.use_background_fill {
        path_block = path_block.style(app.styling.text_style);
    }

    // Get the path and wrap it smartly
    let path_str = app.directory.display().to_string();
    let available_width = area.width.saturating_sub(4) as usize; // Subtract borders and padding
    let max_width = available_width.max(10); // Minimum width of 10 chars

    // Calculate max lines based on available height
    let available_height = area.height.saturating_sub(2) as usize; // Subtract top and bottom borders
    let max_lines = available_height.max(1); // At least 1 line

    // Wrap the path
    let wrapped_lines = wrap_path_smart(&path_str, max_width);

    // Truncate if needed based on dynamic max_lines
    let final_lines = truncate_path_if_needed(wrapped_lines, max_lines, max_width);

    // Convert to Line objects
    let path_text: Vec<Line> = final_lines
        .iter()
        .map(|line| Line::from(Span::styled(line.clone(), app.styling.text_style)))
        .collect();

    let path = Paragraph::new(path_text)
        .block(path_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(path, area);
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
