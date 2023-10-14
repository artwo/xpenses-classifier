use crate::service::classifier::Classifier;
use crate::util::file_util::open_encoded_file;

use glob::glob;
use std::error::Error;
use std::fmt;
use std::io::BufRead;

#[derive(Debug)]
struct FileProcessingError(String);

impl fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for FileProcessingError {}

pub struct FileProcessor<'a> {
    pub classifier: &'a Classifier,
    pub name: String,
    pub transaction_file_pattern: String,
    pub category_segment_idx: Box<[usize]>,
    pub expense_segment_idx: Box<[usize]>,
}

impl<'a> FileProcessor<'a> {
    pub fn process_files<F>(&self, mut f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(String, Option<&str>),
    {
        println!(
            "Getting the files from the directory {:?}",
            self.transaction_file_pattern
        );

        for entry in glob(self.transaction_file_pattern.as_str())? {
            match entry?.as_os_str().to_str() {
                Some(path) => self.process_file(path, |cat, val| f(cat.clone(), val))?,
                None => {
                    return Err(Box::new(FileProcessingError(format!(
                        "Unable to read entry file in the pattern {}",
                        self.transaction_file_pattern
                    ))));
                }
            }
        }
        Ok(())
    }

    pub fn process_file<F>(&self, file_path: &str, mut f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(String, Option<&str>),
    {
        let rdr = open_encoded_file(file_path)?;
        for line_result in rdr.lines() {
            let line = line_result?;
            let line_segments: &Vec<&str> = &line.split(';').collect();

            if let Some(cat) = self.find_category(line_segments) {
                f(cat, self.extract_expense_value(line_segments));
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
