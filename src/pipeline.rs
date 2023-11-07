use crate::stages::PipelineStage;
use bson::Document;

pub struct Pipeline {
    pub stages: Vec<Document>,
    names: Vec<&'static str>,
}

impl Pipeline {
    pub fn builder() -> Self {
        Pipeline { stages: Vec::new(), names: Vec::new() }
    }

    pub fn add_stage<T>(&mut self, stage: T)
    where
        T: PipelineStage,
    {
        self.names.push(T::NAME);
        self.stages.push(stage.into());
    }

    pub fn build(self) -> Vec<Document> {
        self.stages
    }
}
