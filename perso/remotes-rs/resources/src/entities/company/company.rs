use super::CompanyWebsiteUrl;
use super::{CompanyDescription, CompanyId, CompanyLogoUrl, CompanyName, CompanyTagLine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Company {
    id: CompanyId,
    name: CompanyName,
    description: CompanyDescription,
    logo_url: CompanyLogoUrl,
    tag_line: CompanyTagLine,
    website_url: CompanyWebsiteUrl,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
