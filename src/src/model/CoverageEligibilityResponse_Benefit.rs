#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse_Benefit<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CoverageEligibilityResponse_Benefit<'_> {
    pub fn new(value: &Value) -> CoverageEligibilityResponse_Benefit {
        CoverageEligibilityResponse_Benefit {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for allowedString
    pub fn _allowed_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for allowedUnsignedInt
    pub fn _allowed_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allowedUnsignedInt") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for usedString
    pub fn _used_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usedString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for usedUnsignedInt
    pub fn _used_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usedUnsignedInt") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The quantity of the benefit which is permitted under the coverage.
    pub fn allowed_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("allowedMoney") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The quantity of the benefit which is permitted under the coverage.
    pub fn allowed_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("allowedString") {
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

    /// Classification of benefit being provided.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// The quantity of the benefit which have been consumed to date.
    pub fn used_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("usedMoney") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
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
    pub fn used_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("usedUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._allowed_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._allowed_unsigned_int() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._used_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._used_unsigned_int() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.allowed_money() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.allowed_string() {}
        if let Some(_val) = self.allowed_unsigned_int() {}
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
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.used_money() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.used_string() {}
        if let Some(_val) = self.used_unsigned_int() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CoverageEligibilityResponse_BenefitBuilder {
    pub(crate) value: Value,
}

impl CoverageEligibilityResponse_BenefitBuilder {
    pub fn build(&self) -> CoverageEligibilityResponse_Benefit {
        CoverageEligibilityResponse_Benefit {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: CoverageEligibilityResponse_Benefit,
    ) -> CoverageEligibilityResponse_BenefitBuilder {
        CoverageEligibilityResponse_BenefitBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> CoverageEligibilityResponse_BenefitBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return CoverageEligibilityResponse_BenefitBuilder { value: __value };
    }

    pub fn _allowed_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["_allowedString"] = json!(val.value);
        return self;
    }

    pub fn _allowed_unsigned_int<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["_allowedUnsignedInt"] = json!(val.value);
        return self;
    }

    pub fn _used_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["_usedString"] = json!(val.value);
        return self;
    }

    pub fn _used_unsigned_int<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["_usedUnsignedInt"] = json!(val.value);
        return self;
    }

    pub fn allowed_money<'a>(
        &'a mut self,
        val: Money,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["allowedMoney"] = json!(val.value);
        return self;
    }

    pub fn allowed_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["allowedString"] = json!(val);
        return self;
    }

    pub fn allowed_unsigned_int<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["allowedUnsignedInt"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn used_money<'a>(
        &'a mut self,
        val: Money,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["usedMoney"] = json!(val.value);
        return self;
    }

    pub fn used_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["usedString"] = json!(val);
        return self;
    }

    pub fn used_unsigned_int<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut CoverageEligibilityResponse_BenefitBuilder {
        self.value["usedUnsignedInt"] = json!(val);
        return self;
    }
}
