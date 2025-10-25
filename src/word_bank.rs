use std::collections::HashSet;

pub struct WordBank {
    words: HashSet<String>,
}

impl WordBank {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    pub fn add_words(&mut self, words: Vec<String>) {
        for word in words {
            if !is_stop_word(&word) && word.len() >= 3 {
                self.words.insert(word);
            }
        }
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words.iter().cloned().collect()
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

fn is_stop_word(word: &str) -> bool {
    const STOP_WORDS: &[&str] = &[
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on",
        "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we",
        "say", "her", "she", "or", "an", "will", "my", "one", "all", "would", "there", "their",
        "what", "so", "up", "out", "if", "about", "who", "get", "which", "go", "me", "when",
        "make", "can", "like", "time", "no", "just", "him", "know", "take", "people", "into",
        "year", "your", "good", "some", "could", "them", "see", "other", "than", "then", "now",
        "look", "only", "come", "its", "over", "think", "also", "back", "after", "use", "two",
        "how", "our", "work", "first", "well", "way", "even", "new", "want", "because", "any",
        "these", "give", "day", "most", "us", "is", "was", "are", "been", "has", "had", "were",
        "said", "did", "having", "may", "should", "am", "being", "does",
    ];

    STOP_WORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_filtering() {
        let mut bank = WordBank::new();
        bank.add_words(vec![
            "the".to_string(),
            "wonderful".to_string(),
            "and".to_string(),
            "beautiful".to_string(),
        ]);

        let words = bank.get_words();
        assert!(words.contains(&"wonderful".to_string()));
        assert!(words.contains(&"beautiful".to_string()));
        assert!(!words.contains(&"the".to_string()));
        assert!(!words.contains(&"and".to_string()));
    }

    #[test]
    fn test_minimum_word_length() {
        let mut bank = WordBank::new();
        bank.add_words(vec!["hi".to_string(), "hello".to_string()]);

        let words = bank.get_words();
        assert_eq!(words.len(), 1);
        assert!(words.contains(&"hello".to_string()));
    }
}
