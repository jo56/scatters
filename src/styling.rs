use ratatui::{
    style::{Color, Style},
    widgets::BorderType,
};

pub struct AppStyling {
    pub border_style: Style,
    pub highlighted_border_style: Style,
    pub text_style: Style,
    pub selected_text_style: Style,
    pub current_selected_style: Style,  // Style for currently selected word (brighter)
    pub density_bar_style: Style,  // Style for filled portion of density bar
    pub border_type: BorderType,
    pub use_background_fill: bool,  // Whether to fill backgrounds (for monochrome theme)
}

impl AppStyling {
    pub fn from_theme(theme: &str) -> Result<Self, String> {
        match theme.to_lowercase().as_str() {
            "monochrome" => Ok(Self::monochrome_theme()),
            "softmono" => Ok(Self::softmono_theme()),
            "nord" => Ok(Self::nord_theme()),
            "gruvbox" => Ok(Self::gruvbox_theme()),
            "rosepine" => Ok(Self::rosepine_theme()),
            _ => Err(format!(
                "Invalid theme '{}'. Valid themes: monochrome, softmono, nord, gruvbox, rosepine",
                theme
            )),
        }
    }

    // Nord theme
    fn nord_theme() -> Self {
        const NORD_BG: &str = "#2e3440"; // Nord dark background (Polar Night)
        const NORD_FG: &str = "#e5e9f0"; // Nord light foreground (Snow Storm)
        const NORD_FROST_BLUE: &str = "#88c0d0"; // Nord Frost bright blue
        const NORD_FROST_DARK: &str = "#5e81ac"; // Nord Frost dark blue
        const NORD_FROST_CYAN: &str = "#8fbcbb"; // Nord Frost cyan

        Self {
            border_style: Self::hex_style(NORD_FROST_BLUE),  // Bright blue for sidebar
            highlighted_border_style: Self::hex_style(NORD_FROST_DARK),  // Dark blue for canvas
            text_style: Self::hex_style(NORD_FG),
            selected_text_style: Self::hex_style(NORD_BG)
                .bg(Self::hex_color(NORD_FROST_BLUE)),  // Dark on bright blue
            current_selected_style: Self::hex_style(NORD_BG)
                .bg(Self::hex_color(NORD_FROST_CYAN)),  // Dark on cyan for current selection
            density_bar_style: Self::hex_style(NORD_FROST_BLUE),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: false,  // No background fill for nord theme
        }
    }

    // Gruvbox theme
    fn gruvbox_theme() -> Self {
        const GRUVBOX_BG: &str = "#282828"; // Gruvbox dark background
        const GRUVBOX_FG: &str = "#ebdbb2"; // Gruvbox light foreground
        const GRUVBOX_ORANGE: &str = "#fe8019"; // Gruvbox orange accent
        const GRUVBOX_YELLOW: &str = "#fabd2f"; // Gruvbox yellow accent
        const GRUVBOX_DARK: &str = "#1d2021"; // Gruvbox darker variant

        Self {
            border_style: Self::hex_style(GRUVBOX_FG).bg(Self::hex_color(GRUVBOX_BG)),
            highlighted_border_style: Self::hex_style(GRUVBOX_ORANGE).bg(Self::hex_color(GRUVBOX_BG)),
            text_style: Self::hex_style(GRUVBOX_FG).bg(Self::hex_color(GRUVBOX_BG)),
            selected_text_style: Self::hex_style(GRUVBOX_DARK)
                .bg(Self::hex_color(GRUVBOX_FG)),  // Dark text on light background (inverted)
            current_selected_style: Self::hex_style(GRUVBOX_DARK)
                .bg(Self::hex_color(GRUVBOX_YELLOW)),  // Dark text on yellow background for current selection
            density_bar_style: Self::hex_style(GRUVBOX_FG).bg(Self::hex_color(GRUVBOX_BG)),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for gruvbox theme
        }
    }

    // Monochrome theme
    fn monochrome_theme() -> Self {
        Self {
            border_style: Style::default().fg(Color::Black).bg(Color::White),
            highlighted_border_style: Style::default().fg(Color::Black).bg(Color::White),
            text_style: Style::default().fg(Color::Black).bg(Color::White),
            selected_text_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = solid black boxes
            current_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = solid black boxes
            density_bar_style: Style::default().fg(Color::Black).bg(Color::White),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for seamless white background
        }
    }

    // Soft Monochrome theme - Like monochrome but with soft pink-white (#FCF6F8)
    fn softmono_theme() -> Self {
        const SOFT_WHITE: &str = "#FCF6F8"; // Soft pink-white background

        Self {
            border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            highlighted_border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            text_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            selected_text_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = solid black boxes
            current_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = solid black boxes
            density_bar_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for seamless soft white background
        }
    }

    // Rose Pine theme 
    fn rosepine_theme() -> Self {
        const ROSE_BG: &str = "#191724"; // Rose Pine deep purple-black background
        const ROSE_FG: &str = "#e0def4"; // Rose Pine light lavender foreground
        const ROSE_IRIS: &str = "#c4a7e7"; // Rose Pine iris (soft purple)
        const ROSE_LOVE: &str = "#eb6f92"; // Rose Pine love (dusty rose pink)
        const ROSE_GOLD: &str = "#f6c177"; // Rose Pine gold (warm gold)
        const ROSE_FOAM: &str = "#907aa9"; // b4637a or 907aa9

        Self {
            border_style: Self::hex_style(ROSE_IRIS).bg(Self::hex_color(ROSE_BG)),
            highlighted_border_style: Self::hex_style(ROSE_FOAM).bg(Self::hex_color(ROSE_BG)),  // Muted teal for canvas
            text_style: Self::hex_style(ROSE_FG).bg(Self::hex_color(ROSE_BG)),
            selected_text_style: Self::hex_style(ROSE_BG)
                .bg(Self::hex_color(ROSE_LOVE)),  // Dark on rose pink
            current_selected_style: Self::hex_style(ROSE_BG)
                .bg(Self::hex_color(ROSE_GOLD)),  // Dark on warm gold for current selection
            density_bar_style: Self::hex_style(ROSE_LOVE).bg(Self::hex_color(ROSE_BG)),  // Rose pink like highlighted text
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for rose pine theme
        }
    }

    // Helper to convert hex string to Color
    fn hex_color(hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color::Rgb(r, g, b)
    }

    // Helper to create Style with hex color
    fn hex_style(hex: &str) -> Style {
        Style::default().fg(Self::hex_color(hex))
    }
}
