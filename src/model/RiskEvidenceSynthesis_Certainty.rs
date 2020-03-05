#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::RiskEvidenceSynthesis_CertaintySubcomponent::RiskEvidenceSynthesis_CertaintySubcomponent;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_Certainty<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RiskEvidenceSynthesis_Certainty<'_> {
    pub fn new(value: &Value) -> RiskEvidenceSynthesis_Certainty {
        RiskEvidenceSynthesis_Certainty {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A description of a component of the overall certainty.
    pub fn certainty_subcomponent(
        &self,
    ) -> Option<Vec<RiskEvidenceSynthesis_CertaintySubcomponent>> {
        if let Some(Value::Array(val)) = self.value.get("certaintySubcomponent") {
            return Some(
                val.into_iter()
                    .map(|e| RiskEvidenceSynthesis_CertaintySubcomponent {
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

    /// A human-readable string to clarify or explain concepts about the resource.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A rating of the certainty of the effect estimate.
    pub fn rating(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("rating") {
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
        if let Some(_val) = self.certainty_subcomponent() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rating() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_CertaintyBuilder {
    pub(crate) value: Value,
}

impl RiskEvidenceSynthesis_CertaintyBuilder {
    pub fn build(&self) -> RiskEvidenceSynthesis_Certainty {
        RiskEvidenceSynthesis_Certainty {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: RiskEvidenceSynthesis_Certainty,
    ) -> RiskEvidenceSynthesis_CertaintyBuilder {
        RiskEvidenceSynthesis_CertaintyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RiskEvidenceSynthesis_CertaintyBuilder {
        let mut __value: Value = json!({});
        return RiskEvidenceSynthesis_CertaintyBuilder { value: __value };
    }

    pub fn certainty_subcomponent<'a>(
        &'a mut self,
        val: Vec<RiskEvidenceSynthesis_CertaintySubcomponent>,
    ) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["certaintySubcomponent"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rating<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut RiskEvidenceSynthesis_CertaintyBuilder {
        self.value["rating"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
