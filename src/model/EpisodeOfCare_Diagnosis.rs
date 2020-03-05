#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.

#[derive(Debug)]
pub struct EpisodeOfCare_Diagnosis<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EpisodeOfCare_Diagnosis<'_> {
    pub fn new(value: &Value) -> EpisodeOfCare_Diagnosis {
        EpisodeOfCare_Diagnosis {
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

    /// A list of conditions/problems/diagnoses that this episode of care is intended to
    /// be providing care for.
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

    /// Role that this diagnosis has within the episode of care (e.g. admission,
    /// billing, discharge …).
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
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
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EpisodeOfCare_DiagnosisBuilder {
    pub(crate) value: Value,
}

impl EpisodeOfCare_DiagnosisBuilder {
    pub fn build(&self) -> EpisodeOfCare_Diagnosis {
        EpisodeOfCare_Diagnosis {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: EpisodeOfCare_Diagnosis) -> EpisodeOfCare_DiagnosisBuilder {
        EpisodeOfCare_DiagnosisBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(condition: Reference) -> EpisodeOfCare_DiagnosisBuilder {
        let mut __value: Value = json!({});
        __value["condition"] = json!(condition.value);
        return EpisodeOfCare_DiagnosisBuilder { value: __value };
    }

    pub fn _rank<'a>(&'a mut self, val: Element) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["_rank"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rank<'a>(&'a mut self, val: i64) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["rank"] = json!(val);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: CodeableConcept) -> &'a mut EpisodeOfCare_DiagnosisBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }
}
