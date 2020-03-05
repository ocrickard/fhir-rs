#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Cost<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_Cost<'_> {
    pub fn new(value: &Value) -> InsurancePlan_Cost {
        InsurancePlan_Cost {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Whether the cost applies to in-network or out-of-network providers (in-network;
    /// out-of-network; other).
    pub fn applicability(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("applicability") {
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

    /// Additional information about the cost, such as information about funding sources
    /// (e.g. HSA, HRA, FSA, RRA).
    pub fn qualifiers(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("qualifiers") {
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

    /// Type of cost (copay; individual cap; family cap; coinsurance; deductible).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// The actual cost value. (some of the costs may be represented as percentages
    /// rather than currency, e.g. 10% coinsurance).
    pub fn value(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("value") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.applicability() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.qualifiers() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.value() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct InsurancePlan_CostBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_CostBuilder {
    pub fn build(&self) -> InsurancePlan_Cost {
        InsurancePlan_Cost {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_Cost) -> InsurancePlan_CostBuilder {
        InsurancePlan_CostBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> InsurancePlan_CostBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return InsurancePlan_CostBuilder { value: __value };
    }

    pub fn applicability<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut InsurancePlan_CostBuilder {
        self.value["applicability"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut InsurancePlan_CostBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_CostBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_CostBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn qualifiers<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut InsurancePlan_CostBuilder {
        self.value["qualifiers"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value<'a>(&'a mut self, val: Quantity) -> &'a mut InsurancePlan_CostBuilder {
        self.value["value"] = json!(val.value);
        return self;
    }
}
