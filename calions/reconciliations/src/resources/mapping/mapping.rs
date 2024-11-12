use super::{MappingField, MappingMatchingId};
use crate::decoders::ActivatedMappings;
use crate::resources::ResourcesError;
use ahash::AHashMap;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::sync::Arc;

pub type Mapping = AHashMap<Arc<MappingField>, Arc<MappingMatchingId>>;

impl TryFrom<&ActivatedMappings> for Mapping {
    type Error = ResourcesError;

    fn try_from(activated_mappings: &ActivatedMappings) -> Result<Self, Self::Error> {
        let mut mapping = Mapping::new();

        for map in activated_mappings.mappings.iter() {
            if !*map.is_contact_point {
                continue;
            }

            let mmid = if let Some(mmid) = mapping.get(&map.field_alias) {
                mmid.clone()
            } else {
                let mmid = Arc::new(MappingMatchingId::new()?);
                mapping.insert(map.field_alias.clone(), mmid.clone());
                mmid
            };

            for mapping_field in &map.matching_mappings {
                if let Some(matching_mmid) = mapping.get(&*mapping_field) {
                    let match_mmid = matching_mmid.clone();
                    match mmid.cmp(&matching_mmid) {
                        Ordering::Less => {
                            let mut mfs: Vec<Arc<MappingField>> = Vec::new();
                            for (mf, mid) in mapping.iter() {
                                if mid == matching_mmid {
                                    mfs.push(mf.clone());
                                }
                            }
                            for mf in mfs {
                                mapping.insert(mf, mmid.clone());
                            }
                        }
                        Ordering::Greater => {
                            let mut mfs: Vec<Arc<MappingField>> = Vec::new();
                            for (mf, mid) in mapping.iter() {
                                if *mid == mmid {
                                    mfs.push(mf.clone());
                                }
                            }
                            for mf in mfs {
                                mapping.insert(mf, match_mmid.clone());
                            }
                        }
                        Ordering::Equal => continue,
                    }
                } else {
                    mapping.insert(mapping_field.clone(), mmid.clone());
                }
            }
        }

        Ok(mapping)
    }
}
