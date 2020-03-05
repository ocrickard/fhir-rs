#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductIngredient_Strength::MedicinalProductIngredient_Strength;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct MedicinalProductIngredient_SpecifiedSubstance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductIngredient_SpecifiedSubstance<'_> {
    pub fn new(value: &Value) -> MedicinalProductIngredient_SpecifiedSubstance {
        MedicinalProductIngredient_SpecifiedSubstance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The specified substance.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// Confidentiality level of the specified substance as the ingredient.
    pub fn confidentiality(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("confidentiality") {
            return Some(CodeableConcept {
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

    /// The group of specified substance, e.g. group 1 to 4.
    pub fn group(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["group"]),
        }
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

    /// Quantity of the substance or specified substance present in the manufactured
    /// item or pharmaceutical product.
    pub fn strength(&self) -> Option<Vec<MedicinalProductIngredient_Strength>> {
        if let Some(Value::Array(val)) = self.value.get("strength") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductIngredient_Strength {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.confidentiality() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.group().validate() {
            return false;
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.strength() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductIngredient_SpecifiedSubstanceBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductIngredient_SpecifiedSubstanceBuilder {
    pub fn build(&self) -> MedicinalProductIngredient_SpecifiedSubstance {
        MedicinalProductIngredient_SpecifiedSubstance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductIngredient_SpecifiedSubstance,
    ) -> MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        MedicinalProductIngredient_SpecifiedSubstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        code: CodeableConcept,
        group: CodeableConcept,
    ) -> MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        __value["group"] = json!(group.value);
        return MedicinalProductIngredient_SpecifiedSubstanceBuilder { value: __value };
    }

    pub fn confidentiality<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        self.value["confidentiality"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength<'a>(
        &'a mut self,
        val: Vec<MedicinalProductIngredient_Strength>,
    ) -> &'a mut MedicinalProductIngredient_SpecifiedSubstanceBuilder {
        self.value["strength"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
