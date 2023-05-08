pub mod multiselect;
pub mod selection;
pub mod multi_option_prompt;

pub struct Options<T> {
    all_options: Vec<T>,
    transformed_options: Vec<String>,
    filtered_options: Vec<usize>,
}

impl<T> Options<T>
where
    T: Into<String> + Clone,
{
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let options: Vec<T> = iter.collect();
        let options_count = options.len();
        Options {
            all_options: options.clone(),
            transformed_options: options.into_iter().map(|s| s.into()).collect(),
            filtered_options: (0..options_count).collect(),
        }
    }
}

impl<T> Options<T> {
    pub fn from_iter_transformed<I, F>(iter: I, transformation: F) -> Self
    where
        I: Iterator<Item = T>,
        F: Fn(&T) -> String,
    {
        let all_options: Vec<T> = iter.collect();
        let transformed_options: Vec<String> = all_options.iter().map(transformation).collect();
        let options_count = all_options.len();

        Options {
            all_options,
            transformed_options,
            filtered_options: (0..options_count).collect(),
        }
    }

    pub fn filter(&mut self, filter: &str) {
        self.filtered_options.clear();
        for (index, option) in self.transformed_options.iter().enumerate() {
            if option.contains(filter) {
                self.filtered_options.push(index);
            }
        }
    }

    pub fn filtered_options(&self) -> &[usize] {
        &self.filtered_options
    }

    pub fn all_options_mut(&mut self) -> &mut Vec<T> {
        &mut self.all_options
    }

    pub fn transformed_options(&self) -> &[String] {
        &self.transformed_options
    }
} 
