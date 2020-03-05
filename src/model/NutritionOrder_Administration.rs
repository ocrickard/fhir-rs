#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_Administration<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder_Administration<'_> {
    pub fn new(value: &Value) -> NutritionOrder_Administration {
        NutritionOrder_Administration {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The volume of formula to provide to the patient per the specified administration
    /// schedule.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub fn rate_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("rateQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The rate of administration of formula via a feeding pump, e.g. 60 mL per hour,
    /// according to the specified schedule.
    pub fn rate_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("rateRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time period and frequency at which the enteral formula should be delivered
    /// to the patient.
    pub fn schedule(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("schedule") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.rate_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.rate_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.schedule() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionOrder_AdministrationBuilder {
    pub(crate) value: Value,
}

impl NutritionOrder_AdministrationBuilder {
    pub fn build(&self) -> NutritionOrder_Administration {
        NutritionOrder_Administration {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder_Administration) -> NutritionOrder_AdministrationBuilder {
        NutritionOrder_AdministrationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionOrder_AdministrationBuilder {
        let mut __value: Value = json!({});
        return NutritionOrder_AdministrationBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn rate_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["rateQuantity"] = json!(val.value);
        return self;
    }

    pub fn rate_ratio<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["rateRatio"] = json!(val.value);
        return self;
    }

    pub fn schedule<'a>(&'a mut self, val: Timing) -> &'a mut NutritionOrder_AdministrationBuilder {
        self.value["schedule"] = json!(val.value);
        return self;
    }
}
