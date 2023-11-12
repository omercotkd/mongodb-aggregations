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

    pub fn lookup(
        &mut self,
        from: impl Into<String>,
        local_field: impl Into<String>,
        foreign_field: impl Into<String>,
        as_field: impl Into<String>,
        pipeline: Option<Vec<Document>>,
        let_: Option<Document>,
    ) -> &mut Self {
        self.add_stage(Lookup::new(
            from,
            local_field,
            foreign_field,
            as_field,
            pipeline,
            let_,
        ));
        self
    }

    pub fn project(&mut self, fields: impl Into<Document>) -> &mut Self {
        self.add_stage(Project::new(fields));
        self
    }

    pub fn unwind<'a>(
        &mut self,
        path: &'a str,
        preserve_null_and_empty_arrays: bool,
        include_array_index: Option<&'a str>,
    ) -> &mut Self {
        self.add_stage(Unwind::new(
            path,
            preserve_null_and_empty_arrays,
            include_array_index,
        ));
        self
    }
}

impl Into<Vec<Document>> for Pipeline {
    fn into(self) -> Vec<Document> {
        self.stages
    }
}
