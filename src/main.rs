mod parser;
mod scatters;
mod styling;
mod ui;
mod word_bank;

use clap::Parser as ClapParser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(ClapParser, Debug)]
#[command(name = "text-scatters")]
#[command(about = "A cut-up poetry generator from text files", long_about = None)]
struct Args {
    #[arg(help = "File or directory containing text files to parse (optional - uses last path if omitted)")]
    path: Option<PathBuf>,

    #[arg(
        short = 't',
        long = "theme",
        value_name = "THEME",
        help = "Color theme to use",
        default_value = "monochrome"
    )]
    theme: String,
}

/// Get the config directory for text-scatters
fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("text-scatters");

    // Create the config directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

/// Save the last used path to config file
fn save_last_path(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    let config_file = config_dir.join("last_path.txt");
    fs::write(config_file, path.to_string_lossy().as_bytes())?;
    Ok(())
}

/// Load the last used path from config file
fn load_last_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    let config_file = config_dir.join("last_path.txt");

    if !config_file.exists() {
        return Err("No previous path saved. Please provide a file or directory path.".into());
    }

    let path_str = fs::read_to_string(config_file)?;
    let path = PathBuf::from(path_str.trim());

    if !path.exists() {
        return Err(format!("Previously saved path '{}' no longer exists", path.display()).into());
    }

    Ok(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine which path to use
    let input_path = match args.path {
        Some(p) => p,
        None => {
            match load_last_path() {
                Ok(path) => {
                    println!("Using last path: {}", path.display());
                    path
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };

    if !input_path.exists() {
        eprintln!(
            "Error: '{}' does not exist",
            input_path.display()
        );
        std::process::exit(1);
    }

    let mut word_bank = word_bank::WordBank::new();
    let mut file_count = 0;
    let display_path: PathBuf;

    // Handle both single file and directory
    if input_path.is_file() {
        // Process single file
        let extension = input_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        match extension.as_deref() {
            Some("txt") | Some("md") | Some("markdown") | Some("epub") => {
                println!("Parsing file: {}", input_path.display());
                match parser::parse_file(&input_path) {
                    Ok(words) => {
                        let file_name = input_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        word_bank.add_words(words, file_name);
                        file_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to parse {}: {}", input_path.display(), e);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("Error: '{}' is not a supported file type (txt, md, markdown, epub)", input_path.display());
                std::process::exit(1);
            }
        }
        display_path = input_path.clone();
    } else if input_path.is_dir() {
        // Process directory
        println!("Scanning directory: {}", input_path.display());

        for entry in fs::read_dir(&input_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let extension = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase());

                match extension.as_deref() {
                    Some("txt") | Some("md") | Some("markdown") | Some("epub") => {
                        println!("Parsing: {}", path.display());
                        match parser::parse_file(&path) {
                            Ok(words) => {
                                // Compute relative path from base directory
                                let relative_path = path
                                    .strip_prefix(&input_path)
                                    .unwrap_or(&path)
                                    .to_string_lossy()
                                    .replace('\\', "/"); // Normalize path separators
                                word_bank.add_words(words, relative_path);
                                file_count += 1;
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        display_path = input_path.clone();
    } else {
        eprintln!("Error: '{}' is neither a file nor a directory", input_path.display());
        std::process::exit(1);
    }

    println!("Parsed {} files", file_count);
    println!("Collected {} unique words", word_bank.word_count());

    if word_bank.word_count() == 0 {
        eprintln!("Error: No words found");
        std::process::exit(1);
    }

    // Save the successfully used path for next time
    if let Err(e) = save_last_path(&display_path) {
        eprintln!("Warning: Could not save path for next time: {}", e);
    }

    println!("Starting TUI...");
    std::thread::sleep(std::time::Duration::from_secs(1));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let words = word_bank.get_words();
    let word_count = words.len();
    let generator = scatters::ScattersGenerator::new(words);

    let size = terminal.size()?;

    // Initialize styling based on theme first (needed for canvas calculation)
    let styling = match styling::AppStyling::from_theme(&args.theme) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Create temporary app to calculate sidebar width
    let temp_app = ui::App::new(Vec::new(), word_count, styling.clone(), display_path.clone());
    let sidebar_width = ui::calculate_sidebar_width_for_app(&temp_app);

    // Calculate actual canvas area based on dynamic sidebar
    let canvas_width = size.width.saturating_sub(sidebar_width).saturating_sub(2);
    let canvas_height = size.height.saturating_sub(2);
    let scattered_words = generator.generate_with_density(canvas_width, canvas_height, 1.0);

    let mut app = ui::App::new(scattered_words, word_count, styling, display_path);

    let res = run_app(&mut terminal, &mut app, &generator);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut ui::App,
    generator: &scatters::ScattersGenerator,
) -> io::Result<()> {
    // Draw initial UI
    terminal.draw(|f| ui::ui(f, app))?;

    loop {
        // Read event (blocking)
        let event = event::read()?;

        match event {
            Event::Key(key) => {
                // Only process Press events - ignore Repeat and Release
                // This ensures each physical keypress is counted exactly once
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                // Handle Ctrl+C
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                // Process the key event
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        let size = terminal.size()?;
                        let canvas_width = if app.fullscreen_mode {
                            size.width.saturating_sub(2)
                        } else {
                            let sidebar_width = ui::calculate_sidebar_width_for_app(app);
                            size.width.saturating_sub(sidebar_width).saturating_sub(2)
                        };
                        let canvas_height = size.height.saturating_sub(2);
                        let new_scattered = generator.generate_with_density(canvas_width, canvas_height, app.density);
                        app.update_words(new_scattered);
                    }
                    KeyCode::Right | KeyCode::Tab | KeyCode::Char('n') => {
                        app.select_next_word();
                    }
                    KeyCode::Left | KeyCode::BackTab | KeyCode::Char('p') => {
                        app.select_prev_word();
                    }
                    KeyCode::Up => {
                        let size = terminal.size()?;
                        let bar_width = ui::get_density_bar_width(size.width);
                        app.increase_density(bar_width);
                    }
                    KeyCode::Down => {
                        let size = terminal.size()?;
                        let bar_width = ui::get_density_bar_width(size.width);
                        app.decrease_density(bar_width);
                    }
                    KeyCode::Char(' ') => {
                        app.toggle_current_highlight();
                    }
                    KeyCode::Char('v') | KeyCode::Char('V') => {
                        app.fullscreen_mode = !app.fullscreen_mode;
                    }
                    _ => {}
                }
            }
            Event::Resize(_, _) => {
                // Handle resize events so UI adapts to new terminal size
            }
            _ => {}
        }

        // Render the updated UI after processing event
        terminal.draw(|f| ui::ui(f, app))?;
    }
}
