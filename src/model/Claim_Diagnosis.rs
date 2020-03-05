#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_Diagnosis<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Claim_Diagnosis<'_> {
    pub fn new(value: &Value) -> Claim_Diagnosis {
        Claim_Diagnosis {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The nature of illness or problem in a coded form or as a reference to an
    /// external defined Condition.
    pub fn diagnosis_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("diagnosisCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The nature of illness or problem in a coded form or as a reference to an
    /// external defined Condition.
    pub fn diagnosis_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("diagnosisReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Indication of whether the diagnosis was present on admission to a facility.
    pub fn on_admission(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("onAdmission") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A package billing code or bundle code used to group products and services to a
    /// particular health condition (such as heart attack) which is based on a
    /// predetermined grouping code system.
    pub fn package_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("packageCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A number to uniquely identify diagnosis entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// When the condition was observed or the relative ranking.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.diagnosis_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.diagnosis_reference() {
            if !_val.validate() {
                return false;
            }
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
        if let Some(_val) = self.on_admission() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.package_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Claim_DiagnosisBuilder {
    pub(crate) value: Value,
}

impl Claim_DiagnosisBuilder {
    pub fn build(&self) -> Claim_Diagnosis {
        Claim_Diagnosis {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Claim_Diagnosis) -> Claim_DiagnosisBuilder {
        Claim_DiagnosisBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Claim_DiagnosisBuilder {
        let mut __value: Value = json!({});
        return Claim_DiagnosisBuilder { value: __value };
    }

    pub fn _sequence<'a>(&'a mut self, val: Element) -> &'a mut Claim_DiagnosisBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn diagnosis_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Claim_DiagnosisBuilder {
        self.value["diagnosisCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn diagnosis_reference<'a>(&'a mut self, val: Reference) -> &'a mut Claim_DiagnosisBuilder {
        self.value["diagnosisReference"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Claim_DiagnosisBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Claim_DiagnosisBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Claim_DiagnosisBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_admission<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_DiagnosisBuilder {
        self.value["onAdmission"] = json!(val.value);
        return self;
    }

    pub fn package_code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_DiagnosisBuilder {
        self.value["packageCode"] = json!(val.value);
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: i64) -> &'a mut Claim_DiagnosisBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Claim_DiagnosisBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
