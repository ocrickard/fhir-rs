#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Observation_ReferenceRange::Observation_ReferenceRange;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::SampledData::SampledData;
use serde_json::value::Value;

/// Measurements and simple assertions made about a patient, device or other
/// subject.

#[derive(Debug)]
pub struct Observation_Component<'a> {
    pub value: &'a Value,
}

impl Observation_Component<'_> {
    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    /// Provides a reason why the expected value in the element
    /// Observation.component.value[x] is missing.
    pub fn data_absent_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("dataAbsentReason") {
            return Some(CodeableConcept { value: val });
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

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A categorical assessment of an observation value.  For example, high, low,
    /// normal.
    pub fn interpretation(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("interpretation") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Guidance on how to interpret the value by comparison to a normal or recommended
    /// range.
    pub fn reference_range(&self) -> Option<Vec<Observation_ReferenceRange>> {
        if let Some(Value::Array(val)) = self.value.get("referenceRange") {
            return Some(
                val.into_iter()
                    .map(|e| Observation_ReferenceRange { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
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

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("valueRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Describes what was observed. Sometimes this is called the observation "code".
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("valueSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.value_time() {}
        if let Some(_val) = self.data_absent_reason() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self._value_time() {
            _val.validate();
        }
        if let Some(_val) = self.value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._value_integer() {
            _val.validate();
        }
        if let Some(_val) = self.interpretation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reference_range() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._value_string() {
            _val.validate();
        }
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.value_period() {
            _val.validate();
        }
        if let Some(_val) = self.value_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.value_range() {
            _val.validate();
        }
        let _ = self.code().validate();
        if let Some(_val) = self.value_sampled_data() {
            _val.validate();
        }
        return true;
    }
}
