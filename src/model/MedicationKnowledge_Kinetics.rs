#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Kinetics<'a> {
    pub value: &'a Value,
}

impl MedicationKnowledge_Kinetics<'_> {
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

    /// The drug concentration measured at certain discrete points in time.
    pub fn area_under_curve(&self) -> Option<Vec<Quantity>> {
        if let Some(Value::Array(val)) = self.value.get("areaUnderCurve") {
            return Some(
                val.into_iter()
                    .map(|e| Quantity { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The median lethal dose of a drug.
    pub fn lethal_dose_5_0(&self) -> Option<Vec<Quantity>> {
        if let Some(Value::Array(val)) = self.value.get("lethalDose50") {
            return Some(
                val.into_iter()
                    .map(|e| Quantity { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The time required for any specified property (e.g., the concentration of a
    /// substance in the body) to decrease by half.
    pub fn half_life_period(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("halfLifePeriod") {
            return Some(Duration { value: val });
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
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.area_under_curve() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.lethal_dose_5_0() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.half_life_period() {
            _val.validate();
        }
        return true;
    }
}
