#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.

#[derive(Debug)]
pub struct StructureDefinition_Mapping<'a> {
    pub value: &'a Value,
}

impl StructureDefinition_Mapping<'_> {
    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A name for the specification that is being mapped to.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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

    /// Extensions for uri
    pub fn _uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An Internal id that is used to identify this mapping set when specific mappings
    /// are made.
    pub fn identity(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identity") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Comments about this mapping, including version notes, issues, scope limitations,
    /// and other important notes for usage.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for identity
    pub fn _identity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identity") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An absolute URI that identifies the specification that this mapping is expressed
    /// to.
    pub fn uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uri") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._uri() {
            _val.validate();
        }
        if let Some(_val) = self.identity() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self._identity() {
            _val.validate();
        }
        if let Some(_val) = self.uri() {}
        return true;
    }
}
