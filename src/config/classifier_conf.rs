use crate::service::classifier::Classifier;
use radix_trie::Trie;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

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

pub fn process_categories_config(file_path: &str) -> Result<Classifier, Box<dyn Error>> {
    let json_config = fs::read_to_string(file_path)?;
    let cat_config: CategoryConfigFile = serde_json::from_str(&json_config)?;
    let mut category_trie: Trie<String, String> = Trie::new();
    for c in cat_config.categories {
        for p in c.patterns {
            category_trie.insert(p, c.name.clone());
        }
    }

    Ok(Classifier::from(
        category_trie,
        cat_config.fallback_categories,
    ))
}
