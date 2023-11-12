use super::{PipelineStage, Stage};
use bson::{Bson, Document};

pub struct Bucket {
    pub group_by: Bson,
    pub boundaries: Vec<Bson>,
    pub default: Option<String>,
    pub output: Option<Document>,
}

pub struct BucketAuto {
    pub group_by: Bson,
    pub buckets: i32,
    pub granularity: Option<String>,
    pub output: Option<Document>,
}

impl PipelineStage for Bucket {
    const NAME: &'static str = "$bucket";
}

impl Bucket {
    pub fn new<IB, IS, ID>(
        group_by: IB,
        boundaries: impl IntoIterator<Item = IB>,
        default: Option<IS>,
        output: Option<ID>,
    ) -> Self
    where
        IB: Into<Bson>,
        IS: ToString,
        ID: Into<Document>,
    {
        Bucket {
            group_by: group_by.into(),
            boundaries: boundaries.into_iter().map(|b| b.into()).collect(),
            default: default.map(|d| d.to_string()),
            output: output.map(|o| o.into()),
        }
    }

    pub fn add_output_field(
        &mut self,
        field: impl Into<String>,
        expression: impl Into<Bson>,
    ) -> &mut Self {
        if let Some(output) = &mut self.output {
            output.insert(field, expression);
        } else {
            let mut output = Document::new();
            output.insert(field, expression);
            self.output = Some(output);
        }

        self
    }

    pub fn add_output_fields(&mut self, fields: Document) -> &mut Self {
        if let Some(output) = &mut self.output {
            output.extend(fields);
        } else {
            self.output = Some(fields);
        }

        self
    }
}

impl Into<Document> for Bucket {
    fn into(self) -> Document {
        let mut doc = Document::new();

        doc.insert("groupBy", self.group_by);
        doc.insert("boundaries", self.boundaries);
        if let Some(default) = self.default {
            doc.insert("default", default);
        }
        if let Some(output) = self.output {
            doc.insert("output", output);
        }

        let mut bucket = Document::new();

        bucket.insert(Self::NAME, doc);

        bucket
    }
}

impl Into<Stage> for Bucket {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}

impl PipelineStage for BucketAuto {
    const NAME: &'static str = "$bucketAuto";
}

impl BucketAuto {
    pub fn new<IB, IS, ID>(
        group_by: IB,
        boundaries: impl IntoIterator<Item = IB>,
        default: Option<IS>,
        output: Option<ID>,
    ) -> Self
    where
        IB: Into<Bson>,
        IS: ToString,
        ID: Into<Document>,
    {
        Bucket {
            group_by: group_by.into(),
            boundaries: boundaries.into_iter().map(|b| b.into()).collect(),
            default: default.map(|d| d.to_string()),
            output: output.map(|o| o.into()),
        }
    }

    pub fn add_output_field(
        &mut self,
        field: impl Into<String>,
        expression: impl Into<Bson>,
    ) -> &mut Self {
        if let Some(output) = &mut self.output {
            output.insert(field, expression);
        } else {
            let mut output = Document::new();
            output.insert(field, expression);
            self.output = Some(output);
        }

        self
    }

    pub fn add_output_fields(&mut self, fields: Document) -> &mut Self {
        if let Some(output) = &mut self.output {
            output.extend(fields);
        } else {
            self.output = Some(fields);
        }

        self
    } 
}

impl Into<Document> for BucketAuto {
    fn into(self) -> Document {
        todo!()
    }
}

impl Into<Stage> for BucketAuto {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
