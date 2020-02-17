#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductPharmaceutical_WithdrawalPeriod::MedicinalProductPharmaceutical_WithdrawalPeriod;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A pharmaceutical product described in terms of its composition and dose form.

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_TargetSpecies<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductPharmaceutical_TargetSpecies<'_> {
    pub fn new(value: &Value) -> MedicinalProductPharmaceutical_TargetSpecies {
        MedicinalProductPharmaceutical_TargetSpecies {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Coded expression for the species.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
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

    /// A species specific time during which consumption of animal product is not
    /// appropriate.
    pub fn withdrawal_period(
        &self,
    ) -> Option<Vec<MedicinalProductPharmaceutical_WithdrawalPeriod>> {
        if let Some(Value::Array(val)) = self.value.get("withdrawalPeriod") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductPharmaceutical_WithdrawalPeriod {
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
        if let Some(_val) = self.withdrawal_period() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_TargetSpeciesBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductPharmaceutical_TargetSpeciesBuilder {
    pub fn build(&self) -> MedicinalProductPharmaceutical_TargetSpecies {
        MedicinalProductPharmaceutical_TargetSpecies {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductPharmaceutical_TargetSpecies,
    ) -> MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        MedicinalProductPharmaceutical_TargetSpeciesBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return MedicinalProductPharmaceutical_TargetSpeciesBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn withdrawal_period<'a>(
        &'a mut self,
        val: Vec<MedicinalProductPharmaceutical_WithdrawalPeriod>,
    ) -> &'a mut MedicinalProductPharmaceutical_TargetSpeciesBuilder {
        self.value["withdrawalPeriod"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
