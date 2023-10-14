use std::collections::HashSet;

use radix_trie::Trie;

pub type Category = String;
type Pattern = String;

pub struct Classifier {
    pub pattern_category_map: Trie<Pattern, Category>,
    fallback_categories: HashSet<Category>,
}

impl Classifier {
    pub fn get_category(&self, text: &str) -> Option<Category> {
        let mut result: Option<Category> = None;
        let text_patterns: Vec<&str> = text.split(&['*', ',']).collect();

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

    pub(crate) fn new() -> Classifier {
        Classifier {
            pattern_category_map: Trie::new(),
            fallback_categories: HashSet::new(),
        }
    }

    pub fn from(
        pattern_category_map: Trie<Pattern, Category>,
        fallback_categories: HashSet<Category>,
    ) -> Classifier {
        Classifier {
            pattern_category_map,
            fallback_categories,
        }
    }
}
