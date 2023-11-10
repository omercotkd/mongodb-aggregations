use super::{PipelineStage, Stage};
use bson::{Bson, Document};

pub struct AddFields {
    pub fields: Document,
}

impl PipelineStage for AddFields {
    const NAME: &'static str = "$addFields";
}

impl AddFields {
    pub fn new(fields: Option<Document>) -> Self {
        AddFields {
            fields: fields.unwrap_or_default(),
        }
    }

    pub fn add_field(&mut self, name: &str, value: impl Into<Bson>) {
        self.fields.insert(name, value);
    }

    pub fn add_fields(&mut self, fields: Document) {
        self.fields.extend(fields);
    }
}

impl Into<Document> for AddFields {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.fields);
        doc
    }
}

impl Into<Stage> for AddFields {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}

impl Default for AddFields {
    fn default() -> Self {
        AddFields::new(None)
    }
}
