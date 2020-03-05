#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::RiskEvidenceSynthesis_PrecisionEstimate::RiskEvidenceSynthesis_PrecisionEstimate;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_RiskEstimate<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RiskEvidenceSynthesis_RiskEstimate<'_> {
    pub fn new(value: &Value) -> RiskEvidenceSynthesis_RiskEstimate {
        RiskEvidenceSynthesis_RiskEstimate {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for denominatorCount
    pub fn _denominator_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_denominatorCount") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numeratorCount
    pub fn _numerator_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numeratorCount") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The sample size for the group that was measured for this risk estimate.
    pub fn denominator_count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("denominatorCount") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Human-readable summary of risk estimate.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// The number of group members with the outcome of interest.
    pub fn numerator_count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numeratorCount") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// A description of the precision of the estimate for the effect.
    pub fn precision_estimate(&self) -> Option<Vec<RiskEvidenceSynthesis_PrecisionEstimate>> {
        if let Some(Value::Array(val)) = self.value.get("precisionEstimate") {
            return Some(
                val.into_iter()
                    .map(|e| RiskEvidenceSynthesis_PrecisionEstimate {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Examples include proportion and mean.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies the UCUM unit for the outcome.
    pub fn unit_of_measure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfMeasure") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The point estimate of the risk estimate.
    pub fn value(&self) -> Option<f64> {
        if let Some(val) = self.value.get("value") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._denominator_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._numerator_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.denominator_count() {}
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.numerator_count() {}
        if let Some(_val) = self.precision_estimate() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit_of_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_RiskEstimateBuilder {
    pub(crate) value: Value,
}

impl RiskEvidenceSynthesis_RiskEstimateBuilder {
    pub fn build(&self) -> RiskEvidenceSynthesis_RiskEstimate {
        RiskEvidenceSynthesis_RiskEstimate {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: RiskEvidenceSynthesis_RiskEstimate,
    ) -> RiskEvidenceSynthesis_RiskEstimateBuilder {
        RiskEvidenceSynthesis_RiskEstimateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RiskEvidenceSynthesis_RiskEstimateBuilder {
        let mut __value: Value = json!({});
        return RiskEvidenceSynthesis_RiskEstimateBuilder { value: __value };
    }

    pub fn _denominator_count<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["_denominatorCount"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _numerator_count<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["_numeratorCount"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn denominator_count<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["denominatorCount"] = json!(val);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn numerator_count<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["numeratorCount"] = json!(val);
        return self;
    }

    pub fn precision_estimate<'a>(
        &'a mut self,
        val: Vec<RiskEvidenceSynthesis_PrecisionEstimate>,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["precisionEstimate"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn unit_of_measure<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["unitOfMeasure"] = json!(val.value);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: f64) -> &'a mut RiskEvidenceSynthesis_RiskEstimateBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
