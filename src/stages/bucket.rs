use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

pub struct Bucket {
    group_by: Bson,
    boundaries: Vec<Bson>,
    default: Option<Bson>,
    output: Option<Document>,
}

impl Bucket {
    pub fn new<IB, ID>(
        group_by: IB,
        boundaries: impl IntoIterator<Item = IB>,
        default: Option<IB>,
        output: Option<ID>,
    ) -> Self
    where
        IB: Into<Bson>,
        ID: Into<Document>,
    {
        Bucket {
            group_by: group_by.into(),
            boundaries: boundaries.into_iter().map(|b| b.into()).collect(),
            default: default.map(|d| d.into()),
            output: output.map(|o| o.into()),
        }
    }

    pub fn group_by(&self) -> &Bson {
        &self.group_by
    }

    pub fn boundaries(&self) -> &[Bson] {
        &self.boundaries
    }

    pub fn default(&self) -> Option<&Bson> {
        self.default.as_ref()
    }

    pub fn output(&self) -> Option<&Document> {
        self.output.as_ref()
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

    pub fn set_group_by<IB>(&mut self, group_by: IB) -> &mut Self
    where
        IB: Into<Bson>,
    {
        self.group_by = group_by.into();
        self
    }

    pub fn set_boundaries<IB>(&mut self, boundaries: impl IntoIterator<Item = IB>) -> &mut Self
    where
        IB: Into<Bson>,
    {
        self.boundaries = boundaries.into_iter().map(|b| b.into()).collect();
        self
    }

    pub fn set_default<IB>(&mut self, default: IB) -> &mut Self
    where
        IB: Into<Bson>,
    {
        self.default = Some(default.into());
        self
    }

    pub fn set_output<ID>(&mut self, output: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        self.output = Some(output.into());
        self
    }

    pub fn unset_default(&mut self) -> &mut Self {
        self.default = None;
        self
    }

    pub fn unset_output(&mut self) -> &mut Self {
        self.output = None;
        self
    }
}

impl PipelineStage for Bucket {
    const NAME: &'static str = "$bucket";
    const LOCATION: StageLocation = StageLocation::Any;
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

pub struct BucketAuto {
    group_by: Bson,
    buckets: i32,
    granularity: Option<Granularity>,
    output: Option<Document>,
}

pub enum Granularity {
    R5,
    R10,
    R20,
    R40,
    R80,
    _1_2_5,
    E6,
    E12,
    E24,
    E48,
    E96,
    E192,
    PowerOf2,
}

impl BucketAuto {
    pub fn new<IB, ID>(
        group_by: IB,
        buckets: i32,
        output: Option<ID>,
        granularity: Option<Granularity>,
    ) -> Self
    where
        IB: Into<Bson>,
        ID: Into<Document>,
    {
        BucketAuto {
            group_by: group_by.into(),
            buckets,
            granularity: granularity,
            output: output.map(|o| o.into()),
        }
    }

    pub fn group_by(&self) -> &Bson {
        &self.group_by
    }

    pub fn buckets(&self) -> i32 {
        self.buckets
    }

    pub fn granularity(&self) -> Option<&Granularity> {
        self.granularity.as_ref()
    }

    pub fn output(&self) -> Option<&Document> {
        self.output.as_ref()
    }

    pub fn add_output_field<IS, IB>(
        &mut self,
        field: IS,
        accumulator: IS,
        expression: IB,
    ) -> &mut Self
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        let mut value = Document::new();

        value.insert(accumulator, expression);

        if let Some(output) = &mut self.output {
            output.insert(field, value);
        } else {
            let mut output = Document::new();
            output.insert(field, value);
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

    pub fn set_group_by<IB>(&mut self, group_by: IB) -> &mut Self
    where
        IB: Into<Bson>,
    {
        self.group_by = group_by.into();
        self
    }

    pub fn set_buckets(&mut self, buckets: i32) -> &mut Self {
        self.buckets = buckets;
        self
    }

    pub fn set_granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.granularity = Some(granularity);
        self
    }

    pub fn set_output<ID>(&mut self, output: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        self.output = Some(output.into());
        self
    }

    pub fn unset_granularity(&mut self) -> &mut Self {
        self.granularity = None;
        self
    }

    pub fn unset_output(&mut self) -> &mut Self {
        self.output = None;
        self
    }
}

impl PipelineStage for BucketAuto {
    const NAME: &'static str = "$bucketAuto";
    const LOCATION: StageLocation = StageLocation::Any;
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
        if self.buckets < 1 {
            todo!("add errorbuckets must be greater than 0");
        }
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}

impl Into<Bson> for Granularity {
    fn into(self) -> Bson {
        match self {
            Granularity::R5 => Bson::String("R5".to_string()),
            Granularity::R10 => Bson::String("R10".to_string()),
            Granularity::R20 => Bson::String("R20".to_string()),
            Granularity::R40 => Bson::String("R40".to_string()),
            Granularity::R80 => Bson::String("R80".to_string()),
            Granularity::_1_2_5 => Bson::String("1-2-5".to_string()),
            Granularity::E6 => Bson::String("E6".to_string()),
            Granularity::E12 => Bson::String("E12".to_string()),
            Granularity::E24 => Bson::String("E24".to_string()),
            Granularity::E48 => Bson::String("E48".to_string()),
            Granularity::E96 => Bson::String("E96".to_string()),
            Granularity::E192 => Bson::String("E192".to_string()),
            Granularity::PowerOf2 => Bson::String("POWERSOF2".to_string()),
        }
    }
}
