extern crate encoding_rs;
extern crate encoding_rs_io;

mod config;
mod service;
mod util;

use crate::service::file_classifier::FileProcessor;
use service::classified_expenses_service::ClassifiedExpensesService;
use std::collections::HashMap;
use std::process;

const CONFIG_PATH: &str = "./conf.json";

fn main() {
    let classifier_result = config::classifier_conf::process_categories_config(CONFIG_PATH);
    let classifier = classifier_result.unwrap_or_else(|err| {
        eprintln!("Unable to read the config file {CONFIG_PATH}, error: {err}");
        process::exit(1);
    });
    let mut expenses_service = ClassifiedExpensesService {
        expenses_map: &mut HashMap::new(),
    };
    let mut file_processor = FileProcessor {
        classifier: &classifier,
        expenses_service: &mut expenses_service,
        category_segment_idx: Box::new([3, 4]),
        expense_segment_idx: Box::new([15, 16]),
    };

    let transactions_file = "./Transactions_701_311319800_20230416_182556.csv";
    file_processor
        .process_file(transactions_file)
        .unwrap_or_else(|err| {
            eprintln!("Unable to process file with name {transactions_file}, error: {err}");
            process::exit(1);
        });
}
