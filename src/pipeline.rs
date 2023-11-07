use crate::stages::{PipelineStage, Stage};
use bson::Document;

pub struct Pipeline {
    pub stages: Vec<Stage>,
}

impl Pipeline {
    pub fn builder() -> Self {
        Pipeline { stages: Vec::new() }
    }

    pub fn add_stage<T>(&mut self, stage: T)
    where
        T: Into<Stage>,
    {
        self.stages.push(stage.into());
    }

    pub fn build(self) -> Vec<Document> {
        let mut pipeline = Vec::with_capacity(self.stages.len());

        let length = self.stages.len();

        for (pos, e) in self.stages.into_iter().enumerate() {
            if e.location.is_first() && pos != 0 {
                panic!("First stage must be first");
            }
            if e.location.is_last() && pos != length - 1 {
                panic!("Last stage must be last");
            }

            pipeline.push(e.doc);
        }

        pipeline
    }
}
