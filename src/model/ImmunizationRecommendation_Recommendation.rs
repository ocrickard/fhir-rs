#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ImmunizationRecommendation_DateCriterion::ImmunizationRecommendation_DateCriterion;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.

#[derive(Debug)]
pub struct ImmunizationRecommendation_Recommendation<'a> {
    pub value: &'a Value,
}

impl ImmunizationRecommendation_Recommendation<'_> {
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

    /// Vaccine(s) or vaccine group that pertain to the recommendation.
    pub fn vaccine_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("vaccineCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The reason for the assigned forecast status.
    pub fn forecast_reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("forecastReason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// Extensions for doseNumberPositiveInt
    pub fn _dose_number_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doseNumberPositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Nominal position of the recommended dose in a series (e.g. dose 2 is the next
    /// recommended dose).
    pub fn dose_number_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("doseNumberString") {
            return Some(string);
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

    /// Extensions for series
    pub fn _series(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_series") {
            return Some(Element { value: val });
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

    /// Vaccine(s) which should not be used to fulfill the recommendation.
    pub fn contraindicated_vaccine_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("contraindicatedVaccineCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the patient status with respect to the path to immunity for the target
    /// disease.
    pub fn forecast_status(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["forecastStatus"],
        }
    }

    /// The targeted disease for the recommendation.
    pub fn target_disease(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("targetDisease") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Vaccine date recommendations.  For example, earliest date to administer, latest
    /// date to administer, etc.
    pub fn date_criterion(&self) -> Option<Vec<ImmunizationRecommendation_DateCriterion>> {
        if let Some(Value::Array(val)) = self.value.get("dateCriterion") {
            return Some(
                val.into_iter()
                    .map(|e| ImmunizationRecommendation_DateCriterion { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contains the description about the protocol under which the vaccine was
    /// administered.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Nominal position of the recommended dose in a series (e.g. dose 2 is the next
    /// recommended dose).
    pub fn dose_number_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("doseNumberPositiveInt") {
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

    /// Extensions for seriesDosesPositiveInt
    pub fn _series_doses_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_seriesDosesPositiveInt") {
            return Some(Element { value: val });
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

    /// Patient Information that supports the status and recommendation.  This includes
    /// patient observations, adverse reactions and allergy/intolerance information.
    pub fn supporting_patient_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingPatientInformation") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Immunization event history and/or evaluation that supports the status and
    /// recommendation.
    pub fn supporting_immunization(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingImmunization") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.vaccine_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.forecast_reason() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._dose_number_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.dose_number_string() {}
        if let Some(_val) = self._series_doses_string() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.series() {}
        if let Some(_val) = self._series() {
            _val.validate();
        }
        if let Some(_val) = self.series_doses_positive_int() {}
        if let Some(_val) = self.contraindicated_vaccine_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.forecast_status().validate();
        if let Some(_val) = self.target_disease() {
            _val.validate();
        }
        if let Some(_val) = self.date_criterion() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.dose_number_positive_int() {}
        if let Some(_val) = self._dose_number_string() {
            _val.validate();
        }
        if let Some(_val) = self._series_doses_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.series_doses_string() {}
        if let Some(_val) = self.supporting_patient_information() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.supporting_immunization() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
