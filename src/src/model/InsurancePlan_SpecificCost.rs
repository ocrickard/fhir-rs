#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::InsurancePlan_Benefit1::InsurancePlan_Benefit1;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_SpecificCost<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_SpecificCost<'_> {
    pub fn new(value: &Value) -> InsurancePlan_SpecificCost {
        InsurancePlan_SpecificCost {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// List of the specific benefits under this category of benefit.
    pub fn benefit(&self) -> Option<Vec<InsurancePlan_Benefit1>> {
        if let Some(Value::Array(val)) = self.value.get("benefit") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Benefit1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// General category of benefit (Medical; Dental; Vision; Drug; Mental Health;
    /// Substance Abuse; Hospice, Home Health).
    pub fn category(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["category"]),
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.benefit() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.category().validate() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct InsurancePlan_SpecificCostBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_SpecificCostBuilder {
    pub fn build(&self) -> InsurancePlan_SpecificCost {
        InsurancePlan_SpecificCost {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_SpecificCost) -> InsurancePlan_SpecificCostBuilder {
        InsurancePlan_SpecificCostBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(category: CodeableConcept) -> InsurancePlan_SpecificCostBuilder {
        let mut __value: Value = json!({});
        __value["category"] = json!(category.value);
        return InsurancePlan_SpecificCostBuilder { value: __value };
    }

    pub fn benefit<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_Benefit1>,
    ) -> &'a mut InsurancePlan_SpecificCostBuilder {
        self.value["benefit"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_SpecificCostBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_SpecificCostBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_SpecificCostBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
