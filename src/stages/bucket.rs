use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

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
    const LOCATION: StageLocation = StageLocation::Any;
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

    pub fn add_output_field<IS, IB>(&mut self, field: IS, expression: IB) -> &mut Self
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        if let Some(output) = &mut self.output {
            output.insert(field, expression);
        } else {
            let mut output = Document::new();
            output.insert(field, expression);
            self.output = Some(output);
        }

        self
    }

    pub fn add_output_fields<ID>(&mut self, fields: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        if let Some(output) = &mut self.output {
            output.extend(fields.into());
        } else {
            self.output = Some(fields.into());
        }

        self
    }
}

impl Into<Document> for Bucket {
    fn into(self) -> Document {
        let mut fields = doc! {
            "groupBy": self.group_by,
            "boundaries": self.boundaries
        };

        if let Some(default) = self.default {
            fields.insert("default", default);
        }
        if let Some(output) = self.output {
            fields.insert("output", output);
        }

        doc! {
            Self::NAME: fields
        }
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
    const LOCATION: StageLocation = StageLocation::Any;
}

impl BucketAuto {
    pub fn new<IB, IS, ID>(
        group_by: IB,
        buckets: i32,
        granularity: Option<IS>,
        output: Option<ID>,
    ) -> Self
    where
        IB: Into<Bson>,
        IS: Into<String>,
        ID: Into<Document>,
    {
        BucketAuto {
            group_by: group_by.into(),
            buckets,
            granularity: granularity.map(|g| g.into()),
            output: output.map(|o| o.into()),
        }
    }

    pub fn add_output_field<IS, IB>(&mut self, field: IS, expression: IB) -> &mut Self
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        if let Some(output) = &mut self.output {
            output.insert(field, expression);
        } else {
            let mut output = Document::new();
            output.insert(field, expression);
            self.output = Some(output);
        }

        self
    }

    pub fn add_output_fields<ID>(&mut self, fields: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        if let Some(output) = &mut self.output {
            output.extend(fields.into());
        } else {
            self.output = Some(fields.into());
        }

        self
    }
}

impl Into<Document> for BucketAuto {
    fn into(self) -> Document {
        let mut fields = doc! {
            "groupBy": self.group_by,
            "buckets": self.buckets
        };

        if let Some(granularity) = self.granularity {
            fields.insert("granularity", granularity);
        };
        if let Some(output) = self.output {
            fields.insert("output", output);
        }

        doc! {
            Self::NAME: fields
        }
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
