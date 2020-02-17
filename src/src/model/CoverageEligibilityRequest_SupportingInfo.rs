#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.

#[derive(Debug)]
pub struct CoverageEligibilityRequest_SupportingInfo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CoverageEligibilityRequest_SupportingInfo<'_> {
    pub fn new(value: &Value) -> CoverageEligibilityRequest_SupportingInfo {
        CoverageEligibilityRequest_SupportingInfo {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for appliesToAll
    pub fn _applies_to_all(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_appliesToAll") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The supporting materials are applicable for all detail items, product/servce
    /// categories and specific billing codes.
    pub fn applies_to_all(&self) -> Option<bool> {
        if let Some(val) = self.value.get("appliesToAll") {
            return Some(val.as_bool().unwrap());
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

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn information(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["information"]),
        }
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

    /// A number to uniquely identify supporting information entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._applies_to_all() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.applies_to_all() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self.information().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CoverageEligibilityRequest_SupportingInfoBuilder {
    pub(crate) value: Value,
}

impl CoverageEligibilityRequest_SupportingInfoBuilder {
    pub fn build(&self) -> CoverageEligibilityRequest_SupportingInfo {
        CoverageEligibilityRequest_SupportingInfo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: CoverageEligibilityRequest_SupportingInfo,
    ) -> CoverageEligibilityRequest_SupportingInfoBuilder {
        CoverageEligibilityRequest_SupportingInfoBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(information: Reference) -> CoverageEligibilityRequest_SupportingInfoBuilder {
        let mut __value: Value = json!({});
        __value["information"] = json!(information.value);
        return CoverageEligibilityRequest_SupportingInfoBuilder { value: __value };
    }

    pub fn _applies_to_all<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["_appliesToAll"] = json!(val.value);
        return self;
    }

    pub fn _sequence<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn applies_to_all<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["appliesToAll"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sequence<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut CoverageEligibilityRequest_SupportingInfoBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }
}
