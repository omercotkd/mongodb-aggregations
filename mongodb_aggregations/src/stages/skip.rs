use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(internal_impl = true, document_field = "amount")]
pub struct Skip {
    amount: i32,
}
