#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Filter::TerminologyCapabilities_Filter;
use serde_json::value::Value;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_Version<'a> {
    pub value: &'a Value,
}

impl TerminologyCapabilities_Version<'_> {
    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If this is the default version for this code system.
    pub fn is_default(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDefault") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// If the compositional grammar defined by the code system is supported.
    pub fn compositional(&self) -> Option<bool> {
        if let Some(val) = self.value.get("compositional") {
            return Some(val.as_bool().unwrap());
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_language") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// Language Displays supported.
    pub fn language(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("language") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Properties supported for $lookup.
    pub fn property(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// For version-less code systems, there should be a single version with no
    /// identifier.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for property
    pub fn _property(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_property") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// Extensions for isDefault
    pub fn _is_default(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDefault") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for compositional
    pub fn _compositional(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compositional") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Filter Properties supported.
    pub fn filter(&self) -> Option<Vec<TerminologyCapabilities_Filter>> {
        if let Some(Value::Array(val)) = self.value.get("filter") {
            return Some(
                val.into_iter()
                    .map(|e| TerminologyCapabilities_Filter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.is_default() {}
        if let Some(_val) = self.compositional() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._language() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.property() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self._property() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._is_default() {
            _val.validate();
        }
        if let Some(_val) = self._compositional() {
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
