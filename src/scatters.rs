use rand::seq::SliceRandom;
use rand::Rng;

pub struct ScatteredWord {
    pub word: String,
    pub x: u16,
    pub y: u16,
}

pub struct ScattersGenerator {
    word_pool: Vec<String>,
}

impl ScattersGenerator {
    pub fn new(words: Vec<String>) -> Self {
        Self { word_pool: words }
    }

    pub fn generate_with_density(&self, width: u16, height: u16, density: f32) -> Vec<ScatteredWord> {
        let mut rng = rand::thread_rng();

        // Calculate word count based on canvas area
        // Use roughly 1 word per 40 character cells (width * height / 40)
        // Add randomization so each reroll uses a different count
        let canvas_area = (width as usize).saturating_mul(height as usize);
        let base_count = ((canvas_area as f32 / 40.0) * density) as usize;
        let base_count = base_count.max(2);

        let min_count = (base_count * 70 / 100).max(2);
        let max_count = (base_count * 130 / 100).min(self.word_pool.len());

        let count = if min_count < max_count {
            rng.gen_range(min_count..=max_count)
        } else {
            min_count.min(self.word_pool.len())
        };

        let mut selected_words: Vec<String> = self
            .word_pool
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect();

        selected_words.shuffle(&mut rng);

        let usable_width = width;
        let usable_height = height;

        let mut scattered_words = Vec::new();
        let mut occupied_positions = Vec::new();

        for word in selected_words.iter() {
            let mut attempts = 0;
            let max_attempts = 100;
            let mut placed = false;

            while attempts < max_attempts {
                let max_x = usable_width.saturating_sub(word.len() as u16);

                if max_x == 0 {
                    break;
                }

                let x = rng.gen_range(0..=max_x);
                let y = rng.gen_range(0..usable_height);

                if !is_overlapping_tight(x, y, word, &occupied_positions) {
                    occupied_positions.push((x, y, word.len() as u16));
                    scattered_words.push(ScatteredWord {
                        word: word.clone(),
                        x,
                        y,
                    });
                    placed = true;
                    break;
                }

                attempts += 1;
            }

            // Fallback placement if collision avoidance failed
            if !placed && usable_width >= word.len() as u16 {
                let max_x = usable_width.saturating_sub(word.len() as u16);
                let x = if max_x > 0 { rng.gen_range(0..=max_x) } else { 0 };
                let y = rng.gen_range(0..usable_height);
                scattered_words.push(ScatteredWord {
                    word: word.clone(),
                    x,
                    y,
                });
            }
        }

        scattered_words
    }

}

fn is_overlapping_tight(x: u16, y: u16, word: &str, occupied: &[(u16, u16, u16)]) -> bool {
    let word_len = word.len() as u16;
    let min_gap = 2u16;

    for &(ox, oy, olen) in occupied {
        if y == oy {
            let x_end = x + word_len;
            let ox_end = ox + olen;

            if x_end + min_gap > ox && x < ox_end + min_gap {
                return true;
            }
        }

        if (y as i16 - oy as i16).abs() <= 0 {
            let x_end = x + word_len;
            let ox_end = ox + olen;
            if x_end + min_gap > ox && x < ox_end + min_gap {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scatters_generation() {
        let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let generator = ScattersGenerator::new(words);
        let scattered = generator.generate_with_density(80, 24, 1.0);

        assert!(!scattered.is_empty());
        assert!(scattered.len() <= 3);
    }

}
