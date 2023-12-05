use bson::{doc, Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(location = "first", into_document = true)]
pub struct Densify {
    field: String,
    partition_by_fields: Option<Vec<String>>,
    range: Range,
}

#[derive(Debug, Builder, Clone)]
pub struct Range {
    bounds: RangeBounds,
    step: RangeStep,
    unit: Option<RangeUnit>,
}

impl Into<Bson> for Range {
    fn into(self) -> Bson {
        let mut doc = Document::new();
        doc.insert("bounds", self.bounds);
        doc.insert("step", self.step);
        if let Some(unit) = self.unit {
            doc.insert("unit", unit);
        }
        doc.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RangeBounds {
    Array([i32; 2]),
    Full,
    Partition,
}

impl Into<Bson> for RangeBounds {
    fn into(self) -> Bson {
        match self {
            RangeBounds::Array([min, max]) => vec![min, max].into(),
            RangeBounds::Full => "full".into(),
            RangeBounds::Partition => "partition".into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RangeStep {
    Int(i32),
    Number(f32),
}

impl Into<Bson> for RangeStep {
    fn into(self) -> Bson {
        match self {
            RangeStep::Int(step) => step.into(),
            RangeStep::Number(step) => step.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RangeUnit {
    Milliseconds,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl Into<Bson> for RangeUnit {
    fn into(self) -> Bson {
        match self {
            RangeUnit::Milliseconds => "milliseconds".into(),
            RangeUnit::Second => "second".into(),
            RangeUnit::Minute => "minute".into(),
            RangeUnit::Hour => "hour".into(),
            RangeUnit::Day => "day".into(),
            RangeUnit::Week => "week".into(),
            RangeUnit::Month => "month".into(),
            RangeUnit::Quarter => "quarter".into(),
            RangeUnit::Year => "year".into(),
        }
    }
}
