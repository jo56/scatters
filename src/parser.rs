use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::fs;
use std::path::Path;

pub fn parse_file(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("txt") => parse_txt(path),
        Some("md") | Some("markdown") => parse_markdown(path),
        Some("epub") => parse_epub(path),
        _ => Ok(Vec::new()),
    }
}

fn parse_txt(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(extract_words(&content))
}

fn parse_markdown(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let parser = Parser::new(&content);

    let mut text_content = String::new();
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code_block = true,
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::Text(text) | Event::Code(text) if !in_code_block => {
                text_content.push_str(&text);
                text_content.push(' ');
            }
            _ => {}
        }
    }

    Ok(extract_words(&text_content))
}

//TODO: Update this when epub publishes latest git changes to crates.io
#[allow(deprecated)]
fn parse_epub(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let doc = epub::doc::EpubDoc::new(path)?;
    let mut all_text = String::new();

    for i in 0..doc.get_num_pages() {
        if let Ok(mut doc_copy) = epub::doc::EpubDoc::new(path) {
            doc_copy.set_current_page(i);
            if let Some((content, _)) = doc_copy.get_current_str() {
                all_text.push_str(&strip_html_tags(&content));
                all_text.push(' ');
            }
        }
    }

    Ok(extract_words(&all_text))
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;

    for c in html.chars() {
        match c {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => result.push(c),
            _ => {}
        }
    }

    result
}

fn extract_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_words() {
        let text = "Hello, world! This is a test.";
        let words = extract_words(text);
        assert_eq!(words, vec!["hello", "world", "this", "is", "a", "test"]);
    }
}
