use bson::Document;

pub enum StageLocation {
    First,
    Last,
    Any,
}

pub struct Stage {
    pub location: StageLocation,
    pub doc: Document,
    pub name: &'static str,
}

pub trait PipelineStage: Into<Document> + Into<Stage> {
    const NAME: &'static str;
    const LOCATION: StageLocation = StageLocation::Any;
}

impl StageLocation {
    pub fn is_first(&self) -> bool {
        match self {
            StageLocation::First => true,
            _ => false,
        }
    }

    pub fn is_last(&self) -> bool {
        match self {
            StageLocation::Last => true,
            _ => false,
        }
    }
}

mod add_fields;
mod bucket;
mod count;
mod group;
mod limit;
mod lookup;
mod project;
mod unwind;
pub use add_fields::AddFields;
pub use bucket::{Bucket, BucketAuto};
pub use count::Count;
pub use group::Group;
pub use limit::Limit;
pub use lookup::Lookup;
pub use project::Project;
pub use unwind::Unwind;