#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::NutritionOrder_Nutrient::NutritionOrder_Nutrient;
use crate::model::NutritionOrder_Texture::NutritionOrder_Texture;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_OralDiet<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder_OralDiet<'_> {
    pub fn new(value: &Value) -> NutritionOrder_OralDiet {
        NutritionOrder_OralDiet {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for instruction
    pub fn _instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instruction") {
            return Some(Element {
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

    /// The required consistency (e.g. honey-thick, nectar-thick, thin, thickened.) of
    /// liquids or fluids served to the patient.
    pub fn fluid_consistency_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("fluidConsistencyType") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
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

    /// Free text or additional instructions or information pertaining to the oral diet.
    pub fn instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instruction") {
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

    /// Class that defines the quantity and type of nutrient modifications (for example
    /// carbohydrate, fiber or sodium) required for the oral diet.
    pub fn nutrient(&self) -> Option<Vec<NutritionOrder_Nutrient>> {
        if let Some(Value::Array(val)) = self.value.get("nutrient") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionOrder_Nutrient {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The time period and frequency at which the diet should be given.  The diet
    /// should be given for the combination of all schedules if more than one schedule
    /// is present.
    pub fn schedule(&self) -> Option<Vec<Timing>> {
        if let Some(Value::Array(val)) = self.value.get("schedule") {
            return Some(
                val.into_iter()
                    .map(|e| Timing {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Class that describes any texture modifications required for the patient to
    /// safely consume various types of solid foods.
    pub fn texture(&self) -> Option<Vec<NutritionOrder_Texture>> {
        if let Some(Value::Array(val)) = self.value.get("texture") {
            return Some(
                val.into_iter()
                    .map(|e| NutritionOrder_Texture {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The kind of diet or dietary restriction such as fiber restricted diet or
    /// diabetic diet.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fluid_consistency_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.instruction() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.nutrient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.schedule() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.texture() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionOrder_OralDietBuilder {
    pub(crate) value: Value,
}

impl NutritionOrder_OralDietBuilder {
    pub fn build(&self) -> NutritionOrder_OralDiet {
        NutritionOrder_OralDiet {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder_OralDiet) -> NutritionOrder_OralDietBuilder {
        NutritionOrder_OralDietBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionOrder_OralDietBuilder {
        let mut __value: Value = json!({});
        return NutritionOrder_OralDietBuilder { value: __value };
    }

    pub fn _instruction<'a>(&'a mut self, val: Element) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["_instruction"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fluid_consistency_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["fluidConsistencyType"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn instruction<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["instruction"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn nutrient<'a>(
        &'a mut self,
        val: Vec<NutritionOrder_Nutrient>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["nutrient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn schedule<'a>(&'a mut self, val: Vec<Timing>) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["schedule"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn texture<'a>(
        &'a mut self,
        val: Vec<NutritionOrder_Texture>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["texture"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut NutritionOrder_OralDietBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
