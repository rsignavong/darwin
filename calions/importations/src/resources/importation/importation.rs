use super::ImportationFilesProgressions;
use crate::decoders::{Import, ImportFile, Mapping};
use crate::processors::StatusProcessorSender;
use crate::producers::ImportationsProducerSender;
use crate::resources::{CsvLineCount, ResourcesError};
use crate::resources::{ImportInstanceId, MappingId, RecordImportInstance, S3BucketName};
use crate::resources::{Record, RecordData, RecordFile, RecordId, RecordValue};
use anyhow::anyhow;
use csv::{Reader, ReaderBuilder};
use data_stream::producer::ProducerMessage;
use data_stream::stream::OutputMessage;
use derive_new::new;
use log::info;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

#[derive(new)]
pub struct Importation {
    bucket: Arc<S3BucketName>,
    file: ImportFile,
    files_progresions: Arc<ImportationFilesProgressions>,
    import: Arc<Import>,
    import_instance_id: Arc<ImportInstanceId>,
    mapping: Arc<Vec<Mapping>>,
    mapping_id: Arc<MappingId>,
    producer: ImportationsProducerSender,
    s3_client: S3Client,
    status_processor_sender: StatusProcessorSender,
    total_lines: Arc<CsvLineCount>,
}

impl Importation {
    pub fn reader<R>(&self, rdr: R) -> Result<Reader<R>, ResourcesError>
    where
        R: Read,
    {
        let delimiter = self.import.separator.as_byte()?;

        Ok(ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(**self.import.header)
            .from_reader(rdr))
    }

    pub async fn import(&self) -> Result<(), ResourcesError> {
        info!("Fetching {} from S3", self.file.path);
        let csv = self
            .s3_client
            .get_object(GetObjectRequest {
                bucket: self.bucket.to_string(),
                key: self.file.path.to_string(),
                ..Default::default()
            })
            .await?
            .body
            .ok_or_else(|| ResourcesError::ImportationS3NoObject)?
            .into_blocking_read();

        info!("Reading {}", self.file.path);
        for (line, record) in self.reader(csv)?.records().enumerate() {
            let record = record?;
            let line = line as u64 + 1;
            let record_id = Arc::new(RecordId::new()?);
            let mut record_data: RecordData = HashMap::new();
            for mapping in self.mapping.iter() {
                let record_value = RecordValue::from_mapping(&record, &mapping)?;

                record_data.insert(mapping.field_alias.clone(), record_value);
            }

            let csv_line_count = CsvLineCount::from(line);

            let file = Arc::new(RecordFile::new(
                self.bucket.clone(),
                Arc::new(self.file.path.clone()),
                self.file.id.clone(),
                csv_line_count,
            ));
            let import_instance = Arc::new(RecordImportInstance::new(
                self.import_instance_id.clone(),
                self.total_lines.clone(),
            ));
            let record = Record::new(
                file,
                self.files_progresions.clone(),
                record_id,
                import_instance,
                self.mapping_id.clone(),
                Arc::new(record_data),
                self.status_processor_sender.clone(),
            );

            self.producer
                .send(ProducerMessage::Output(OutputMessage::new(record, None)))
                .await
                .map_err(|e| ResourcesError::ImportationProducer { source: anyhow!(e) })?;
        }
        info!("Read {} completed", self.file.path);

        Ok(())
    }
}
