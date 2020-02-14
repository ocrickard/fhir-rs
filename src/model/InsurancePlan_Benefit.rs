#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::InsurancePlan_Limit::InsurancePlan_Limit;
use serde_json::value::Value;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Benefit<'a> {
    pub value: &'a Value,
}

impl InsurancePlan_Benefit<'_> {
    /// The specific limits on the benefit.
    pub fn limit(&self) -> Option<Vec<InsurancePlan_Limit>> {
        if let Some(Value::Array(val)) = self.value.get("limit") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_Limit { value: e })
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

    /// Type of benefit (primary care; speciality care; inpatient; outpatient).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
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

    /// The referral requirements to have access/coverage for this benefit.
    pub fn requirement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirement") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for requirement
    pub fn _requirement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirement") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.limit() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        let _ = self.fhir_type().validate();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.requirement() {}
        if let Some(_val) = self._requirement() {
            _val.validate();
        }
        return true;
    }
}
