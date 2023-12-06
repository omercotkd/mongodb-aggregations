use bson::{doc, Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(internal_impl = true)]
pub struct Unset {}

