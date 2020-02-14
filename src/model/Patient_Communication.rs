#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient_Communication<'a> {
    pub value: &'a Value,
}

impl Patient_Communication<'_> {
    /// The ISO-639-1 alpha 2 code in lower case for the language, optionally followed
    /// by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g.
    /// "en" for English, or "en-US" for American English versus "en-EN" for England
    /// English.
    pub fn language(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["language"],
        }
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

    /// Extensions for preferred
    pub fn _preferred(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preferred") {
            return Some(Element { value: val });
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

    /// Indicates whether or not the patient prefers this language (over other languages
    /// he masters up a certain level).
    pub fn preferred(&self) -> Option<bool> {
        if let Some(val) = self.value.get("preferred") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        let _ = self.language().validate();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._preferred() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.preferred() {}
        return true;
    }
}
