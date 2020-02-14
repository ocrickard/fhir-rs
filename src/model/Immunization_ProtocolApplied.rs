#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.

#[derive(Debug)]
pub struct Immunization_ProtocolApplied<'a> {
    pub value: &'a Value,
}

impl Immunization_ProtocolApplied<'_> {
    /// Nominal position in a series.
    pub fn dose_number_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("doseNumberPositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The recommended number of doses to achieve immunity.
    pub fn series_doses_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("seriesDosesPositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for doseNumberString
    pub fn _dose_number_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doseNumberString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates the authority who published the protocol (e.g. ACIP) that is being
    /// followed.
    pub fn authority(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("authority") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The recommended number of doses to achieve immunity.
    pub fn series_doses_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("seriesDosesString") {
            return Some(string);
        }
        return None;
    }

    /// The vaccine preventable disease the dose is being administered against.
    pub fn target_disease(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("targetDisease") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for seriesDosesString
    pub fn _series_doses_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_seriesDosesString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Nominal position in a series.
    pub fn dose_number_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("doseNumberString") {
            return Some(string);
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

    /// Extensions for seriesDosesPositiveInt
    pub fn _series_doses_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_seriesDosesPositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for series
    pub fn _series(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_series") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for doseNumberPositiveInt
    pub fn _dose_number_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doseNumberPositiveInt") {
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

    /// One possible path to achieve presumed immunity against a disease - within the
    /// context of an authority.
    pub fn series(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("series") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.dose_number_positive_int() {}
        if let Some(_val) = self.series_doses_positive_int() {}
        if let Some(_val) = self._dose_number_string() {
            _val.validate();
        }
        if let Some(_val) = self.authority() {
            _val.validate();
        }
        if let Some(_val) = self.series_doses_string() {}
        if let Some(_val) = self.target_disease() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._series_doses_string() {
            _val.validate();
        }
        if let Some(_val) = self.dose_number_string() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._series_doses_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self._series() {
            _val.validate();
        }
        if let Some(_val) = self._dose_number_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.series() {}
        return true;
    }
}
