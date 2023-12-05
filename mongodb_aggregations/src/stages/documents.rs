use bson::{Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(into_document = true)]
pub struct Documents {
    expr: Bson,
}

// impl Into<Document> for Documents {
//     fn into(self) -> Document {
//         doc! {
//             Self::NAME: self.expr
//         }
//     }
// }

