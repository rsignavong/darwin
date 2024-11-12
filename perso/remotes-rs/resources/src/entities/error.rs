use super::{CompanyError, CreditError, JobError, JobListingError, TagError, TaggedJobError};
use super::{FeatureError, JobCategoryError, JobCommentError, JobDetailError, JobTypeError};
use super::{FeedbackError, UserAccountError, UserError, UserSessionError};
use super::{PackagingError, PaymentError, ProductError, PromotionError, PurchaseOrderError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("Company")]
    Company(#[from] CompanyError),
    #[error("Credit")]
    Credit(#[from] CreditError),
    #[error("Feature")]
    Feature(#[from] FeatureError),
    #[error("Feedback")]
    Feedback(#[from] FeedbackError),
    #[error("Job")]
    Job(#[from] JobError),
    #[error("JobCategory")]
    JobCategory(#[from] JobCategoryError),
    #[error("JobComment")]
    JobComment(#[from] JobCommentError),
    #[error("JobDetail")]
    JobDetail(#[from] JobDetailError),
    #[error("JobType")]
    JobType(#[from] JobTypeError),
    #[error("JobListing")]
    JobListing(#[from] JobListingError),
    #[error("Packaging")]
    Packaging(#[from] PackagingError),
    #[error("Payment")]
    Payment(#[from] PaymentError),
    #[error("Product")]
    Product(#[from] ProductError),
    #[error("Promotion")]
    Promotion(#[from] PromotionError),
    #[error("PurchaseOrder")]
    PurchaseOrder(#[from] PurchaseOrderError),
    #[error("Tag")]
    Tag(#[from] TagError),
    #[error("Tagged")]
    TaggedJob(#[from] TaggedJobError),
    #[error("User")]
    User(#[from] UserError),
    #[error("UserAccount")]
    UserAccount(#[from] UserAccountError),
    #[error("UserSession")]
    UserSession(#[from] UserSessionError),
}
