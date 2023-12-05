use bson::{Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(location = "first", into_document = false)]
pub struct Count {
    pub field: String,
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

// impl Into<Document> for Count {
//     fn into(self) -> Document {
//         let mut doc = Document::new();
//         doc.insert(Self::NAME, self.field);
//         doc
//     }
// }

