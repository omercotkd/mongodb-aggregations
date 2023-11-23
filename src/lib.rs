mod pipeline;
mod errors;
pub mod stages;

pub use pipeline::{Pipeline, PipelineBuilder};

// TODO add features that are per mongo db feature