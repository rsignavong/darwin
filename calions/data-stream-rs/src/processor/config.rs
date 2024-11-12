use derive_builder::Builder;

#[derive(Clone, Builder, Debug)]
pub struct ProcessorConfig {
    pub(crate) name: &'static str,
}
