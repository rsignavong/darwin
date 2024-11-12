use super::{ReconciliationCount, ReconciliationKey, ReconciliationProfiles};
use crate::decoders::AnonymizationBody;
use crate::decoders::ReconciliationProcessBody;
use crate::decoders::{RawRecordBody, RawRecordMetadata};
use crate::decoders::{ReconciliationDataBody, ReconciliationDataMetadata};
use crate::resources::OrganizationId;
use crate::resources::{Mapping, MappingId};
use crate::resources::{ProcessorId, ResourcesError, TransactionId};
use crate::resources::{Profile, ProfileCount, ProfileId, ProfileLog};
use crate::resources::{ReconciliationData, ReconciliationDataProfile, ReconciliationDataRecord};
use crate::resources::{ReconciliationDataContext, ReconciliationDataMapping};
use crate::resources::{Record, RecordProfile};
use derive_new::new;
use log::{info, warn};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct Reconciliation {
    processor_id: Arc<ProcessorId>,
    #[new(default)]
    mapping_id: Option<Arc<MappingId>>,
    #[new(default)]
    mapping: Mapping,
    #[new(default)]
    organization_id: Option<Arc<OrganizationId>>,
    #[new(default)]
    profiles: Option<ReconciliationProfiles>,
    #[new(default)]
    reconciliations_count: ReconciliationCount,
}

impl Reconciliation {
    pub fn anonymize_profile(
        &mut self,
        anonymization_body: &AnonymizationBody,
    ) -> Result<(), ResourcesError> {
        let organization_id = self
            .organization_id
            .clone()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingOrganizationId)?;

        if !organization_id.eq(&anonymization_body.organization_id) {
            warn!(
                "Wrong OrganizationId {:?}, expecting {:?}",
                anonymization_body.organization_id, organization_id
            );
            return Ok(());
        }

        if !anonymization_body.is_validated.is_validated() {
            return Ok(());
        }

        self.profiles.as_mut().map(|profiles| {
            let mut keys: Vec<ReconciliationKey> = Vec::new();
            for (key, pid) in profiles.iter() {
                if pid.eq(&*anonymization_body.contact_id) {
                    keys.push(key.clone());
                }
            }

            for key in keys.iter() {
                profiles.remove(key);
            }

            info!(
                "Profile Id {:?} successfully anonymized",
                anonymization_body.contact_id.to_string()
            );
        });

        Ok(())
    }

    pub fn counts(&self) -> (Arc<ProfileCount>, Arc<ReconciliationCount>) {
        let profiles_count = Arc::new(ProfileCount::from(
            self.profiles
                .as_ref()
                .map(|profiles| profiles.len() as u64)
                .unwrap_or_default(),
        ));
        let reconciliations_count = Arc::new(self.reconciliations_count.clone());
        (profiles_count, reconciliations_count)
    }

    pub fn is_ready(&self) -> bool {
        let ready =
            self.mapping_id.is_some() && self.profiles.is_some() && !self.mapping.is_empty();
        info!(
            "Reconciliations is {}",
            if ready { "ready" } else { "not ready..." }
        );
        ready
    }

    pub fn mapping_id(&self) -> Result<Arc<MappingId>, ResourcesError> {
        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| ResourcesError::ReconciliationMissingActivatedMappingId)?;
        Ok(mapping_id)
    }

    pub fn match_record(
        &mut self,
        raw_record_body: &RawRecordBody,
        raw_record_metadata: &RawRecordMetadata,
    ) -> Result<Option<(Record, Vec<ReconciliationDataRecord>)>, ResourcesError> {
        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| ResourcesError::ReconciliationMatchMissingActivatedMappingId)?;

        if !mapping_id.eq(&raw_record_metadata.activated_mapping_id) {
            warn!(
                "Wrong ActivatedMappingId {:?}, expecting {:?}",
                raw_record_metadata.activated_mapping_id, mapping_id
            );
            return Ok(None);
        }

        let transaction_id = Arc::new(TransactionId::new()?);

        let mut profile = Profile::New;

        let profiles: Vec<(ReconciliationKey, Profile)> = raw_record_body
            .record
            .iter()
            .filter_map(
                |(mapping_field, record_value)| -> Option<(ReconciliationKey, Profile)> {
                    let mapping_matching_id = self.mapping.get(&*mapping_field)?;
                    let reconciliation_key =
                        ReconciliationKey::new(mapping_matching_id.clone(), record_value.clone());
                    let curr_profile = if let Some(profile_id) = self
                        .profiles
                        .as_ref()
                        .and_then(|profiles| profiles.get(&reconciliation_key))
                    {
                        Profile::Old(profile_id.clone())
                    } else {
                        Profile::New
                    };

                    if curr_profile.cmp(&profile) == Ordering::Less {
                        profile = curr_profile.clone();
                    }
                    Some((reconciliation_key, curr_profile))
                },
            )
            .collect();

        let record_profile = Arc::new(RecordProfile::try_from(profile)?);

        let mut merge: Vec<ProfileId> = Vec::new();
        let mut profiles_logs: Vec<ProfileLog> = Vec::new();

        for (reconciliation_key, curr_profile) in profiles {
            if let Profile::Old(old_profile_id) = curr_profile {
                if old_profile_id == *record_profile.unwrap() {
                    continue;
                }
                self.profiles.as_mut().map(|profiles| {
                    for (key, p_id) in profiles.iter_mut() {
                        if *p_id == old_profile_id {
                            *p_id = record_profile.unwrap().clone();
                            profiles_logs.push(ProfileLog::new(
                                key.clone(),
                                Some(old_profile_id.clone()),
                                record_profile.unwrap().clone(),
                            ));
                        }
                    }
                });
                merge.push(old_profile_id);
            } else {
                self.profiles.as_mut().map(|profiles| {
                    profiles.insert(reconciliation_key.clone(), record_profile.unwrap().clone())
                });
                profiles_logs.push(ProfileLog::new(
                    reconciliation_key,
                    None,
                    record_profile.unwrap().clone(),
                ));
            }
        }

        self.reconciliations_count.inc();

        let (profiles_count, reconciliations_count) = self.counts();

        let record = Record::new(
            raw_record_body.record.clone(),
            mapping_id,
            Arc::new(merge),
            self.processor_id.clone(),
            record_profile,
            profiles_count,
            reconciliations_count.clone(),
            raw_record_metadata.source.clone(),
            transaction_id,
        );

        let mut reconciliation_data_records: Vec<ReconciliationDataRecord> = Vec::new();
        for log in profiles_logs.into_iter() {
            let profile =
                ReconciliationDataProfile::new(reconciliations_count.clone(), Arc::new(log));
            let reconciliation_data_record = ReconciliationDataRecord::new(
                ReconciliationData::Profile(profile),
                self.processor_id.clone(),
            );
            reconciliation_data_records.push(reconciliation_data_record);
        }

        Ok(Some((record, reconciliation_data_records)))
    }

    pub fn set_mapping(
        &mut self,
        rpb: &ReconciliationProcessBody,
    ) -> Result<Option<Vec<ReconciliationDataRecord>>, ResourcesError> {
        if !self.processor_id.eq(&rpb.processor_id) {
            return Ok(None);
        }

        if self.mapping_id.is_none() {
            self.mapping_id = Some(rpb.activated_mappings.id.clone());
        }

        if self.mapping.is_empty() {
            self.mapping = Mapping::try_from(&rpb.activated_mappings)?;
        }

        if self.organization_id.is_none() {
            self.organization_id = Some(rpb.organization_id.clone());
        }

        if self.profiles.is_none() {
            self.profiles = Some(ReconciliationProfiles::new());
        }

        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| ResourcesError::ReconciliationSetMappingMissingActivatedMappingId)?;

        let organization_id = self
            .organization_id
            .clone()
            .ok_or_else(|| ResourcesError::ReconciliationSetMappingMissingOrganizationId)?;

        info!("Activated Mapping: {}", mapping_id);

        let mut reconciliation_data_records: Vec<ReconciliationDataRecord> = Vec::new();
        for (field, matching_mapping_id) in self.mapping.iter() {
            let mapping =
                ReconciliationDataMapping::new(field.clone(), matching_mapping_id.clone());
            let reconciliation_data_record = ReconciliationDataRecord::new(
                ReconciliationData::Mapping(mapping),
                self.processor_id.clone(),
            );
            reconciliation_data_records.push(reconciliation_data_record);
        }

        let context = ReconciliationDataContext::new(mapping_id, organization_id);
        let reconciliation_data_record = ReconciliationDataRecord::new(
            ReconciliationData::Context(context),
            self.processor_id.clone(),
        );
        reconciliation_data_records.push(reconciliation_data_record);

        Ok(Some(reconciliation_data_records))
    }

    pub fn set_state(
        &mut self,
        rdb: &ReconciliationDataBody,
        rdm: &ReconciliationDataMetadata,
    ) -> Result<(), ResourcesError> {
        if !self.processor_id.eq(&rdm.processor_id) {
            return Ok(());
        }

        match rdb {
            ReconciliationData::Context(ref context) => {
                let mapping_id = Some(context.mapping_id.clone());
                if self.mapping_id != mapping_id {
                    self.mapping_id = mapping_id;
                }

                let organization_id = Some(context.organization_id.clone());
                if self.organization_id != organization_id {
                    self.organization_id = organization_id;
                }
            }
            ReconciliationData::Mapping(ref mapping) => {
                self.mapping
                    .insert(mapping.field.clone(), mapping.mapping_matching_id.clone());
            }
            ReconciliationData::Profile(ref profile) => {
                let reconciliation_key = profile.log.key.clone();
                let profile_id = profile.log.new.clone();
                if let Some(profiles) = self.profiles.as_mut() {
                    profiles.insert(reconciliation_key, profile_id);
                } else {
                    let mut profiles = ReconciliationProfiles::new();
                    profiles.insert(reconciliation_key, profile_id);
                    self.profiles = Some(profiles);
                }

                // Reconciliations count can only increment so we keep the highest
                if profile.count.as_ref() > &self.reconciliations_count {
                    self.reconciliations_count = profile.count.as_ref().clone();
                }
            }
            ReconciliationData::Topics(_) => (),
        }

        Ok(())
    }

    pub fn set_state_delete(
        &mut self,
        rdb: &ReconciliationDataBody,
        rdm: &ReconciliationDataMetadata,
    ) -> Result<(), ResourcesError> {
        if !self.processor_id.eq(&rdm.processor_id) {
            return Ok(());
        }

        match rdb {
            ReconciliationData::Mapping(ref mapping) => {
                self.mapping.remove(mapping.field.as_ref());
            }
            ReconciliationData::Profile(ref profile) => {
                if let Some(profiles) = self.profiles.as_mut() {
                    profiles.remove(&profile.log.key);
                }

                self.reconciliations_count = profile.count.as_ref().clone();
            }
            _ => (),
        }

        Ok(())
    }
}
