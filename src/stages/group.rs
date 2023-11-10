use super::{PipelineStage, Stage};
use bson::{Bson, Document};

pub struct Group {
    pub id: Bson,
    pub fields: Document,
}

impl PipelineStage for Group {
    const NAME: &'static str = "$group";
}

impl Group {
    pub fn new(id: impl Into<Bson>, fields: impl Into<Document>) -> Self {
        Group {
            id: id.into(),
            fields: fields.into(),
        }
    }

    pub fn add_field(&mut self, name: &str, accumulator: &str, expression: impl Into<Bson>) {
        let mut doc = Document::new();

        doc.insert(accumulator, expression.into());

        self.fields.insert(name, doc);
    }

    pub fn add_fields(&mut self, fields: Document) {
        self.fields.extend(fields);
    }
}

impl Into<Document> for Group {
    fn into(self) -> Document {
        let mut doc = Document::new();
        doc.insert("_id", self.id);
        doc.extend(self.fields);
        let mut group = Document::new();
        group.insert(Self::NAME, doc);
        group
    }
}

impl Into<Stage> for Group {
    fn into(self) -> Stage {
        Stage {
            location: Self::LOCATION,
            doc: self.into(),
            name: Self::NAME,
        }
    }
}
