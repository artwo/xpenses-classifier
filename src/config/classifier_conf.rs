use crate::service::classifier::Classifier;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClassifierConfig {
    fallback_categories: HashSet<String>,
    categories: Vec<ClassifierConfigItem>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClassifierConfigItem {
    name: String,
    patterns: HashSet<String>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileProcessorConfigItem {
    pub name: String,
    pub file_name_pattern: String,
    pub category_segment_idx: Box<[usize]>,
    pub expense_segment_idx: Box<[usize]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub expenses_path: String,
    file_processor_config: Vec<FileProcessorConfigItem>,
    classifier_config: ClassifierConfig,
    pub uncategorized_enabled: bool,
}

impl Config {
    fn read_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let json_config = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&json_config)?;
        Ok(config)
    }

    pub fn new(file_path: &str) -> Result<Config, Box<dyn Error>> {
        Config::read_config(file_path)
    }

    pub fn generate_classifier(&self) -> Classifier {
        let mut category_trie: HashMap<String, String> = HashMap::new();

        for c in self.classifier_config.categories.iter() {
            for p in c.patterns.clone() {
                category_trie.insert(p.to_lowercase(), c.name.clone());
            }
        }

        Classifier::from(
            category_trie,
            self.classifier_config.fallback_categories.clone(),
        )
    }

    pub fn get_file_processor_config(&self) -> &[FileProcessorConfigItem] {
        return self.file_processor_config.iter().as_slice();
    }
}
