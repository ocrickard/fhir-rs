#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_Nutrient<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder_Nutrient<'_> {
    pub fn new(value: &Value) -> NutritionOrder_Nutrient {
        NutritionOrder_Nutrient {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The quantity of the specified nutrient to include in diet.
    pub fn amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amount") {
            return Some(Quantity {
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

    /// The nutrient that is being modified such as carbohydrate or sodium.
    pub fn modifier(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("modifier") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.amount() {
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
        if let Some(_val) = self.modifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionOrder_NutrientBuilder {
    pub(crate) value: Value,
}

impl NutritionOrder_NutrientBuilder {
    pub fn build(&self) -> NutritionOrder_Nutrient {
        NutritionOrder_Nutrient {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder_Nutrient) -> NutritionOrder_NutrientBuilder {
        NutritionOrder_NutrientBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionOrder_NutrientBuilder {
        let mut __value: Value = json!({});
        return NutritionOrder_NutrientBuilder { value: __value };
    }

    pub fn amount<'a>(&'a mut self, val: Quantity) -> &'a mut NutritionOrder_NutrientBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_NutrientBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_NutrientBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut NutritionOrder_NutrientBuilder {
        self.value["modifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_NutrientBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
