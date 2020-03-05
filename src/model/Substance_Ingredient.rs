#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A homogeneous material with a definite composition.

#[derive(Debug)]
pub struct Substance_Ingredient<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Substance_Ingredient<'_> {
    pub fn new(value: &Value) -> Substance_Ingredient {
        Substance_Ingredient {
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

    /// The amount of the ingredient in the substance - a concentration ratio.
    pub fn quantity(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Another substance that is a component of this substance.
    pub fn substance_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substanceCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Another substance that is a component of this substance.
    pub fn substance_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("substanceReference") {
            return Some(Reference {
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
        if let Some(_val) = self.substance_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.substance_reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Substance_IngredientBuilder {
    pub(crate) value: Value,
}

impl Substance_IngredientBuilder {
    pub fn build(&self) -> Substance_Ingredient {
        Substance_Ingredient {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Substance_Ingredient) -> Substance_IngredientBuilder {
        Substance_IngredientBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Substance_IngredientBuilder {
        let mut __value: Value = json!({});
        return Substance_IngredientBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Substance_IngredientBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Substance_IngredientBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Substance_IngredientBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Ratio) -> &'a mut Substance_IngredientBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn substance_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Substance_IngredientBuilder {
        self.value["substanceCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn substance_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Substance_IngredientBuilder {
        self.value["substanceReference"] = json!(val.value);
        return self;
    }
}
