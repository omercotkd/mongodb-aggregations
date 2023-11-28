use super::{PipelineStage, Stage, StageLocation};
use bson::Document;

pub struct Lookup {
    from: Option<String>,
    local_field: String,
    foreign_field: String,
    as_field: String,
    pipeline: Option<Vec<Document>>,
    let_: Option<Document>,
}


pub struct GraphLookup {
    from: Option<String>,
    start_with: Option<String>,
    connect_from_field: String,
    connect_to_field: String,
    as_field: String,
    max_depth: Option<i32>,
    depth_field: Option<String>,
    restrict_search_with_match: Option<Document>,
}


impl PipelineStage for Lookup {
    const NAME: &'static str = "$lookup";
    const LOCATION: StageLocation = StageLocation::Any;
}

impl Lookup {
    pub fn new(
        from: Option<impl Into<String>>,
        local_field: impl Into<String>,
        foreign_field: impl Into<String>,
        as_field: impl Into<String>,
        pipeline: Option<Vec<Document>>,
        let_: Option<Document>,
    ) -> Self {
        Lookup {
            from: from.map(|s| s.into()),
            local_field: local_field.into(),
            foreign_field: foreign_field.into(),
            as_field: as_field.into(),
            pipeline,
            let_,
        }
    }
}

impl Into<Document> for Lookup {
    fn into(self) -> Document {
        let mut doc = Document::new();
        if let Some(from) = self.from {
            doc.insert("from", from);
        }
        doc.insert("localField", self.local_field);
        doc.insert("foreignField", self.foreign_field);
        doc.insert("as", self.as_field);
        if let Some(pipeline) = self.pipeline {
            doc.insert("pipeline", pipeline);
        }
        if let Some(let_) = self.let_ {
            doc.insert("let", let_);
        }
        let mut lookup = Document::new();
        lookup.insert(Self::NAME, doc);
        lookup
    }
}

impl Into<Stage> for Lookup {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
