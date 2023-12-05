use bson::{Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(location = "first", into_document = true)]
pub struct CollStats {
    #[builder(setter(strip_option), default = "None")]
    latency_stats: Option<CollLatencyStats>,
}


impl CollStats {
    pub fn new() -> Self {
        todo!()
    }
}


#[derive(Debug, Builder, Default, Clone)]
pub struct CollLatencyStats {
    #[builder(setter(strip_option), default = "None")]
    histogram: Option<bool>,
}

impl CollLatencyStats {
    pub fn empty() -> Self {
        CollLatencyStats::default()
    }
}

impl Into<Bson> for CollLatencyStats {
    fn into(self) -> Bson {
        let mut doc = Document::new();
        if let Some(histogram) = self.histogram {
            doc.insert("histogram", histogram);
        }
        doc.into()
    }
}


#[derive(Debug, Clone, Builder, Default)]
pub struct CollStorageStats {
    #[builder(setter(strip_option), default = "None")]
    scale: Option<i32>,
} 

impl CollStorageStats {
    pub fn empty() -> Self {
        CollStorageStats::default()
    }
}


impl Into<Bson> for CollStorageStats {
    fn into(self) -> Bson {
        let mut doc = Document::new();
        if let Some(scale) = self.scale {
            doc.insert("scale", scale);
        }
        doc.into()
    }
}
