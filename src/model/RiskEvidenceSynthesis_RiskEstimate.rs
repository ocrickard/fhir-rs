#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::RiskEvidenceSynthesis_PrecisionEstimate::RiskEvidenceSynthesis_PrecisionEstimate;
use serde_json::value::Value;

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.

#[derive(Debug)]
pub struct RiskEvidenceSynthesis_RiskEstimate<'a> {
    pub value: &'a Value,
}

impl RiskEvidenceSynthesis_RiskEstimate<'_> {
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

    /// Specifies the UCUM unit for the outcome.
    pub fn unit_of_measure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfMeasure") {
            return Some(CodeableConcept { value: val });
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

    /// The sample size for the group that was measured for this risk estimate.
    pub fn denominator_count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("denominatorCount") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// A description of the precision of the estimate for the effect.
    pub fn precision_estimate(&self) -> Option<Vec<RiskEvidenceSynthesis_PrecisionEstimate>> {
        if let Some(Value::Array(val)) = self.value.get("precisionEstimate") {
            return Some(
                val.into_iter()
                    .map(|e| RiskEvidenceSynthesis_PrecisionEstimate { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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

    /// Extensions for numeratorCount
    pub fn _numerator_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numeratorCount") {
            return Some(Element { value: val });
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

    /// Examples include proportion and mean.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for denominatorCount
    pub fn _denominator_count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_denominatorCount") {
            return Some(Element { value: val });
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

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.unit_of_measure() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.denominator_count() {}
        if let Some(_val) = self.precision_estimate() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.value() {}
        if let Some(_val) = self._numerator_count() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._denominator_count() {
            _val.validate();
        }
        if let Some(_val) = self.numerator_count() {}
        if let Some(_val) = self._value() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        return true;
    }
}
