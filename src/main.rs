extern crate encoding_rs;
extern crate encoding_rs_io;

mod config;
mod service;
mod util;

use service::file_classifier;

fn main() {
    let classifier = config::classifier_conf::process_categories_config();
    let mut file_classifier = file_classifier::FileClassifier::new();
    file_classifier
        .process_file("./Transactions_701_311319800_20230416_182556.csv")
        .expect("TODO: panic message");
    println!("{:?}", file_classifier.classified_expenses)
}
