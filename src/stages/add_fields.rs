use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct AddFields {
    fields: Document,
}

impl PipelineStage for AddFields {
    const NAME: &'static str = "$addFields";
    const LOCATION: StageLocation = StageLocation::Any;
}

impl AddFields {
    pub fn new<ID>(fields: ID) -> Self
    where
        ID: Into<Document>,
    {
        AddFields {
            fields: fields.into(),
        }
    }

    pub fn fields(&self) -> &Document {
        &self.fields
    }
}

impl AddFieldsBuilder {
    pub fn add_field<IS, IB>(&mut self, key: &str, value: Bson) -> &mut Self
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        self.fields
            .get_or_insert_with(Document::new)
            .insert(key, value);
        self
    }

    pub fn add_fields<ID>(&mut self, fields: Document) -> &mut Self
    where
        ID: Into<Document>,
    {
        self.fields.get_or_insert_with(Document::new).extend(fields);
        self
    }
}

impl Into<Document> for AddFields {
    fn into(self) -> Document {
        doc! {
            Self::NAME: self.fields
        }
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
