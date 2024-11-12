use crate::{Templates, WebError};
use actix_web::web::{block, Data, Path};
use actix_web::{HttpResponse, Result};
use resources::commands::Listing as ListingCommand;
use services::{Listing as ListingService, Services};
use tera::Context;

pub struct AdPages;

impl AdPages {
    pub async fn index(services: Data<Services>) -> Result<HttpResponse> {
        let conn = services.postgresql.pool()?;
        let listings = block(move || ListingService::list(&conn)).await?;
        log::debug!("{:?}", listings);
        let body = Templates::get()
            .render(
                "web/listings/listings.html",
                &Context::from_serialize(listings)
                    .map_err(|source| WebError::TemplateContext { source })?,
            )
            .map_err(|source| WebError::TemplateRender { source })?;

        Ok(HttpResponse::Ok().body(body))
    }

    pub async fn new() -> impl IntoResponse {
        Templates("web/app.html", Context::new())
    }

    pub async fn show(
        services: Data<Services>,
        listing: Path<ListingCommand>,
    ) -> Result<HttpResponse> {
        let conn = services.postgresql.pool()?;
        let listing = block(move || ListingService::get(&conn, &listing.listing_id)).await?;
        let body = Templates::get()
            .render(
                "web/listings/listing.html",
                &Context::from_serialize(listing)
                    .map_err(|source| WebError::TemplateContext { source })?,
            )
            .map_err(|source| WebError::TemplateRender { source })?;

        Ok(HttpResponse::Ok().body(body))
    }
}
