#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_Texture<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder_Texture<'_> {
    pub fn new(value: &Value) -> NutritionOrder_Texture {
        NutritionOrder_Texture {
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

    /// The food type(s) (e.g. meats, all foods)  that the texture modification applies
    /// to.  This could be all foods types.
    pub fn food_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("foodType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Any texture modifications (for solid foods) that should be made, e.g. easy to
    /// chew, chopped, ground, and pureed.
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
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.food_type() {
            if !_val.validate() {
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
pub struct NutritionOrder_TextureBuilder {
    pub(crate) value: Value,
}

impl NutritionOrder_TextureBuilder {
    pub fn build(&self) -> NutritionOrder_Texture {
        NutritionOrder_Texture {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder_Texture) -> NutritionOrder_TextureBuilder {
        NutritionOrder_TextureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionOrder_TextureBuilder {
        let mut __value: Value = json!({});
        return NutritionOrder_TextureBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_TextureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn food_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut NutritionOrder_TextureBuilder {
        self.value["foodType"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_TextureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut NutritionOrder_TextureBuilder {
        self.value["modifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_TextureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
