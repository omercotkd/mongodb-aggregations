use super::{PipelineStage, Stage, StageLocation};
use bson::{doc, Bson, Document};

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

    pub fn add_field<IS, IB>(&mut self, name: IS, value: IB) -> &mut Self
    where
        IS: Into<String>,
        IB: Into<Bson>,
    {
        self.fields.insert(name, value);
        self
    }

    pub fn add_fields<ID>(&mut self, fields: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        self.fields.extend(fields.into());
        self
    }

    pub fn set_fields<ID>(&mut self, fields: ID) -> &mut Self
    where
        ID: Into<Document>,
    {
        self.fields = fields.into();
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

impl Default for AddFields {
    fn default() -> Self {
        AddFields::new(Document::new())
    }
}
