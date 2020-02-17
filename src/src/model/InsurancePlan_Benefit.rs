#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::InsurancePlan_Limit::InsurancePlan_Limit;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Benefit<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_Benefit<'_> {
    pub fn new(value: &Value) -> InsurancePlan_Benefit {
        InsurancePlan_Benefit {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for requirement
    pub fn _requirement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirement") {
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

    /// The specific limits on the benefit.
    pub fn limit(&self) -> Option<Vec<InsurancePlan_Limit>> {
        if let Some(Value::Array(val)) = self.value.get("limit") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Limit {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The referral requirements to have access/coverage for this benefit.
    pub fn requirement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirement") {
            return Some(string);
        }
        return None;
    }

    /// Type of benefit (primary care; speciality care; inpatient; outpatient).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._requirement() {
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
        if let Some(_val) = self.limit() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requirement() {}
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct InsurancePlan_BenefitBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_BenefitBuilder {
    pub fn build(&self) -> InsurancePlan_Benefit {
        InsurancePlan_Benefit {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_Benefit) -> InsurancePlan_BenefitBuilder {
        InsurancePlan_BenefitBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> InsurancePlan_BenefitBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return InsurancePlan_BenefitBuilder { value: __value };
    }

    pub fn _requirement<'a>(&'a mut self, val: Element) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["_requirement"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn limit<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_Limit>,
    ) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["limit"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requirement<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_BenefitBuilder {
        self.value["requirement"] = json!(val);
        return self;
    }
}
