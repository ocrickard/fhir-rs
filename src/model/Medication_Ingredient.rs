#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource is primarily used for the identification and definition of a
/// medication for the purposes of prescribing, dispensing, and administering a
/// medication as well as for making statements about medication use.

#[derive(Debug)]
pub struct Medication_Ingredient<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Medication_Ingredient<'_> {
    pub fn new(value: &Value) -> Medication_Ingredient {
        Medication_Ingredient {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for isActive
    pub fn _is_active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isActive") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Indication of whether this ingredient affects the therapeutic action of the
    /// drug.
    pub fn is_active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isActive") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The actual ingredient - either a substance (simple ingredient) or another
    /// medication of a medication.
    pub fn item_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("itemCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual ingredient - either a substance (simple ingredient) or another
    /// medication of a medication.
    pub fn item_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("itemReference") {
            return Some(Reference {
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

    /// Specifies how many (or how much) of the items there are in this Medication.  For
    /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
    /// 250mg and the denominator is 1 tablet.
    pub fn strength(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("strength") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._is_active() {
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
        if let Some(_val) = self.is_active() {}
        if let Some(_val) = self.item_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.item_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.strength() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Medication_IngredientBuilder {
    pub(crate) value: Value,
}

impl Medication_IngredientBuilder {
    pub fn build(&self) -> Medication_Ingredient {
        Medication_Ingredient {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Medication_Ingredient) -> Medication_IngredientBuilder {
        Medication_IngredientBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Medication_IngredientBuilder {
        let mut __value: Value = json!({});
        return Medication_IngredientBuilder { value: __value };
    }

    pub fn _is_active<'a>(&'a mut self, val: Element) -> &'a mut Medication_IngredientBuilder {
        self.value["_isActive"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Medication_IngredientBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Medication_IngredientBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn is_active<'a>(&'a mut self, val: bool) -> &'a mut Medication_IngredientBuilder {
        self.value["isActive"] = json!(val);
        return self;
    }

    pub fn item_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Medication_IngredientBuilder {
        self.value["itemCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn item_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Medication_IngredientBuilder {
        self.value["itemReference"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Medication_IngredientBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength<'a>(&'a mut self, val: Ratio) -> &'a mut Medication_IngredientBuilder {
        self.value["strength"] = json!(val.value);
        return self;
    }
}
