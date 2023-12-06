use bson::Bson;
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(internal_impl = true, document_field = "expr")]
pub struct Documents {
    expr: Bson,
}
