#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Related artifacts such as additional documentation, justification, or
/// bibliographic references.

#[derive(Debug)]
pub struct RelatedArtifact<'a> {
    pub value: &'a Value,
}

impl RelatedArtifact<'_> {
    /// A url for the artifact that can be followed to access the actual content.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The document being referenced, represented as an attachment. This is exclusive
    /// with the resource element.
    pub fn document(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("document") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Extensions for label
    pub fn _label(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_label") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
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

    /// Extensions for citation
    pub fn _citation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_citation") {
            return Some(Element { value: val });
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

    /// A brief description of the document or knowledge resource being referenced,
    /// suitable for display to a consumer.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.document() {
            _val.validate();
        }
        if let Some(_val) = self._label() {
            _val.validate();
        }
        if let Some(_val) = self._display() {
            _val.validate();
        }
        if let Some(_val) = self.citation() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.resource() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.label() {}
        if let Some(_val) = self._citation() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.display() {}
        return true;
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
}
