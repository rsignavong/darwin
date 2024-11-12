use super::ListingError;
use repository::{Crud, PgConn};
use resources::entities::JobListingId;
use resources::queries::{ListingDetail, Listings};
use std::convert::TryFrom;

pub struct Listing;

impl Listing {
    pub fn get(conn: &PgConn, listing_id: &str) -> Result<ListingDetail, ListingError> {
        let job_listing_id = JobListingId::try_from(listing_id)?;
        let detail = ListingDetail::new("detail".into());
        Ok(detail)
    }

    pub fn list(conn: &PgConn) -> Result<Listings, ListingError> {
        let listings = Listings::new(vec!["first".into(), "second".into(), "third".into()], 1, 10);
        Ok(listings)
    }
}
