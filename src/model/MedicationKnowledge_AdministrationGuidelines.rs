#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_Dosage::MedicationKnowledge_Dosage;
use crate::model::MedicationKnowledge_PatientCharacteristics::MedicationKnowledge_PatientCharacteristics;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_AdministrationGuidelines<'a> {
    pub value: &'a Value,
}

impl MedicationKnowledge_AdministrationGuidelines<'_> {
    /// Indication for use that apply to the specific administration guidelines.
    pub fn indication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("indicationCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Characteristics of the patient that are relevant to the administration
    /// guidelines (for example, height, weight, gender, etc.).
    pub fn patient_characteristics(
        &self,
    ) -> Option<Vec<MedicationKnowledge_PatientCharacteristics>> {
        if let Some(Value::Array(val)) = self.value.get("patientCharacteristics") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_PatientCharacteristics { value: e })
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

    /// Dosage for the medication for the specific guidelines.
    pub fn dosage(&self) -> Option<Vec<MedicationKnowledge_Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosage") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Dosage { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indication for use that apply to the specific administration guidelines.
    pub fn indication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("indicationReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.indication_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.patient_characteristics() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
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
        if let Some(_val) = self.dosage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.indication_reference() {
            _val.validate();
        }
        return true;
    }
}
