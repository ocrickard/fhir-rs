#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_Diagnosis<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Encounter_Diagnosis<'_> {
    pub fn new(value: &Value) -> Encounter_Diagnosis {
        Encounter_Diagnosis {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for rank
    pub fn _rank(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rank") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reason the encounter takes place, as specified using information from another
    /// resource. For admissions, this is the admission diagnosis. The indication will
    /// typically be a Condition (with other resources referenced in the
    /// evidence.detail), or a Procedure.
    pub fn condition(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["condition"]),
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

    /// Ranking of the diagnosis (for each role type).
    pub fn rank(&self) -> Option<i64> {
        if let Some(val) = self.value.get("rank") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Role that this diagnosis has within the encounter (e.g. admission, billing,
    /// discharge …).
    pub fn fhir_use(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("use") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._rank() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.condition().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rank() {}
        if let Some(_val) = self.fhir_use() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Encounter_DiagnosisBuilder {
    pub(crate) value: Value,
}

impl Encounter_DiagnosisBuilder {
    pub fn build(&self) -> Encounter_Diagnosis {
        Encounter_Diagnosis {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Encounter_Diagnosis) -> Encounter_DiagnosisBuilder {
        Encounter_DiagnosisBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(condition: Reference) -> Encounter_DiagnosisBuilder {
        let mut __value: Value = json!({});
        __value["condition"] = json!(condition.value);
        return Encounter_DiagnosisBuilder { value: __value };
    }

    pub fn _rank<'a>(&'a mut self, val: Element) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["_rank"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rank<'a>(&'a mut self, val: i64) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["rank"] = json!(val);
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Encounter_DiagnosisBuilder {
        self.value["use"] = json!(val.value);
        return self;
    }
}
