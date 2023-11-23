use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

pub struct Documents {
    expr: Bson,
}

impl PipelineStage for Documents {
    const NAME: &'static str = "$documents";
    const LOCATION: StageLocation = StageLocation::Any;
}

impl Into<Document> for Documents {
    fn into(self) -> Document {
        doc! {
            Self::NAME: self.expr
        }
    }
}

impl Into<Stage> for Documents {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
