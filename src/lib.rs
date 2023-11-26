mod pipeline;
mod errors;
pub mod stages;

pub use pipeline::{Pipeline, PipelineBuilder};


#[macro_use]
extern crate derive_builder;
// TODO add features that are per mongo db feature