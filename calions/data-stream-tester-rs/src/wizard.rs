use crate::resources::{ActivatedMappingId, ProcessorId, RecordValue};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use ulid::Ulid;

enum Id {
    ActivatedMappingId,
    ProcessorId,
}

enum Value {
    Cookie,
    Email,
    Phone,
}

pub enum Context {
    Web {
        cookie: RecordValue,
        phone: RecordValue,
    },
    CRM {
        email: RecordValue,
        phone: RecordValue,
    },
    Newsletter {
        email: RecordValue,
        phone: RecordValue,
    },
}

#[derive(Clone, Copy)]
pub enum Mapping {
    NewsletterEmailCRMEmail,
    NewsletterEmailWebPhoneCRMEmailPhone,
    NewsletterEmailPhoneWebPhoneCookieCRMEmailPhone,
}

pub enum Wizard {
    ActivatedMapping(Mapping),
    ReconciliationData(ProcessorId),
    ReconciliationSaga(ProcessorId, ActivatedMappingId),
    RawRecords(ActivatedMappingId, Context),
}

impl Wizard {
    pub fn new() -> IoResult<Self> {
        let common = Self::ask_common()?;
        Self::ask_details(common)
    }

    fn ask_common() -> IoResult<usize> {
        let streams = vec![
            "Activated Mapping",
            "Reconciliation data",
            "Reconciliation Saga",
            "Raw Records",
        ];

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick one Stream")
            .default(0)
            .items(&streams)
            .interact()
    }

    fn ask_details(common: usize) -> IoResult<Self> {
        match common {
            0 => {
                let mapping = Self::ask_mapping()?;
                Ok(Wizard::ActivatedMapping(mapping))
            }
            1 => {
                let processor_id = Self::ask_ulid(Id::ProcessorId)?;
                Ok(Wizard::ReconciliationData(processor_id))
            }
            2 => {
                let processor_id = Self::ask_ulid(Id::ProcessorId)?;
                let activated_mapping_id = Self::ask_ulid(Id::ActivatedMappingId)?;
                Ok(Wizard::ReconciliationSaga(
                    processor_id,
                    activated_mapping_id,
                ))
            }
            3 => {
                let activated_mapping_id = Self::ask_ulid(Id::ActivatedMappingId)?;
                let context = Self::ask_context()?;
                Ok(Wizard::RawRecords(activated_mapping_id, context))
            }
            _ => Err(IoError::new(ErrorKind::Other, "Non existing choice")),
        }
    }

    fn ask_context() -> IoResult<Context> {
        let contexts = vec!["Web", "CRM", "Newsletter"];

        let select_context = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick one Context")
            .default(0)
            .items(&contexts)
            .interact()?;

        let context = match select_context {
            0 => Context::Web {
                phone: Self::ask_string(Value::Phone)?,
                cookie: Self::ask_string(Value::Cookie)?,
            },
            1 => Context::CRM {
                email: Self::ask_string(Value::Email)?,
                phone: Self::ask_string(Value::Phone)?,
            },
            2 => Context::Newsletter {
                email: Self::ask_string(Value::Email)?,
                phone: Self::ask_string(Value::Phone)?,
            },
            _ => Err(IoError::new(ErrorKind::Other, "Non existing choice"))?,
        };

        Ok(context)
    }

    fn ask_string(value: Value) -> IoResult<String> {
        let s = match value {
            Value::Cookie => "Cookie",
            Value::Email => "Email",
            Value::Phone => "Phone",
        };
        Input::<String>::new()
            .with_prompt(format!("Enter {}", s))
            .interact()
    }

    fn ask_mapping() -> IoResult<Mapping> {
        let mappings = vec![
            "Newsletter (email) & CRM (email)",
            "Newsletter (email)  & Web (phone) & CRM (email + phone)",
            "Newsletter (email + phone) & Web (phone + cookie) & CRM (email + phone)",
        ];

        let select_mapping = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick one Mapping")
            .default(0)
            .items(&mappings)
            .interact()?;

        let mapping = match select_mapping {
            0 => Mapping::NewsletterEmailCRMEmail,
            1 => Mapping::NewsletterEmailWebPhoneCRMEmailPhone,
            2 => Mapping::NewsletterEmailPhoneWebPhoneCookieCRMEmailPhone,
            _ => Err(IoError::new(ErrorKind::Other, "Non existing choice"))?,
        };

        Ok(mapping)
    }

    fn ask_ulid(id: Id) -> IoResult<Ulid> {
        let s = match id {
            Id::ActivatedMappingId => "ActivatedMappingId",
            Id::ProcessorId => "ProcessorId",
        };
        let ulid_str = Input::<String>::new()
            .with_prompt(format!("Enter {}", s))
            .interact()?;
        Ulid::from_string(&ulid_str)
            .map_err(|e| IoError::new(ErrorKind::Other, format!("{}: {}", e.to_string(), ulid_str)))
    }
}
