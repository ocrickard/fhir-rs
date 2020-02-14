#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_Material<'a> {
    pub value: &'a Value,
}

impl DeviceDefinition_Material<'_> {
    /// The substance.
    pub fn substance(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["substance"],
        }
    }

    /// Extensions for allergenicIndicator
    pub fn _allergenic_indicator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allergenicIndicator") {
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

    /// Indicates an alternative material of the device.
    pub fn alternate(&self) -> Option<bool> {
        if let Some(val) = self.value.get("alternate") {
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

    /// Whether the substance is a known or suspected allergen.
    pub fn allergenic_indicator(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allergenicIndicator") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for alternate
    pub fn _alternate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_alternate") {
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

    pub fn validate(&self) -> bool {
        let _ = self.substance().validate();
        if let Some(_val) = self._allergenic_indicator() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.alternate() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.allergenic_indicator() {}
        if let Some(_val) = self._alternate() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
