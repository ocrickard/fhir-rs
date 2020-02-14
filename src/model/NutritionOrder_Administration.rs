#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_Administration<'a> {
    pub value: &'a Value,
}

impl NutritionOrder_Administration<'_> {
    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub fn rate_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("rateQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub fn rate_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("rateRatio") {
            return Some(Ratio { value: val });
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

    /// The volume of formula to provide to the patient per the specified administration
    /// schedule.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The time period and frequency at which the enteral formula should be delivered
    /// to the patient.
    pub fn schedule(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("schedule") {
            return Some(Timing { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.rate_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.rate_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.schedule() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}
