use super::{PipelineStage, Stage};
use bson::{doc, Bson, Document};


pub struct ChangeStream {
    all_changes_for_cluster: bool,
    full_document: bool,
    full_document_before_change: bool,
    resume_after: u32,
    show_expanded_events: bool,
    start_after: Document,
    start_at_operation_time: u32,
}

impl PipelineStage for ChangeStream {
    const NAME: &'static str = "$changeStream";
}

impl ChangeStream {
    pub fn new<ID>(fields: Option<ID>) -> Self
    where
        ID: Into<Document>,
    {
        todo!()
    }

}

impl Into<Document> for ChangeStream {
    fn into(self) -> Document {
        doc! {
            Self::NAME: self.fields
        }
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

