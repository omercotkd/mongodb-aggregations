use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Document};

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct ChangeStreanSplitLargeEvent {
    fragments: i32,
    of: i32,
}

impl PipelineStage for ChangeStreanSplitLargeEvent {
    const NAME: &'static str = "$changeStreamLargestEventSplit";
    const LOCATION: StageLocation = StageLocation::First;
}

impl Into<Document> for ChangeStreanSplitLargeEvent {
    fn into(self) -> Document {
        let mut fields = Document::new();

        fields.insert("fragments", self.fragments);
        fields.insert("of", self.of);

        doc! {
            Self::NAME: fields
        }
    }
}

impl Into<Stage> for ChangeStreanSplitLargeEvent {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
