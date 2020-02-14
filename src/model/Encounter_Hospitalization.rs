#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_Hospitalization<'a> {
    pub value: &'a Value,
}

impl Encounter_Hospitalization<'_> {
    /// The location/organization from which the patient came before admission.
    pub fn origin(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("origin") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Pre-admission identifier.
    pub fn pre_admission_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("preAdmissionIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// From where patient was admitted (physician referral, transfer).
    pub fn admit_source(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("admitSource") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Special courtesies (VIP, board member).
    pub fn special_courtesy(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialCourtesy") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Category or kind of location after discharge.
    pub fn discharge_disposition(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("dischargeDisposition") {
            return Some(CodeableConcept { value: val });
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

    /// Whether this hospitalization is a readmission and why if known.
    pub fn re_admission(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reAdmission") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Diet preferences reported by the patient.
    pub fn diet_preference(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("dietPreference") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Location/organization to which the patient is discharged.
    pub fn destination(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("destination") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Any special requests that have been made for this hospitalization encounter,
    /// such as the provision of specific equipment or other things.
    pub fn special_arrangement(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialArrangement") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.origin() {
            _val.validate();
        }
        if let Some(_val) = self.pre_admission_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.admit_source() {
            _val.validate();
        }
        if let Some(_val) = self.special_courtesy() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.discharge_disposition() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.re_admission() {
            _val.validate();
        }
        if let Some(_val) = self.diet_preference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.destination() {
            _val.validate();
        }
        if let Some(_val) = self.special_arrangement() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
