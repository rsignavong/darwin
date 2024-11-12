use super::error::CliError;
use regex::Regex;
use std::str::FromStr;

static COMPONENT_PATTERN: &str = r"(?P<type>[A-Z0-9][a-z0-9]+)(?P<entity>([A-Z0-9][a-z0-9]+)*)";

#[derive(Debug, Clone)]
pub struct Component {
    type_: Option<String>,
    entity: String,
    stack: Option<String>,
    component: Option<String>,
    raw: String,
}

impl FromStr for Component {
    type Err = CliError;

    fn from_str(component: &str) -> std::result::Result<Self, Self::Err> {
        let raw = component.to_owned();
        let components: Vec<_> = component.split('.').collect();
        if components.len() > 3 {
            return Err(CliError::ComponentInvalid(component.to_owned()));
        }

        let mut component_iter = components.iter();
        let processor = component_iter
            .next()
            .ok_or_else(|| CliError::ComponentInvalid("Missing value".to_owned()))?
            .to_string();
        let component_pattern = Regex::new(COMPONENT_PATTERN)?;
        let captures = component_pattern
            .captures(&processor)
            .ok_or_else(|| CliError::ComponentInvalid("Pattern error".to_owned()))?;
        let type_ = match captures
            .name("type")
            .map(|s| s.as_str())
            .ok_or_else(|| CliError::ComponentInvalid("Cannot capture type".to_owned()))?
        {
            t @ ("Command" | "Query" | "Operator") => Some(t.to_owned()),
            _ => None,
        };
        let entity = captures
            .name("entity")
            .ok_or_else(|| CliError::ComponentInvalid("Cannot capture entity".to_owned()))?
            .as_str()
            .to_owned();
        let stack = component_iter.next().map(|s| s.to_string());
        let component = component_iter.next().map(|s| s.to_string());

        Ok(Component {
            type_,
            entity,
            stack,
            component,
            raw,
        })
    }
}

impl Component {
    pub fn get_type(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    pub fn get_entity(&self) -> &str {
        &self.entity
    }

    pub fn get_component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn get_stack(&self) -> Option<&str> {
        self.stack.as_deref()
    }
}
