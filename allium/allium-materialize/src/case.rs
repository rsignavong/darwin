use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};

pub struct Case;

impl Case {
    pub fn to_camel(input: &str) -> String {
        input.to_lower_camel_case()
    }

    pub fn to_pascal(input: &str) -> String {
        input.to_upper_camel_case()
    }

    pub fn to_kebab(input: &str) -> String {
        input.to_kebab_case()
    }

    pub fn to_snake(input: &str) -> String {
        input.to_snake_case()
    }

    pub fn to_title(input: &str) -> String {
        input.to_title_case()
    }
}
