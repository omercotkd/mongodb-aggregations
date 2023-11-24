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

trait PipelineStage: Into<Document> + Into<Stage> {
    const NAME: &'static str;
    const LOCATION: StageLocation;
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

impl Stage {
    pub fn new(location: StageLocation, doc: Document, name: &'static str) -> Self {
        Stage {
            location,
            doc,
            name,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

mod add_fields;
mod bucket;
mod change_stream;
mod coll_stats;
mod count;
mod current_op;
mod densify;
mod documents;
mod facet;
mod fill;
mod group;
mod index_stats;
mod limit;
mod list;
mod lookup;
mod match_;
mod merge;
mod out;
mod plan_cache_stats;
mod project;
mod redact;
mod replace;
mod sample;
mod search;
mod set;
mod set_window_fields;
mod skip;
mod sort;
mod union_with;
mod unset;
mod unwind;
pub use add_fields::AddFields;
pub use bucket::{Bucket, BucketAuto, Granularity};
pub use change_stream::{ChangeStream, FullDocumentBeforeChangeOptions, FullDocumentOptions};
pub use coll_stats::CollStats;
pub use count::Count;
pub use documents::Documents;
pub use group::Group;
pub use limit::Limit;
pub use lookup::Lookup;
pub use project::Project;
pub use unwind::Unwind;
