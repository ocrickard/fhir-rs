#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_Dosage::MedicationKnowledge_Dosage;
use crate::model::MedicationKnowledge_PatientCharacteristics::MedicationKnowledge_PatientCharacteristics;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_AdministrationGuidelines<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_AdministrationGuidelines<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_AdministrationGuidelines {
        MedicationKnowledge_AdministrationGuidelines {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Dosage for the medication for the specific guidelines.
    pub fn dosage(&self) -> Option<Vec<MedicationKnowledge_Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosage") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Dosage {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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

    /// Indication for use that apply to the specific administration guidelines.
    pub fn indication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("indicationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indication for use that apply to the specific administration guidelines.
    pub fn indication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("indicationReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| MedicationKnowledge_PatientCharacteristics {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.dosage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.indication_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.indication_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.patient_characteristics() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_AdministrationGuidelinesBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_AdministrationGuidelinesBuilder {
    pub fn build(&self) -> MedicationKnowledge_AdministrationGuidelines {
        MedicationKnowledge_AdministrationGuidelines {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_AdministrationGuidelines,
    ) -> MedicationKnowledge_AdministrationGuidelinesBuilder {
        MedicationKnowledge_AdministrationGuidelinesBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_AdministrationGuidelinesBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_AdministrationGuidelinesBuilder { value: __value };
    }

    pub fn dosage<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Dosage>,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["dosage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn indication_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["indicationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn indication_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["indicationReference"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn patient_characteristics<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_PatientCharacteristics>,
    ) -> &'a mut MedicationKnowledge_AdministrationGuidelinesBuilder {
        self.value["patientCharacteristics"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
