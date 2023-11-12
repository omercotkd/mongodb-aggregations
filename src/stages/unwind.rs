use super::{PipelineStage, Stage};
use bson::Document;

pub struct Unwind {
    pub path: String,
    pub preserve_null_and_empty_arrays: bool,
    pub include_array_index: Option<String>,
}

impl PipelineStage for Unwind {
    const NAME: &'static str = "$unwind";
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

impl Into<Document> for Unwind {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.path);
        doc
    }
}

impl Into<Stage> for Unwind {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
