use crate::service::classifier::Classifier;
use radix_trie::Trie;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CategoryConfigFile {
    fallback_categories: HashSet<String>,
    categories: Vec<CategoryConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CategoryConfig {
    name: String,
    patterns: HashSet<String>,
}

pub fn process_categories_config() -> Classifier {
    let json_config = r#"{
        "fallbackCategories": ["Paypal", "Amazon"],
        "categories": [
            { "name": "Supermarket", "patterns": ["Rewe", "Edeka", "Lidl"] }
        ]
    }"#;
    let cat_config: CategoryConfigFile = serde_json::from_str(json_config).unwrap();
    let mut category_trie: Trie<String, String> = Trie::new();
    for c in cat_config.categories {
        for p in c.patterns {
            category_trie.insert(p, c.name.clone());
        }
    }

    Classifier::from(category_trie, cat_config.fallback_categories)
}
