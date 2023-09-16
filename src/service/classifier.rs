use std::collections::HashSet;

use radix_trie::Trie;

pub struct Classifier {
    pattern_category_map: Trie<String, String>,
    fallback_categories: HashSet<String>,
}

impl Classifier {
    pub fn classify(&self, text: &str) -> Option<String> {
        let mut result: Option<String> = None;
        let words: Vec<&str> = text.split(&['*', ',']).collect();

        for w in words {
            if let Some(cat) = self.pattern_category_map.get(w).cloned() {
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
}
