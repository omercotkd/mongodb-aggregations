use super::{PipelineStage, Stage, StageLocation};
use bson::Document;

pub struct Count {
    pub field: String,
}

impl PipelineStage for Count {
    const NAME: &'static str = "$addFields";
    const LOCATION: StageLocation = StageLocation::Any;
}

impl Count {
    pub fn new<IS>(field: IS) -> Self
    where
        IS: Into<String>,
    {
        Count {
            field: field.into(),
        }
    }
}

impl Into<Document> for Count {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.field);
        doc
    }
}

impl Into<Stage> for Count {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
