use bson::{doc, Bson, Document};
use mongodb_aggregations_derive::PipelineStage;

#[derive(Debug, Builder, Default, PipelineStage)]
#[builder(setter(into))]
#[pipeline_stage(internal_impl = true, document_field = "fields")]
pub struct AddFields {
    fields: Document,
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
