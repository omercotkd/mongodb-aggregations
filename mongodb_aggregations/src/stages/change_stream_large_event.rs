use bson::doc;
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(location = "first")]
#[pipeline_stage(into_document = true)]
pub struct ChangeStreanSplitLargeEvent {
    fragments: i32,
    of: i32,
}

impl ChangeStreanSplitLargeEvent {
    pub fn new(fragments: i32, of: i32) -> Self {
        ChangeStreanSplitLargeEvent { fragments, of }
    }

    pub fn fragments(&self) -> i32 {
        self.fragments
    }

    pub fn of(&self) -> i32 {
        self.of
    }
}
