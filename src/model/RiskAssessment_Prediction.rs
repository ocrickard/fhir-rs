#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::value::Value;

/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.

#[derive(Debug)]
pub struct RiskAssessment_Prediction<'a> {
    pub value: &'a Value,
}

impl RiskAssessment_Prediction<'_> {
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

    /// Additional information explaining the basis for the prediction.
    pub fn rationale(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rationale") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for relativeRisk
    pub fn _relative_risk(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relativeRisk") {
            return Some(Element { value: val });
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

    /// Indicates how likely the outcome is (in the specified timeframe).
    pub fn probability_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("probabilityDecimal") {
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

    /// Extensions for probabilityDecimal
    pub fn _probability_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_probabilityDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates how likely the outcome is (in the specified timeframe).
    pub fn probability_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("probabilityRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Indicates how likely the outcome is (in the specified timeframe), expressed as a
    /// qualitative value (e.g. low, medium, or high).
    pub fn qualitative_risk(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("qualitativeRisk") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub fn when_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("whenPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub fn when_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("whenRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Extensions for rationale
    pub fn _rationale(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rationale") {
            return Some(Element { value: val });
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

    /// One of the potential outcomes for the patient (e.g. remission, death,  a
    /// particular condition).
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.relative_risk() {}
        if let Some(_val) = self.rationale() {}
        if let Some(_val) = self._relative_risk() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.probability_decimal() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._probability_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.probability_range() {
            _val.validate();
        }
        if let Some(_val) = self.qualitative_risk() {
            _val.validate();
        }
        if let Some(_val) = self.when_period() {
            _val.validate();
        }
        if let Some(_val) = self.when_range() {
            _val.validate();
        }
        if let Some(_val) = self._rationale() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.outcome() {
            _val.validate();
        }
        return true;
    }
}
