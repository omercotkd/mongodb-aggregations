use super::{PipelineStage, Stage};
use bson::{Bson, Document};

pub struct Project {
    pub fields: Document,
}

impl PipelineStage for Project {
    const NAME: &'static str = "$project";
}

impl Project {
    pub fn new(fields: impl Into<Document>) -> Self {
        Project {
            fields: fields.into(),
        }
    }

    pub fn retain_fields(
        &mut self,
        fields: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        for field in fields {
            self.fields.insert(field, 1);
        }

        self
    }

    pub fn remove_id(&mut self) -> &mut Self {
        self.fields.insert("_id", 0);
        self
    }

    pub fn add_field(&mut self, field: impl Into<String>, value: impl Into<Bson>) -> &mut Self {
        self.fields.insert(field, value);
        self
    }
}

impl Into<Document> for Project {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert(Self::NAME, self.fields);
        doc
    }
}

impl<'a> Into<Stage> for Project {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
