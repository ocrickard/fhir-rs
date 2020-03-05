#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.

#[derive(Debug)]
pub struct RiskAssessment_Prediction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RiskAssessment_Prediction<'_> {
    pub fn new(value: &Value) -> RiskAssessment_Prediction {
        RiskAssessment_Prediction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for probabilityDecimal
    pub fn _probability_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_probabilityDecimal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for rationale
    pub fn _rationale(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rationale") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for relativeRisk
    pub fn _relative_risk(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relativeRisk") {
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

    /// One of the potential outcomes for the patient (e.g. remission, death,  a
    /// particular condition).
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates how likely the outcome is (in the specified timeframe).
    pub fn probability_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("probabilityDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Indicates how likely the outcome is (in the specified timeframe).
    pub fn probability_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("probabilityRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates how likely the outcome is (in the specified timeframe), expressed as a
    /// qualitative value (e.g. low, medium, or high).
    pub fn qualitative_risk(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("qualitativeRisk") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information explaining the basis for the prediction.
    pub fn rationale(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rationale") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the risk for this particular subject (with their specific
    /// characteristics) divided by the risk of the population in general.  (Numbers
    /// greater than 1 = higher risk than the population, numbers less than 1 = lower
    /// risk.).
    pub fn relative_risk(&self) -> Option<f64> {
        if let Some(val) = self.value.get("relativeRisk") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub fn when_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("whenPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub fn when_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("whenRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._probability_decimal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._rationale() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._relative_risk() {
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
        if let Some(_val) = self.outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.probability_decimal() {}
        if let Some(_val) = self.probability_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.qualitative_risk() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.rationale() {}
        if let Some(_val) = self.relative_risk() {}
        if let Some(_val) = self.when_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.when_range() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RiskAssessment_PredictionBuilder {
    pub(crate) value: Value,
}

impl RiskAssessment_PredictionBuilder {
    pub fn build(&self) -> RiskAssessment_Prediction {
        RiskAssessment_Prediction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RiskAssessment_Prediction) -> RiskAssessment_PredictionBuilder {
        RiskAssessment_PredictionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RiskAssessment_PredictionBuilder {
        let mut __value: Value = json!({});
        return RiskAssessment_PredictionBuilder { value: __value };
    }

    pub fn _probability_decimal<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["_probabilityDecimal"] = json!(val.value);
        return self;
    }

    pub fn _rationale<'a>(&'a mut self, val: Element) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["_rationale"] = json!(val.value);
        return self;
    }

    pub fn _relative_risk<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["_relativeRisk"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["outcome"] = json!(val.value);
        return self;
    }

    pub fn probability_decimal<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["probabilityDecimal"] = json!(val);
        return self;
    }

    pub fn probability_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["probabilityRange"] = json!(val.value);
        return self;
    }

    pub fn qualitative_risk<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["qualitativeRisk"] = json!(val.value);
        return self;
    }

    pub fn rationale<'a>(&'a mut self, val: &str) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["rationale"] = json!(val);
        return self;
    }

    pub fn relative_risk<'a>(&'a mut self, val: f64) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["relativeRisk"] = json!(val);
        return self;
    }

    pub fn when_period<'a>(&'a mut self, val: Period) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["whenPeriod"] = json!(val.value);
        return self;
    }

    pub fn when_range<'a>(&'a mut self, val: Range) -> &'a mut RiskAssessment_PredictionBuilder {
        self.value["whenRange"] = json!(val.value);
        return self;
    }
}
