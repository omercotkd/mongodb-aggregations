use bson::doc;
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(internal_impl = true)]
pub struct Unwind {
    pub path: String,
    pub preserve_null_and_empty_arrays: bool,
    pub include_array_index: Option<String>,
}

impl Unwind {
    pub fn new<'a>(
        path: &'a str,
        preserve_null_and_empty_arrays: bool,
        include_array_index: Option<&'a str>,
    ) -> Self {
        let path = match path.starts_with('$') {
            true => path.to_string(),
            false => format!("${}", path),
        };

        Unwind {
            path,
            preserve_null_and_empty_arrays,
            include_array_index: include_array_index.map(|s| s.to_string()),
        }
    }
}
