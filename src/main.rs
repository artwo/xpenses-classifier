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
    let config = config::classifier_conf::Config::new(CONFIG_PATH).unwrap_or_else(|err| {
        eprintln!("Unable to read the config file {CONFIG_PATH}, error: {err}");
        process::exit(1);
    });

    let classifier = config.generate_classifier();
    let file_processor_config_list = config.get_file_processor_config();

    let mut file_processors: Vec<FileProcessor> = Vec::new();
    for c in file_processor_config_list {
        file_processors.insert(
            0,
            FileProcessor {
                classifier: &classifier,
                category_segment_idx: c.expense_segment_idx.clone(),
                expense_segment_idx: c.expense_segment_idx.clone(),
            },
        );
    }

    let mut expenses_service = ClassifiedExpensesService {
        expenses_map: &mut HashMap::new(),
        file_processors: &file_processors,
    };

    let transactions_file = "./Transactions_701_311319800_20230416_182556.csv";
    expenses_service
        .process_file(transactions_file)
        .unwrap_or_else(|err| {
            eprintln!("Unable to process file with name {transactions_file}, error: {err}");
            process::exit(1);
        });
}
