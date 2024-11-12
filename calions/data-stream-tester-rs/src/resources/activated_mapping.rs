use super::{ActivatedMappingId, ContextMatchingId, MappingField};
use crate::wizard::Mapping;
use data_stream::stream::serialize_ulid;
use serde::Serialize;
use std::collections::HashMap;
use ulid::Ulid;

#[derive(Clone, Serialize, new)]
pub struct ActivatedMapping {
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub id: ActivatedMappingId,
    #[new(default)]
    pub activated_mapping: HashMap<MappingField, ContextMatchingId>,
}

impl ActivatedMapping {
    pub fn generate(&mut self, mapping: Mapping) {
        match mapping {
            Mapping::NewsletterEmailCRMEmail => {
                self.id = Ulid::from_string("01E7HD2ZTVREV322YNZGWC20TF").unwrap();
                let mut activated_mapping: HashMap<MappingField, ContextMatchingId> =
                    HashMap::new();
                activated_mapping.insert(
                    String::from("news_email_0"),
                    String::from("01E7HD8HNSZNMJ4D7Z7FJNN546"),
                );
                activated_mapping.insert(
                    String::from("crm_email_0"),
                    String::from("01E7HD8HNSZNMJ4D7Z7FJNN546"),
                );

                self.activated_mapping = activated_mapping;
            }
            Mapping::NewsletterEmailWebPhoneCRMEmailPhone => {
                self.id = Ulid::from_string("01E7HDAZAXABX06FZ9VCK8RQ9Q").unwrap();
                let mut activated_mapping: HashMap<MappingField, ContextMatchingId> =
                    HashMap::new();
                activated_mapping.insert(
                    String::from("news_email_0"),
                    String::from("01E7HDBPPTTZ41EBZ0JCGAEE23"),
                );
                activated_mapping.insert(
                    String::from("web_phone_0"),
                    String::from("01E7HDBPPTTZ41EBZ0JCGAEE23"),
                );
                activated_mapping.insert(
                    String::from("crm_email_0"),
                    String::from("01E7HDBPPTTZ41EBZ0JCGAEE23"),
                );
                activated_mapping.insert(
                    String::from("crm_phone_0"),
                    String::from("01E7HDBPPTTZ41EBZ0JCGAEE23"),
                );

                self.activated_mapping = activated_mapping;
            }
            Mapping::NewsletterEmailPhoneWebPhoneCookieCRMEmailPhone => {
                self.id = Ulid::from_string("01E7HDEK00RFGGV1GAG2BA4D29").unwrap();
                let mut activated_mapping: HashMap<MappingField, ContextMatchingId> =
                    HashMap::new();
                activated_mapping.insert(
                    String::from("news_email_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );
                activated_mapping.insert(
                    String::from("news_phone_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );
                activated_mapping.insert(
                    String::from("web_phone_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );
                activated_mapping.insert(
                    String::from("web_cookie_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );
                activated_mapping.insert(
                    String::from("crm_email_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );
                activated_mapping.insert(
                    String::from("crm_phone_0"),
                    String::from("01E7HDFDE5BHN3NKAFSBCWF1NE"),
                );

                self.activated_mapping = activated_mapping;
            }
        }
    }
}
