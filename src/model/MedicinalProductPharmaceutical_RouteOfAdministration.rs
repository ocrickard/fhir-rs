#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductPharmaceutical_TargetSpecies::MedicinalProductPharmaceutical_TargetSpecies;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use serde_json::value::Value;

/// A pharmaceutical product described in terms of its composition and dose form.

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_RouteOfAdministration<'a> {
    pub value: &'a Value,
}

impl MedicinalProductPharmaceutical_RouteOfAdministration<'_> {
    /// The maximum dose per day (maximum dose quantity to be administered in any one
    /// 24-h period) that can be administered as per the protocol referenced in the
    /// clinical trial authorisation.
    pub fn max_dose_per_day(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxDosePerDay") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The maximum dose per treatment period that can be administered as per the
    /// protocol referenced in the clinical trial authorisation.
    pub fn max_dose_per_treatment_period(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("maxDosePerTreatmentPeriod") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// A species for which this route applies.
    pub fn target_species(&self) -> Option<Vec<MedicinalProductPharmaceutical_TargetSpecies>> {
        if let Some(Value::Array(val)) = self.value.get("targetSpecies") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductPharmaceutical_TargetSpecies { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Coded expression for the route.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The maximum treatment period during which an Investigational Medicinal Product
    /// can be administered as per the protocol referenced in the clinical trial
    /// authorisation.
    pub fn max_treatment_period(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("maxTreatmentPeriod") {
            return Some(Duration { value: val });
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

    /// The first dose (dose quantity) administered in humans can be specified, for a
    /// product under investigation, using a numerical value and its unit of
    /// measurement.
    pub fn first_dose(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("firstDose") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The maximum single dose that can be administered as per the protocol of a
    /// clinical trial can be specified using a numerical value and its unit of
    /// measurement.
    pub fn max_single_dose(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxSingleDose") {
            return Some(Quantity { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.max_dose_per_day() {
            _val.validate();
        }
        if let Some(_val) = self.max_dose_per_treatment_period() {
            _val.validate();
        }
        if let Some(_val) = self.target_species() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.code().validate();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.max_treatment_period() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.first_dose() {
            _val.validate();
        }
        if let Some(_val) = self.max_single_dose() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}
