use super::{PipelineStage, Stage};
use bson::Document;

pub struct Count<'a> {
    pub field: &'a str,
}

impl<'a> PipelineStage for Count<'a> {
    const NAME: &'static str = "$addFields";
}

impl<'a> Count<'a> {
    pub fn new(field: &'a str) -> Self {
        Count { field }
    }
}

impl<'a> Into<Document> for Count<'a> {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.field);
        doc
    }
}

impl<'a> Into<Stage> for Count<'a> {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
