use super::{PipelineStage, Stage, StageLocation};
use crate::Pipeline;
use bson::Document;

pub struct Facet {
    fields: Vec<(String, Pipeline)>,
}

impl PipelineStage for Facet {
    const NAME: &'static str = "$facet";
    const LOCATION: StageLocation = StageLocation::Any;
}

impl Facet {
    pub fn new(fields: Vec<(String, Pipeline)>) -> Self {
        Facet { fields }
    }

    pub fn add_field<IS, IP>(&mut self, name: IS, pipeline: IP) -> &mut Self
    where
        IS: Into<String>,
        IP: Into<Pipeline>,
    {
        self.fields.push((name.into(), pipeline.into()));
        self
    }
}

impl Into<Document> for Facet {
    fn into(self) -> Document {
        let mut doc = Document::new();
        for (name, pipeline) in self.fields {
            doc.insert(name, pipeline);
        }
        doc
    }
}

impl Into<Stage> for Facet {
    fn into(self) -> Stage {
        let mut doc = Document::new();
        for (name, pipeline) in self.fields {
            for stage in pipeline.stages.iter() {
                if stage.contains_key("$out") {
                    panic!("$facet cannot contain $out stage");
                } else if stage.contains_key("$merge") {
                    panic!("$facet cannot contain $merge stage");
                }
                todo!("check for other stages that cannot be in $facet");
            }
            doc.insert(name, pipeline);
        }

        Stage {
            location: Self::LOCATION,
            doc,
            name: Self::NAME,
        }
    }
}
