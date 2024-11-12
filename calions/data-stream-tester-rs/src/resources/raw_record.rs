use super::{ActivatedMappingId, ContextId, ContextName, MappingField, RawRecordId, RecordValue};
use data_stream::stream::serialize_ulid;
use serde::Serialize;
use std::collections::HashMap;
use ulid::Ulid;

#[derive(Clone, Serialize, new)]
pub struct RawRecord {
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub id: RawRecordId,
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub activated_mapping_id: ActivatedMappingId,
    #[serde(serialize_with = "serialize_ulid")]
    #[new(default)]
    pub context_id: ContextId,
    #[new(default)]
    pub context_name: ContextName,
    #[new(default)]
    pub record: HashMap<MappingField, RecordValue>,
}

impl RawRecord {
    pub fn generate_web(
        &mut self,
        activated_mapping_id: ActivatedMappingId,
        cookie: RecordValue,
        phone: RecordValue,
    ) {
        self.id = Ulid::new();
        self.activated_mapping_id = activated_mapping_id;
        self.context_id = Ulid::from_string("01E7JNE7DMHKEGZB5FK78856A3").unwrap();
        self.context_name = String::from("web");
        let mut record: HashMap<MappingField, RecordValue> = HashMap::new();
        record.insert(String::from("web_cookie_0"), cookie);
        record.insert(String::from("web_phone_0"), phone);

        self.record = record;
    }

    pub fn generate_crm(
        &mut self,
        activated_mapping_id: ActivatedMappingId,
        email: RecordValue,
        phone: RecordValue,
    ) {
        self.id = Ulid::new();
        self.activated_mapping_id = activated_mapping_id;
        self.context_id = Ulid::from_string("01E7JP0QRT1B2M91GYZT7Y374J").unwrap();
        self.context_name = String::from("crm");
        let mut record: HashMap<MappingField, RecordValue> = HashMap::new();
        record.insert(String::from("crm_email_0"), email);
        record.insert(String::from("crm_phone_0"), phone);

        self.record = record;
    }

    pub fn generate_news(
        &mut self,
        activated_mapping_id: ActivatedMappingId,
        email: RecordValue,
        phone: RecordValue,
    ) {
        self.id = Ulid::new();
        self.activated_mapping_id = activated_mapping_id;
        self.context_id = Ulid::from_string("01E7JP4G5X168V59H7X8JR9KYE").unwrap();
        self.context_name = String::from("news");
        let mut record: HashMap<MappingField, RecordValue> = HashMap::new();
        record.insert(String::from("news_email_0"), email);
        record.insert(String::from("news_phone_0"), phone);

        self.record = record;
    }
}
