use crate::service::classifier::Classifier;
use crate::util::file_util::open_encoded_file;

use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

pub struct FileClassifier {
    classifier: Classifier,
    pub classified_expenses: HashMap<String, Vec<String>>,
}

impl FileClassifier {
    pub fn process_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let rdr = open_encoded_file(file_path)?;
        for line_result in rdr.lines() {
            let line = line_result?;
            let line_segments: &Vec<&str> = &line.split(';').collect();

            if let Some(cat) = self.find_category(line_segments) {
                self.add_to_classified(cat, FileClassifier::extract_expense_value(line_segments))
            }
        }
        Ok(())
    }

    fn find_category(&self, line_segments: &[&str]) -> Option<String> {
        let transaction_segment_idx = [3, 4];
        for i in transaction_segment_idx {
            let opt_cat = line_segments
                .get(i)
                .and_then(|text| self.classifier.classify(text));
            if opt_cat.is_some() {
                return opt_cat;
            }
        }
        None
    }

    fn add_to_classified(&mut self, category: String, value: Option<&str>) {
        let new_value: String = match value {
            Some(v) => String::from(v),
            None => return,
        };

        match self.classified_expenses.get_mut(category.as_str()) {
            Some(values) => values.push(new_value),
            None => {
                self.classified_expenses
                    .insert(category.clone(), Vec::from([new_value]));
            }
        }
    }

    fn extract_expense_value<'a>(segments: &[&'a str]) -> Option<&'a str> {
        let expense_idx = [15, 16];
        for i in expense_idx {
            // let value = segments.get(i)?.parse::<f64>();
            // if value.is_ok() {
            //     return Option::from(segments[i]);
            // }
            if !segments.get(i)?.is_empty() {
                return Some(segments[i]);
            }
        }
        None
    }

    pub fn new() -> FileClassifier {
        FileClassifier {
            classifier: Classifier::new(),
            classified_expenses: HashMap::new(),
        }
    }
}
