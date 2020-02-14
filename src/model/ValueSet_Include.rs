#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ValueSet_Concept::ValueSet_Concept;
use crate::model::ValueSet_Filter::ValueSet_Filter;
use serde_json::value::Value;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Include<'a> {
    pub value: &'a Value,
}

impl ValueSet_Include<'_> {
    /// An absolute URI which is the code system from which the selected codes come
    /// from.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
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

    /// The version of the code system that the codes are selected from, or the special
    /// version '*' for all versions.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// Selects the concepts found in this value set (based on its value set
    /// definition). This is an absolute URI that is a reference to ValueSet.url.  If
    /// multiple value sets are specified this includes the union of the contents of all
    /// of the referenced value sets.
    pub fn value_set(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("valueSet") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a concept to be included or excluded.
    pub fn concept(&self) -> Option<Vec<ValueSet_Concept>> {
        if let Some(Value::Array(val)) = self.value.get("concept") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Concept { value: e })
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Select concepts by specify a matching criterion based on the properties
    /// (including relationships) defined by the system, or on filters defined by the
    /// system. If multiple filters are specified, they SHALL all be true.
    pub fn filter(&self) -> Option<Vec<ValueSet_Filter>> {
        if let Some(Value::Array(val)) = self.value.get("filter") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Filter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.value_set() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.concept() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._system() {
            _val.validate();
        }
        if let Some(_val) = self.filter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
