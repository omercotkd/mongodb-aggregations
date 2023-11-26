use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct ChangeStream {
    #[builder(setter(strip_option))]
    all_changes_for_cluster: Option<bool>,
    #[builder(setter(strip_option))]
    full_document: Option<FullDocumentOptions>,
    #[builder(setter(strip_option))]
    full_document_before_change: Option<FullDocumentBeforeChangeOptions>,
    resume_after: u32,
    // version 6.0+
    show_expanded_events: bool,
    start_after: Document,
    start_at_operation_time: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum FullDocumentOptions {
    Default,
    // version 6.0+
    Required,
    UpdateLookup,
    WhenAvailable,
}

#[derive(Debug, Clone, Copy)]
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
