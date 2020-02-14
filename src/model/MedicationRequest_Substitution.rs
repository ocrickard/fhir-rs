#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest_Substitution<'a> {
    pub value: &'a Value,
}

impl MedicationRequest_Substitution<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub fn allowed_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allowedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for allowedBoolean
    pub fn _allowed_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedBoolean") {
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

    /// True if the prescriber allows a different drug to be dispensed from what was
    /// prescribed.
    pub fn allowed_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("allowedCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates the reason for the substitution, or why substitution must or must not
    /// be performed.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.allowed_boolean() {}
        if let Some(_val) = self._allowed_boolean() {
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
        if let Some(_val) = self.allowed_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.reason() {
            _val.validate();
        }
        return true;
    }
}
