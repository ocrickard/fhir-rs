#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_PrecisionEstimate<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RiskEvidenceSynthesis_PrecisionEstimate<'_> {
    pub fn new(value: &Value) -> RiskEvidenceSynthesis_PrecisionEstimate {
        RiskEvidenceSynthesis_PrecisionEstimate {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for from
    pub fn _from(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_from") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for level
    pub fn _level(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_level") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for to
    pub fn _to(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_to") {
            return Some(Element {
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

    /// Lower bound of confidence interval.
    pub fn from(&self) -> Option<f64> {
        if let Some(val) = self.value.get("from") {
            return Some(val.as_f64().unwrap());
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

    /// Use 95 for a 95% confidence interval.
    pub fn level(&self) -> Option<f64> {
        if let Some(val) = self.value.get("level") {
            return Some(val.as_f64().unwrap());
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

    /// Upper bound of confidence interval.
    pub fn to(&self) -> Option<f64> {
        if let Some(val) = self.value.get("to") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Examples include confidence interval and interquartile range.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._from() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._level() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._to() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.from() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.level() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.to() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_PrecisionEstimateBuilder {
    pub(crate) value: Value,
}

impl RiskEvidenceSynthesis_PrecisionEstimateBuilder {
    pub fn build(&self) -> RiskEvidenceSynthesis_PrecisionEstimate {
        RiskEvidenceSynthesis_PrecisionEstimate {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: RiskEvidenceSynthesis_PrecisionEstimate,
    ) -> RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        RiskEvidenceSynthesis_PrecisionEstimateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        let mut __value: Value = json!({});
        return RiskEvidenceSynthesis_PrecisionEstimateBuilder { value: __value };
    }

    pub fn _from<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["_from"] = json!(val.value);
        return self;
    }

    pub fn _level<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["_level"] = json!(val.value);
        return self;
    }

    pub fn _to<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["_to"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn from<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["from"] = json!(val);
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn level<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["level"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn to<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["to"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskEvidenceSynthesis_PrecisionEstimateBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
