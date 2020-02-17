#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Related artifacts such as additional documentation, justification, or
/// bibliographic references.

#[derive(Debug)]
pub struct RelatedArtifact<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RelatedArtifact<'_> {
    pub fn new(value: &Value) -> RelatedArtifact {
        RelatedArtifact {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for citation
    pub fn _citation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_citation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for label
    pub fn _label(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_label") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A bibliographic citation for the related artifact. This text SHOULD be formatted
    /// according to an accepted citation format.
    pub fn citation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("citation") {
            return Some(string);
        }
        return None;
    }

    /// A brief description of the document or knowledge resource being referenced,
    /// suitable for display to a consumer.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
            return Some(string);
        }
        return None;
    }

    /// The document being referenced, represented as an attachment. This is exclusive
    /// with the resource element.
    pub fn document(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("document") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A short label that can be used to reference the citation from elsewhere in the
    /// containing artifact, such as a footnote index.
    pub fn label(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("label") {
            return Some(string);
        }
        return None;
    }

    /// The related resource, such as a library, value set, profile, or other knowledge
    /// resource.
    pub fn resource(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resource") {
            return Some(string);
        }
        return None;
    }

    /// The type of relationship to the related artifact.
    pub fn fhir_type(&self) -> Option<RelatedArtifactType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(RelatedArtifactType::from_string(&val).unwrap());
        }
        return None;
    }

    /// A url for the artifact that can be followed to access the actual content.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._citation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._label() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.citation() {}
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.document() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.label() {}
        if let Some(_val) = self.resource() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.url() {}
        return true;
    }
}

#[derive(Debug)]
pub struct RelatedArtifactBuilder {
    pub(crate) value: Value,
}

impl RelatedArtifactBuilder {
    pub fn build(&self) -> RelatedArtifact {
        RelatedArtifact {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RelatedArtifact) -> RelatedArtifactBuilder {
        RelatedArtifactBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RelatedArtifactBuilder {
        let mut __value: Value = json!({});
        return RelatedArtifactBuilder { value: __value };
    }

    pub fn _citation<'a>(&'a mut self, val: Element) -> &'a mut RelatedArtifactBuilder {
        self.value["_citation"] = json!(val.value);
        return self;
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut RelatedArtifactBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _label<'a>(&'a mut self, val: Element) -> &'a mut RelatedArtifactBuilder {
        self.value["_label"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut RelatedArtifactBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut RelatedArtifactBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn citation<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["citation"] = json!(val);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn document<'a>(&'a mut self, val: Attachment) -> &'a mut RelatedArtifactBuilder {
        self.value["document"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut RelatedArtifactBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn label<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["label"] = json!(val);
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["resource"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: RelatedArtifactType) -> &'a mut RelatedArtifactBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut RelatedArtifactBuilder {
        self.value["url"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum RelatedArtifactType {
    Documentation,
    Justification,
    Citation,
    Predecessor,
    Successor,
    DerivedFrom,
    DependsOn,
    ComposedOf,
}

impl RelatedArtifactType {
    pub fn from_string(string: &str) -> Option<RelatedArtifactType> {
        match string {
            "documentation" => Some(RelatedArtifactType::Documentation),
            "justification" => Some(RelatedArtifactType::Justification),
            "citation" => Some(RelatedArtifactType::Citation),
            "predecessor" => Some(RelatedArtifactType::Predecessor),
            "successor" => Some(RelatedArtifactType::Successor),
            "derived-from" => Some(RelatedArtifactType::DerivedFrom),
            "depends-on" => Some(RelatedArtifactType::DependsOn),
            "composed-of" => Some(RelatedArtifactType::ComposedOf),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RelatedArtifactType::Documentation => "documentation".to_string(),
            RelatedArtifactType::Justification => "justification".to_string(),
            RelatedArtifactType::Citation => "citation".to_string(),
            RelatedArtifactType::Predecessor => "predecessor".to_string(),
            RelatedArtifactType::Successor => "successor".to_string(),
            RelatedArtifactType::DerivedFrom => "derived-from".to_string(),
            RelatedArtifactType::DependsOn => "depends-on".to_string(),
            RelatedArtifactType::ComposedOf => "composed-of".to_string(),
        }
    }
}
