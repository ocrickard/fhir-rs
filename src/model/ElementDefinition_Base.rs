#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Base<'a> {
    pub value: &'a Value,
}

impl ElementDefinition_Base<'_> {
    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Minimum cardinality of the base element identified by the path.
    pub fn min(&self) -> Option<u64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_u64().unwrap());
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Maximum cardinality of the base element identified by the path.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The Path that identifies the base element - this matches the
    /// ElementDefinition.path for that element. Across FHIR, there is only one base
    /// definition of any element - that is, an element definition on a
    /// [[[StructureDefinition]]] without a StructureDefinition.base.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
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

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._min() {
            _val.validate();
        }
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self._max() {
            _val.validate();
        }
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._path() {
            _val.validate();
        }
        return true;
    }
}
