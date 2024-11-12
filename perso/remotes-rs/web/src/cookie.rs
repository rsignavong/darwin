use super::WebError;
use actix_session::CookieSession;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Cookie {
    domain: Option<String>,
    name: String,
    private_key: String,
    secure: bool,
}

impl Cookie {
    pub fn is_secure(&self) -> Result<(), WebError> {
        if self.private_key.len() != 32 {
            return Err(WebError::CookiePrivateKeyInvalidLength);
        }

        Ok(())
    }

    pub fn session(&self) -> CookieSession {
        let session = CookieSession::private(self.private_key.as_bytes())
            .name(&self.name)
            .lazy(true);

        let session = if let Some(domain) = &self.domain {
            session.domain(domain)
        } else {
            session
        };

        let session = if self.secure {
            session.secure(true)
        } else {
            session
        };

        session
    }
}
