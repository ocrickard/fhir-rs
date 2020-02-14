#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ValueSet_Designation::ValueSet_Designation;
use serde_json::value::Value;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Contains<'a> {
    pub value: &'a Value,
}

impl ValueSet_Contains<'_> {
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

    /// Extensions for abstract
    pub fn _abstract(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_abstract") {
            return Some(Element { value: val });
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

    /// An absolute URI which is the code system in which the code for this item in the
    /// expansion is defined.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for inactive
    pub fn _inactive(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inactive") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The code for this item in the expansion hierarchy. If this code is missing the
    /// entry in the hierarchy is a place holder (abstract) and does not represent a
    /// valid code in the value set.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
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

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Additional representations for this item - other languages, aliases, specialized
    /// purposes, used for particular purposes, etc. These are relevant when the
    /// conditions of the expansion do not fix to a single correct representation.
    pub fn designation(&self) -> Option<Vec<ValueSet_Designation>> {
        if let Some(Value::Array(val)) = self.value.get("designation") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Designation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The version of the code system from this code was taken. Note that a well-
    /// maintained code system does not need the version reported, because the
    /// meaning of codes is consistent across versions. However this cannot consistently
    /// be assured, and when the meaning is not guaranteed to be consistent, the version
    /// SHOULD be exchanged.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// If true, this entry is included in the expansion for navigational purposes, and
    /// the user cannot select the code directly as a proper value.
    pub fn fhir_abstract(&self) -> Option<bool> {
        if let Some(val) = self.value.get("abstract") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// If the concept is inactive in the code system that defines it. Inactive codes
    /// are those that are no longer to be used, but are maintained by the code system
    /// for understanding legacy data. It might not be known or specified whether an
    /// concept is inactive (and it may depend on the context of use).
    pub fn inactive(&self) -> Option<bool> {
        if let Some(val) = self.value.get("inactive") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The recommended display for this item in the expansion.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// Other codes and entries contained under this entry in the hierarchy.
    pub fn contains(&self) -> Option<Vec<ValueSet_Contains>> {
        if let Some(Value::Array(val)) = self.value.get("contains") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Contains { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._abstract() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.system() {}
        if let Some(_val) = self._inactive() {
            _val.validate();
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self._system() {
            _val.validate();
        }
        if let Some(_val) = self._display() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.designation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.fhir_abstract() {}
        if let Some(_val) = self.inactive() {}
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contains() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
