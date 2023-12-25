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

    let expenses_path = config.expenses_path.clone();
    let classifier = config.generate_classifier();
    let file_processor_config_list = config.get_file_processor_config();

    let mut file_processors: Vec<FileProcessor> = Vec::new();
    for c in file_processor_config_list {
        let pattern = c.file_name_pattern.clone();
        let file_name_pattern = format!("{expenses_path}/{pattern}");
        file_processors.push(FileProcessor {
            classifier: &classifier,
            name: c.name.clone(),
            file_name_pattern: file_name_pattern.clone(),
            category_segment_idx: c.category_segment_idx.clone(),
            expense_segment_idx: c.expense_segment_idx.clone(),
        });
    }

    let mut expenses_service = ClassifiedExpensesService {
        expenses_map: &mut HashMap::new(),
        file_processors: &file_processors,
    };

    expenses_service.process_files().unwrap_or_else(|err| {
        eprintln!("Unable to process files, error: {err}");
        process::exit(1);
    });

    println!("{:?}", classifier.pattern_category_map);
    println!("Successfully processed expense files");
    for el in expenses_service.expenses_map.iter() {
        println!("{}: {:?}", el.0, el.1);
    }
}
