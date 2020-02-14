#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).

#[derive(Debug)]
pub struct OperationDefinition_ReferencedFrom<'a> {
    pub value: &'a Value,
}

impl OperationDefinition_ReferencedFrom<'_> {
    /// Extensions for source
    pub fn _source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_source") {
            return Some(Element { value: val });
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

    /// The name of the parameter or dot-separated path of parameter names pointing to
    /// the resource parameter that is expected to contain a reference to this resource.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// The id of the element in the referencing resource that is expected to resolve to
    /// this resource.
    pub fn source_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceId") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for sourceId
    pub fn _source_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceId") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._source() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.source_id() {}
        if let Some(_val) = self._source_id() {
            _val.validate();
        }
        return true;
    }
}
