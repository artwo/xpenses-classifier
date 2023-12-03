use std::collections::{HashMap, HashSet};

pub type Category = String;
type CategoryPattern = String;

pub struct Classifier {
    pub pattern_category_map: HashMap<CategoryPattern, Category>,
    fallback_categories: HashSet<Category>,
    entry_separators: Box<[char]>,
}

impl Classifier {
    pub fn get_category(&self, text: &str) -> Option<Category> {
        let mut result: Option<Category> = None;
        let lowercase_text = text.to_lowercase();
        let text_patterns: Vec<&str> = lowercase_text
            .split(self.entry_separators.iter().as_slice())
            .collect();

        for p in text_patterns {
            if let Some(cat) = self.pattern_category_map.get(p).cloned() {
                if result.is_none() && self.fallback_categories.contains(cat.as_str()) {
                    result = Some(cat);
                } else {
                    result = Some(cat);
                    break;
                }
            }
        }
        result
    }

    pub fn from(
        pattern_category_map: HashMap<CategoryPattern, Category>,
        fallback_categories: HashSet<Category>,
    ) -> Classifier {
        Classifier {
            pattern_category_map,
            fallback_categories,
            entry_separators: Box::new(['*', ',', ' ']),
        }
    }
}
