use super::handlers::{AdPages, Ads, Apps, Assets, BlogPages, Blogs, Home, Sessions};
use axum::{
    handler::{get, post, put, Handler},
    routing::BoxRoute,
    AddExtensionLayer, Router as AxumRouter,
};
use services::Services;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

pub struct Router;

impl Router {
    pub fn new(services: Services) -> AxumRouter<BoxRoute> {
        let middleware_stack = ServiceBuilder::new()
            .timeout(Duration::from_secs(30))
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(AddExtensionLayer::new(services))
            .into_inner();

        let home = AxumRouter::new().route("/", get(Home::show));

        let ad_pages = AxumRouter::new()
            .route("/", get(AdPages::index))
            .route("/:job_id", get(AdPages::show));

        let blog_pages = AxumRouter::new()
            .route("/", get(BlogPages::index))
            .route("/new", get(BlogPages::new))
            .route("/:article_slug", get(BlogPages::show));

        // TODO cookies
        let private_api = AxumRouter::new()
            .route("/session", get(Sessions::show))
            .route("/ads", get(Ads::index).post(Ads::create))
            .route(
                "/ads/:ad_id",
                get(Ads::show).put(Ads::update).delete(Ads::delete),
            )
            .route("/blogs", get(Blogs::index).post(Blogs::create))
            .route(
                "/blogs/:article_id",
                get(Blogs::show).put(Blogs::update).delete(Blogs::delete),
            );

        let api = AxumRouter::new()
            .route("/", get(Status::show))
            .route("/auth", post(Sessions::create))
            .nest("/private", private_api);

        let app = AxumRouter::new().route("/", get(Apps::show));

        let assets = AxumRouter::new().route("/", get(Assets::show));

        // TODO add 404 page

        AxumRouter::new()
            .route("/", home)
            .nest("/api", api)
            .nest("/app", app)
            .nest("/blog", blog_pages)
            .nest("/jobs", ad_pages)
            .nest("/static", assets)
            .layer(middleware_stack)
            .boxed()
    }
}
