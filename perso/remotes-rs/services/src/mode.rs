#[derive(Clone, Eq, PartialEq)]
pub enum Mode {
    Developement,
    Staging,
    Production,
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "development" | "dev" => Self::Developement,
            "staging" | "test" | "preprod" => Self::Staging,
            "production" | "prod" => Self::Production,
            _ => Self::Developement,
        }
    }
}

impl Mode {
    pub fn is_dev(&self) -> bool {
        *self == Self::Developement
    }
    pub fn is_staging(&self) -> bool {
        *self == Self::Staging
    }
    pub fn is_prod(&self) -> bool {
        *self == Self::Production
    }
}
