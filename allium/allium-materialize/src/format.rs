use inflector::Inflector;

pub struct Format;

impl Format {
    pub fn pluralize(input: &str) -> String {
        input.to_plural()
    }

    pub fn singularize(input: &str) -> String {
        input.to_singular()
    }
}
