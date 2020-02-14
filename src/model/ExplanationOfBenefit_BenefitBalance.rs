#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::ExplanationOfBenefit_Financial::ExplanationOfBenefit_Financial;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_BenefitBalance<'a> {
    pub value: &'a Value,
}

impl ExplanationOfBenefit_BenefitBalance<'_> {
    /// Benefits Used to date.
    pub fn financial(&self) -> Option<Vec<ExplanationOfBenefit_Financial>> {
        if let Some(Value::Array(val)) = self.value.get("financial") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Financial { value: e })
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

    /// Extensions for excluded
    pub fn _excluded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_excluded") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A short name or tag for the benefit.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Is a flag to indicate whether the benefits refer to in-network providers or out-
    /// of-network providers.
    pub fn network(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("network") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// True if the indicated class of service is excluded from the plan, missing or
    /// False indicates the product or service is included in the coverage.
    pub fn excluded(&self) -> Option<bool> {
        if let Some(val) = self.value.get("excluded") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates if the benefits apply to an individual or to the family.
    pub fn unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unit") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub fn category(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["category"],
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

    /// A richer description of the benefit or services covered.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
    /// annual visits'.
    pub fn term(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("term") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.financial() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._excluded() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.network() {
            _val.validate();
        }
        if let Some(_val) = self.excluded() {}
        if let Some(_val) = self.unit() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        let _ = self.category().validate();
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.term() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
