#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with a
/// clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.

#[derive(Debug)]
pub struct ClinicalImpression_Investigation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClinicalImpression_Investigation<'_> {
    pub fn new(value: &Value) -> ClinicalImpression_Investigation {
        ClinicalImpression_Investigation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A name/code for the group ("set") of investigations. Typically, this will be
    /// something like "signs", "symptoms", "clinical", "diagnostic", but the list is
    /// not constrained, and others such groups such as
    /// (exposure|family|travel|nutritional) history may be used.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
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

    /// A record of a specific investigation that was undertaken.
    pub fn item(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ClinicalImpression_InvestigationBuilder {
    pub(crate) value: Value,
}

impl ClinicalImpression_InvestigationBuilder {
    pub fn build(&self) -> ClinicalImpression_Investigation {
        ClinicalImpression_Investigation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ClinicalImpression_Investigation,
    ) -> ClinicalImpression_InvestigationBuilder {
        ClinicalImpression_InvestigationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ClinicalImpression_InvestigationBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ClinicalImpression_InvestigationBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalImpression_InvestigationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClinicalImpression_InvestigationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ClinicalImpression_InvestigationBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClinicalImpression_InvestigationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
