use bson::Document;

pub enum StageLocation {
    First,
    Last,
    Any,
}

pub trait PipelineStage: Into<Document> {
    const NAME: &'static str;
    const LOCATION: StageLocation = StageLocation::Any;
}

mod add_fields;
pub use add_fields::AddFields;
