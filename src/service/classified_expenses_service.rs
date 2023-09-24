use std::collections::HashMap;

pub struct ClassifiedExpensesService<'a> {
    pub expenses_map: &'a mut HashMap<String, Vec<String>>,
}

impl<'a> ClassifiedExpensesService<'a> {
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
