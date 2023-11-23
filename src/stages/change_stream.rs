use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

pub struct ChangeStream {
    all_changes_for_cluster: Option<bool>,
    full_document: Option<FullDocumentOptions>,
    full_document_before_change: Option<FullDocumentBeforeChangeOptions>,
    resume_after: u32,
    // version 6.0+
    show_expanded_events: bool,
    start_after: Document,
    start_at_operation_time: u32,
}

pub enum FullDocumentOptions {
    Default,
    // version 6.0+
    Required,
    UpdateLookup,
    WhenAvailable,
}

pub enum FullDocumentBeforeChangeOptions {
    Off,
    WhenAvailable,
    Required,
}

impl PipelineStage for ChangeStream {
    const NAME: &'static str = "$changeStream";
    const LOCATION: StageLocation = StageLocation::First;
}

impl ChangeStream {
    pub fn new<ID>() -> Self
    where
        ID: Into<Document>,
    {
        todo!()
    }
}

impl Into<Document> for ChangeStream {
    fn into(self) -> Document {
        todo!()
    }
}

impl Into<Stage> for ChangeStream {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
