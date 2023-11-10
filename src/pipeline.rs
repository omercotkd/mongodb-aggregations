use crate::stages::*;
use bson::{Bson, Document};

pub struct Pipeline {
    pub stages: Vec<Document>,
}

pub struct PipelineBuilder {
    pub stages: Vec<Stage>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { stages: Vec::new() }
    }

    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }

    pub fn into_pipeline(self) -> Vec<Document> {
        self.stages
    }
}

impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder { stages: Vec::new() }
    }

    pub fn add_stage<T>(&mut self, stage: T)
    where
        T: Into<Stage>,
    {
        self.stages.push(stage.into());
    }

    pub fn build(self) -> Pipeline {
        let mut stages = Vec::with_capacity(self.stages.len());

        let length = self.stages.len();

        for (pos, e) in self.stages.into_iter().enumerate() {
            if e.location.is_first() && pos != 0 {
                panic!("First stage must be first");
            }
            if e.location.is_last() && pos != length - 1 {
                panic!("Last stage must be last");
            }

            stages.push(e.doc);
        }

        Pipeline { stages }
    }

    #[allow(non_snake_case)]
    pub fn addFields(&mut self, fields: Document) -> &mut Self {
        self.add_stage(AddFields::new(Some(fields)));
        self
    }

    pub fn count(&mut self, field: &str) -> &mut Self {
        self.add_stage(Count::new(field));
        self
    }

    pub fn group(&mut self, id: impl Into<Bson>, fields: impl Into<Document>) -> &mut Self {
        self.add_stage(Group::new(id, fields));
        self
    }

    pub fn project(&mut self, fields: impl Into<Document>) -> &mut Self {
        self.add_stage(Project::new(fields));
        self
    }
}

impl Into<Vec<Document>> for Pipeline {
    fn into(self) -> Vec<Document> {
        self.stages
    }
}
