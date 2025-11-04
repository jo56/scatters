use ratatui::{
    style::{Color, Style},
    widgets::BorderType,
};

#[derive(Clone)]
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
            "lightmono" => Ok(Self::lightmono_theme()),
            "redmono" => Ok(Self::redmono_theme()),
            "softmono" => Ok(Self::softmono_theme()),
            "graymono" => Ok(Self::graymono_theme()),
            "nord" => Ok(Self::nord_theme()),
            "nord-bg" => Ok(Self::nord_bg_theme()),
            "gruvbox" => Ok(Self::gruvbox_theme()),
            "rosepine" => Ok(Self::rosepine_theme()),
            "goldgreen-light" => Ok(Self::goldgreen_light_theme()),
            "goldgreen-dark" => Ok(Self::goldgreen_dark_theme()),
            _ => Err(format!(
                "Invalid theme '{}'. Valid themes: monochrome, lightmono, redmono, softmono, graymono, nord, nord-bg, gruvbox, rosepine, goldgreen-light, goldgreen-dark",
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

    fn nord_bg_theme() -> Self {
        const NORD_BG: &str = "#2e3440"; // Nord dark background (Polar Night)
        const NORD_FG: &str = "#e5e9f0"; // Nord light foreground (Snow Storm)
        const NORD_FROST_BLUE: &str = "#88c0d0"; // Nord Frost bright blue
        const NORD_FROST_DARK: &str = "#5e81ac"; // Nord Frost dark blue
        const NORD_FROST_CYAN: &str = "#8fbcbb"; // Nord Frost cyan

        Self {
            border_style: Self::hex_style(NORD_FROST_BLUE).bg(Self::hex_color(NORD_BG)),  // Bright blue for sidebar
            highlighted_border_style: Self::hex_style(NORD_FROST_DARK).bg(Self::hex_color(NORD_BG)),  // Dark blue for canvas
            text_style: Self::hex_style(NORD_FG).bg(Self::hex_color(NORD_BG)),
            selected_text_style: Self::hex_style(NORD_BG)
                .bg(Self::hex_color(NORD_FROST_BLUE)),  // Dark on bright blue
            current_selected_style: Self::hex_style(NORD_BG)
                .bg(Self::hex_color(NORD_FROST_CYAN)),  // Dark on cyan for current selection
            density_bar_style: Self::hex_style(NORD_FROST_BLUE).bg(Self::hex_color(NORD_BG)),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // No background fill for nord theme
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

   
    fn redmono_theme() -> Self {
        const BLACK: &str = "#3c3836";
        const RED: &str = "#9d0006"; 
        
        Self {
            border_style: Self::hex_style(RED),
            highlighted_border_style: Self::hex_style(RED),
            text_style: Self::hex_style(BLACK),
            selected_text_style: Self::hex_style(RED),  
            current_selected_style: Self::hex_style(BLACK)
                .bg(Self::hex_color(BLACK)),  
            density_bar_style: Self::hex_style(BLACK),  
            border_type: BorderType::Plain,
            use_background_fill: false,  
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
            current_selected_style: Style::default().fg(Color::Black),  // Black text, no background (default state)
            density_bar_style: Style::default().fg(Color::Black).bg(Color::White),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for seamless white background
        }
    }

    fn softmono_theme() -> Self {
        const SOFT_WHITE: &str = "#FCF6F8";

        Self {
            border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            highlighted_border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            text_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            selected_text_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = solid black boxes (previously visited + toggled current)
            current_selected_style: Style::default().fg(Color::Black),  // Black text, no background (default state)
            density_bar_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for seamless soft white background
        }
    }

    fn graymono_theme() -> Self {
        const SOFT_WHITE: &str = "#FCF6F8";
        const SOFT_GRAY: &str = "#8B8B8B";

        Self {
            border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            highlighted_border_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            text_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),
            selected_text_style: Self::hex_style(SOFT_GRAY),  // Soft gray text, no background (previously visited + toggled current)
            current_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Black),  // Black on black = darker highlight (default state)
            density_bar_style: Style::default().fg(Color::Black).bg(Self::hex_color(SOFT_WHITE)),  // Same as border
            border_type: BorderType::Plain,
            use_background_fill: true,  // Enable background fill for seamless soft white background
        }
    }

    fn lightmono_theme() -> Self {
        const MONO_COLOR: &str = "#3c3836"; 

        Self {
            border_style: Self::hex_style(MONO_COLOR),
            highlighted_border_style: Self::hex_style(MONO_COLOR),
            text_style: Self::hex_style(MONO_COLOR),
            selected_text_style: Self::hex_style(MONO_COLOR).bg(Self::hex_color(MONO_COLOR)),  
            current_selected_style: Self::hex_style(MONO_COLOR),
            density_bar_style: Self::hex_style(MONO_COLOR),  
            border_type: BorderType::Plain,
            use_background_fill: false,  
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

    fn goldgreen_dark_theme() -> Self {
        const GOLD: &str = "#C78A14"; 
        const GREEN: &str = "#0F4620";
        
        Self {
            border_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),
            highlighted_border_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),
            text_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),
            selected_text_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),  
            current_selected_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),  
            density_bar_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),  
            border_type: BorderType::Plain,
            use_background_fill: true,  
        }
    }

    fn goldgreen_light_theme() -> Self {
        const GOLD: &str = "#C78A14"; 
        const GREEN: &str = "#0F4620";
        
        Self {
            border_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),
            highlighted_border_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),
            text_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),
            selected_text_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),  
            current_selected_style: Self::hex_style(GOLD).bg(Self::hex_color(GREEN)),  
            density_bar_style: Self::hex_style(GREEN).bg(Self::hex_color(GOLD)),  
            border_type: BorderType::Plain,
            use_background_fill: true,  
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
