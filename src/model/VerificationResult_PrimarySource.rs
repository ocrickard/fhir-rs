#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult_PrimarySource<'a> {
    pub value: &'a Value,
}

impl VerificationResult_PrimarySource<'_> {
    /// Type of primary source (License Board; Primary Education; Continuing Education;
    /// Postal Service; Relationship owner; Registration Authority; legal source;
    /// issuing source; authoritative source).
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Method for communicating with the primary source (manual; API; Push).
    pub fn communication_method(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("communicationMethod") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Status of the validation of the target against the primary source (successful;
    /// failed; unknown).
    pub fn validation_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("validationStatus") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Ability of the primary source to push updates/alerts (yes; no; undetermined).
    pub fn can_push_updates(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("canPushUpdates") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Reference to the primary source.
    pub fn who(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("who") {
            return Some(Reference { value: val });
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

    /// Type of alerts/updates the primary source can send (specific requested changes;
    /// any changes; as defined by source).
    pub fn push_type_available(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("pushTypeAvailable") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// Extensions for validationDate
    pub fn _validation_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_validationDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// When the target was validated against the primary source.
    pub fn validation_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("validationDate") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.communication_method() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.validation_status() {
            _val.validate();
        }
        if let Some(_val) = self.can_push_updates() {
            _val.validate();
        }
        if let Some(_val) = self.who() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.push_type_available() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._validation_date() {
            _val.validate();
        }
        if let Some(_val) = self.validation_date() {}
        if let Some(_val) = self.id() {}
        return true;
    }
}
