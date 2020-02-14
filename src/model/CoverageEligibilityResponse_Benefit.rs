#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use serde_json::value::Value;

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse_Benefit<'a> {
    pub value: &'a Value,
}

impl CoverageEligibilityResponse_Benefit<'_> {
    /// The quantity of the benefit which is permitted under the coverage.
    pub fn allowed_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("allowedString") {
            return Some(string);
        }
        return None;
    }

    /// The quantity of the benefit which have been consumed to date.
    pub fn used_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("usedUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Classification of benefit being provided.
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

    /// Extensions for usedString
    pub fn _used_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usedString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for allowedUnsignedInt
    pub fn _allowed_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for allowedString
    pub fn _allowed_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedString") {
            return Some(Element { value: val });
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

    /// The quantity of the benefit which is permitted under the coverage.
    pub fn allowed_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("allowedMoney") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// Extensions for usedUnsignedInt
    pub fn _used_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usedUnsignedInt") {
            return Some(Element { value: val });
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

    /// The quantity of the benefit which is permitted under the coverage.
    pub fn allowed_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("allowedUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The quantity of the benefit which have been consumed to date.
    pub fn used_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usedString") {
            return Some(string);
        }
        return None;
    }

    /// The quantity of the benefit which have been consumed to date.
    pub fn used_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("usedMoney") {
            return Some(Money { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.allowed_string() {}
        if let Some(_val) = self.used_unsigned_int() {}
        let _ = self.fhir_type().validate();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._used_string() {
            _val.validate();
        }
        if let Some(_val) = self._allowed_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._allowed_string() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.allowed_money() {
            _val.validate();
        }
        if let Some(_val) = self._used_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.allowed_unsigned_int() {}
        if let Some(_val) = self.used_string() {}
        if let Some(_val) = self.used_money() {
            _val.validate();
        }
        return true;
    }
}
