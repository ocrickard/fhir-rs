#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coverage_Exception::Coverage_Exception;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.

#[derive(Debug)]
pub struct Coverage_CostToBeneficiary<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Coverage_CostToBeneficiary<'_> {
    pub fn new(value: &Value) -> Coverage_CostToBeneficiary {
        Coverage_CostToBeneficiary {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A suite of codes indicating exceptions or reductions to patient costs and their
    /// effective periods.
    pub fn exception(&self) -> Option<Vec<Coverage_Exception>> {
        if let Some(Value::Array(val)) = self.value.get("exception") {
            return Some(
                val.into_iter()
                    .map(|e| Coverage_Exception {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The category of patient centric costs associated with treatment.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The amount due from the patient for the cost category.
    pub fn value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("valueMoney") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The amount due from the patient for the cost category.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.exception() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_money() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Coverage_CostToBeneficiaryBuilder {
    pub(crate) value: Value,
}

impl Coverage_CostToBeneficiaryBuilder {
    pub fn build(&self) -> Coverage_CostToBeneficiary {
        Coverage_CostToBeneficiary {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Coverage_CostToBeneficiary) -> Coverage_CostToBeneficiaryBuilder {
        Coverage_CostToBeneficiaryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Coverage_CostToBeneficiaryBuilder {
        let mut __value: Value = json!({});
        return Coverage_CostToBeneficiaryBuilder { value: __value };
    }

    pub fn exception<'a>(
        &'a mut self,
        val: Vec<Coverage_Exception>,
    ) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["exception"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn value_money<'a>(&'a mut self, val: Money) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["valueMoney"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut Coverage_CostToBeneficiaryBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }
}
