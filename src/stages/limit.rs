use super::{PipelineStage, Stage};
use bson::Document;

pub struct Limit {
    pub limit: i64,
}

impl PipelineStage for Limit {
    const NAME: &'static str = "$limit";
}

impl Limit {
    pub fn new<'a>(
        limit: i64,
    ) -> Self {

        Limit {
            limit
        }
    }
}

impl Into<Document> for Limit {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.limit);
        doc
    }
}

impl Into<Stage> for Limit {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}