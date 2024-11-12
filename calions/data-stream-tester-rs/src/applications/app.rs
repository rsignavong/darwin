use crate::processors::{ActivatedMappingMsg, ActivatedMappingProcessor};
use crate::processors::{RawRecordMsg, RawRecordProcessor};
use crate::processors::{ReconciliationDataMsg, ReconciliationDataProcessor};
use crate::processors::{ReconciliationSagaMsg, ReconciliationSagaProcessor};
use crate::producers::{ActivatedMappingProducer, RawRecordProducer};
use crate::producers::{ReconciliationDataProducer, ReconciliationSagaProducer};
use crate::resources::{ActivatedMapping, RawRecord, ReconciliationData, ReconciliationSaga};
use crate::wizard::Wizard;
use data_stream::error::DataStreamError;
use data_stream::processor::Processor;
use data_stream::producer::Producer;
use data_stream::stream::{InputEvent, InputMessage};
use tokio::task;

pub struct App;

impl App {
    pub async fn start() -> Result<(), DataStreamError> {
        let activated_mapping = ActivatedMapping::new();
        let raw_record = RawRecord::new();
        let reconciliation_data = ReconciliationData::new();
        let reconciliation_saga = ReconciliationSaga::new();

        let activated_mapping_producer_sender =
            Producer::spawn(ActivatedMappingProducer::new(), Some(1))?;
        let raw_record_producer_sender = Producer::spawn(RawRecordProducer::new(), Some(1))?;
        let reconciliation_data_producer_sender =
            Producer::spawn(ReconciliationDataProducer::new(), Some(1))?;
        let reconciliation_saga_producer_sender =
            Producer::spawn(ReconciliationSagaProducer::new(), Some(1))?;

        let activated_mapping_processor_sender = Processor::spawn(
            ActivatedMappingProcessor::new(activated_mapping, activated_mapping_producer_sender),
            Some(1),
        )?;
        let raw_record_processor_sender = Processor::spawn(
            RawRecordProcessor::new(raw_record, raw_record_producer_sender),
            Some(1),
        )?;
        let reconciliation_data_processor_sender = Processor::spawn(
            ReconciliationDataProcessor::new(
                reconciliation_data,
                reconciliation_data_producer_sender,
            ),
            Some(1),
        )?;
        let reconciliation_saga_processor_sender = Processor::spawn(
            ReconciliationSagaProcessor::new(
                reconciliation_saga,
                reconciliation_saga_producer_sender,
            ),
            Some(1),
        )?;

        task::spawn_blocking(move || loop {
            let responses = Wizard::new().expect("Error on Wizard");
            match responses {
                Wizard::ActivatedMapping(mapping) => {
                    activated_mapping_processor_sender
                        .send(InputMessage::new(
                            InputEvent::Custom,
                            ActivatedMappingMsg(mapping),
                            None,
                        ))
                        .unwrap();
                }
                Wizard::ReconciliationData(processor_id) => {
                    reconciliation_data_processor_sender
                        .send(InputMessage::new(
                            InputEvent::Custom,
                            ReconciliationDataMsg(processor_id),
                            None,
                        ))
                        .unwrap();
                }
                Wizard::ReconciliationSaga(processor_id, activated_mapping_id) => {
                    reconciliation_saga_processor_sender
                        .send(InputMessage::new(
                            InputEvent::Custom,
                            ReconciliationSagaMsg(processor_id, activated_mapping_id),
                            None,
                        ))
                        .unwrap();
                }
                Wizard::RawRecords(activated_mapping_id, context) => {
                    raw_record_processor_sender
                        .send(InputMessage::new(
                            InputEvent::Custom,
                            RawRecordMsg(activated_mapping_id, context),
                            None,
                        ))
                        .unwrap();
                }
            };
            println!("=> Message sent to Kafka topic");
        });

        Ok(())
    }
}
