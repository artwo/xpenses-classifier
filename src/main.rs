extern crate encoding_rs;
extern crate encoding_rs_io;

mod service;
mod util;

use std::collections::HashSet;

use serde::Deserialize;
use service::file_classifier;

#[derive(Deserialize, Debug)]
struct CategoryRaw {
    name: String,
    patterns: HashSet<String>,
    priority: i16,
}

fn main() {
    let json_config = r#"[
        { "name": "Supermarket", "patterns": ["Rewe", "Edeka", "Lidl"], "priority": 20 }
    ]"#;
    let categories: Vec<CategoryRaw> = serde_json::from_str(json_config).unwrap();
    // let classifier = Classifier{categories};
    // println!("{:?}", categories);

    let mut file_classifier = file_classifier::FileClassifier::new();
    file_classifier
        .process_file("./Transactions_701_311319800_20230416_182556.csv")
        .expect("TODO: panic message");
    println!("{:?}", file_classifier.classified_expenses)
}
