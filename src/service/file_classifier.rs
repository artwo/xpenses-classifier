use crate::service::classified_expenses_service::ClassifiedExpensesService;
use crate::service::classifier::Classifier;
use crate::util::file_util::open_encoded_file;

use std::error::Error;
use std::io::BufRead;

pub struct FileProcessor<'a> {
    pub classifier: &'a Classifier,
    pub expenses_service: &'a mut ClassifiedExpensesService<'a>,
    pub category_segment_idx: Box<[usize]>,
    pub expense_segment_idx: Box<[usize]>,
}

impl<'a> FileProcessor<'a> {
    pub fn process_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let rdr = open_encoded_file(file_path)?;
        for line_result in rdr.lines() {
            let line = line_result?;
            let line_segments: &Vec<&str> = &line.split(';').collect();

            if let Some(cat) = self.find_category(line_segments) {
                self.expenses_service
                    .add_to_classified(cat, self.extract_expense_value(line_segments))
            }
        }
        Ok(())
    }

    fn find_category(&self, segments: &[&str]) -> Option<String> {
        for &i in self.category_segment_idx.iter() {
            let opt_cat = segments
                .get(i)
                .and_then(|text| self.classifier.get_category(text));
            if opt_cat.is_some() {
                return opt_cat;
            }
        }
        None
    }

    fn extract_expense_value<'b>(&self, segments: &[&'b str]) -> Option<&'b str> {
        for &i in self.expense_segment_idx.iter() {
            if !segments.get(i)?.is_empty() {
                return Some(segments[i]);
            }
        }
        None
    }
}
