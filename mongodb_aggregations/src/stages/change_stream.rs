use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct ChangeStream {
    #[builder(setter(strip_option), default = "None")]
    all_changes_for_cluster: Option<bool>,
    #[builder(setter(strip_option), default = "None")]
    full_document: Option<FullDocumentOptions>,
    #[builder(setter(strip_option), default = "None")]
    full_document_before_change: Option<FullDocumentBeforeChangeOptions>,
    resume_after: u32,
    #[cfg(feature = "v6_0")]
    show_expanded_events: bool,
    start_after: Document,
    start_at_operation_time: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum FullDocumentOptions {
    Default,
    #[cfg(feature = "v6_0")]
    Required,
    UpdateLookup,
    #[cfg(feature = "v6_0")]
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
    pub fn new<ID>(
        all_changes_for_cluster: Option<bool>,
        full_document: Option<FullDocumentOptions>,
        full_document_before_change: Option<FullDocumentBeforeChangeOptions>,
        resume_after: u32,
        #[cfg(feature = "v6_0")] show_expanded_events: bool,
        start_after: ID,
        start_at_operation_time: u32,
    ) -> Self
    where
        ID: Into<Document>,
    {
        ChangeStream {
            all_changes_for_cluster,
            full_document,
            full_document_before_change,
            resume_after,
            #[cfg(feature = "v6_0")]
            show_expanded_events,
            start_after: start_after.into(),
            start_at_operation_time,
        }
    }
}

impl Into<Document> for ChangeStream {
    fn into(self) -> Document {
        let mut fields = Document::new();

        if let Some(all_changes_for_cluster) = self.all_changes_for_cluster {
            fields.insert("allChangesForCluster", all_changes_for_cluster);
        }

        if let Some(full_document) = self.full_document {
            fields.insert("fullDocument", full_document);
        }

        if let Some(full_document_before_change) = self.full_document_before_change {
            fields.insert("fullDocument", full_document_before_change);
        }

        #[cfg(feature = "v6_0")]
        fields.insert("showExpandedEvents", self.show_expanded_events);

        fields.insert("resumeAfter", self.resume_after);

        fields.insert("startAfter", self.start_after);

        fields.insert("startAtOperationTime", self.start_at_operation_time);

        doc! {
            Self::NAME: fields
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

impl Into<Bson> for FullDocumentOptions {
    fn into(self) -> Bson {
        match self {
            FullDocumentOptions::Default => Bson::String("default".to_string()),
            #[cfg(feature = "v6_0")]
            FullDocumentOptions::Required => Bson::String("required".to_string()),
            FullDocumentOptions::UpdateLookup => Bson::String("updateLookup".to_string()),
            #[cfg(feature = "v6_0")]
            FullDocumentOptions::WhenAvailable => Bson::String("whenAvailable".to_string()),
        }
    }
}

impl Into<Bson> for FullDocumentBeforeChangeOptions {
    fn into(self) -> Bson {
        match self {
            FullDocumentBeforeChangeOptions::Off => Bson::String("off".to_string()),
            FullDocumentBeforeChangeOptions::WhenAvailable => {
                Bson::String("whenAvailable".to_string())
            }
            FullDocumentBeforeChangeOptions::Required => Bson::String("required".to_string()),
        }
    }
}
