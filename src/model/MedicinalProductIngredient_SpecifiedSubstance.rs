#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductIngredient_Strength::MedicinalProductIngredient_Strength;
use serde_json::value::Value;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct MedicinalProductIngredient_SpecifiedSubstance<'a> {
    pub value: &'a Value,
}

impl MedicinalProductIngredient_SpecifiedSubstance<'_> {
    /// Confidentiality level of the specified substance as the ingredient.
    pub fn confidentiality(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("confidentiality") {
            return Some(CodeableConcept { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specified substance.
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

    /// The group of specified substance, e.g. group 1 to 4.
    pub fn group(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["group"],
        }
    }

    /// Quantity of the substance or specified substance present in the manufactured
    /// item or pharmaceutical product.
    pub fn strength(&self) -> Option<Vec<MedicinalProductIngredient_Strength>> {
        if let Some(Value::Array(val)) = self.value.get("strength") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductIngredient_Strength { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.confidentiality() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
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
        let _ = self.group().validate();
        if let Some(_val) = self.strength() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
