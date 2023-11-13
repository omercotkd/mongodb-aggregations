use super::{PipelineStage, Stage};
use bson::{doc, Bson, Document};

pub struct AddFields {
    pub fields: Document,
}

impl PipelineStage for AddFields {
    const NAME: &'static str = "$addFields";
}

impl AddFields {
    pub fn new<ID>(fields: Option<ID>) -> Self
    where
        ID: Into<Document>,
    {
        AddFields {
            fields: fields.map(|f| f.into()).unwrap_or_default(),
        }
    }

    pub fn add_field<IS, IB>(&mut self, name: IS, value: IB)
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        self.fields.insert(name, value);
    }

    pub fn add_fields<ID>(&mut self, fields: ID)
    where
        ID: Into<Document>,
    {
        self.fields.extend(fields.into());
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

impl Default for AddFields {
    fn default() -> Self {
        AddFields::new(None::<Document>)
    }
}
