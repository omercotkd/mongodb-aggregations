use super::{PipelineStage, Stage};
use bson::Document;

// version 5.1+


pub struct Densify {
    field: String,
    partition_by_fields: Option<Vec<String>>,
    range: Range,
}

struct Range {
    bounds: RangeBounds,
    step: RangeStep,
    unit: Option<RangeUnit>
}

enum RangeBounds {
    Array([i32; 2]),
    Full,
    Partition
}

enum RangeStep {
    Int(i32),
    Number(f32),
}

enum RangeUnit {
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
