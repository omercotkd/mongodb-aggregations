// Atlas only

use bson::{doc, Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(into_document = true, stage_location = "first")]
pub struct Search {}


#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(into_document = true)]
pub struct SearchMeta {}