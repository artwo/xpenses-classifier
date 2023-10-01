use crate::service::file_classifier::FileProcessor;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ClassifyingError(String);

impl fmt::Display for ClassifyingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for ClassifyingError {}

pub struct ClassifiedExpensesService<'a> {
    pub expenses_map: &'a mut HashMap<String, Vec<String>>,
    pub file_processors: &'a Vec<FileProcessor<'a>>,
}

impl<'a> ClassifiedExpensesService<'a> {
    pub fn process_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(p) = self.file_processors.first() {
            return p.process_file(file_path, |cat, val| self.add_to_classified(cat, val));
        }
        Err(Box::new(ClassifyingError(
            "There is no file processor available".to_string(),
        )))
    }

    pub fn add_to_classified(&mut self, category: String, value: Option<&str>) {
        let new_value: String = match value {
            Some(v) => String::from(v),
            None => return,
        };

        match self.expenses_map.get_mut(category.as_str()) {
            Some(values) => values.push(new_value),
            None => {
                self.expenses_map
                    .insert(category.clone(), Vec::from([new_value]));
            }
        }
    }

    pub fn write_to_file(&self) {
        // TODO: Implement
    }
}
