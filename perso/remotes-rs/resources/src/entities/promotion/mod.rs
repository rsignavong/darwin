mod error;
mod promotion;
mod promotion_date;
mod promotion_description;
mod promotion_id;
mod promotion_name;
mod promotion_period;
mod promotion_price;

pub use error::PromotionError;
pub use promotion::Promotion;
pub use promotion_date::PromotionDate;
pub use promotion_description::PromotionDescription;
pub use promotion_id::PromotionId;
pub use promotion_name::PromotionName;
pub use promotion_period::PromotionPeriod;
pub use promotion_price::PromotionPrice;
