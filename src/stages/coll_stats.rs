use super::{PipelineStage, Stage, StageLocation};
use bson::Document;

pub struct CollStats {}

impl PipelineStage for CollStats {
    const NAME: &'static str = "$collStats";
    const LOCATION: StageLocation = StageLocation::First;
}

impl CollStats {
    pub fn new() -> Self {
        CollStats {}
    }
}

impl Into<Document> for CollStats {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc
    }
}

impl Into<Stage> for CollStats {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
