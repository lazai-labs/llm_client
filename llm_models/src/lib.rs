use crate::tokenizer::LlmTokenizer;
pub mod api_model;
pub mod local_model;
pub mod tokenizer;

#[allow(unused_imports)]
pub(crate) use anyhow::{anyhow, bail, Error, Result};

#[allow(unused_imports)]
pub(crate) use tracing::{debug, error, info, span, trace, warn, Level};

#[derive(Clone)]
pub struct LlmModelBase {
    pub model_id: String,
    pub model_ctx_size: u64,
    pub inference_ctx_size: u64,
    pub tokenizer: std::sync::Arc<LlmTokenizer>,
}
