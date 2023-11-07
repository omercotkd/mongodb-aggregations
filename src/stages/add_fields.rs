use super::PipelineStage;
use bson::Document;

pub struct AddFields {
    pub fields: Document,
}