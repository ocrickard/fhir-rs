#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Verification<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Consent_Verification<'_> {
    pub fn new(value: &Value) -> Consent_Verification {
        Consent_Verification {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for verificationDate
    pub fn _verification_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_verificationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for verified
    pub fn _verified(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_verified") {
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

    /// Date verification was collected.
    pub fn verification_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("verificationDate") {
            return Some(string);
        }
        return None;
    }

    /// Has the instruction been verified.
    pub fn verified(&self) -> Option<bool> {
        if let Some(val) = self.value.get("verified") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Who verified the instruction (Patient, Relative or other Authorized Person).
    pub fn verified_with(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("verifiedWith") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._verification_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._verified() {
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
        if let Some(_val) = self.verification_date() {}
        if let Some(_val) = self.verified() {}
        if let Some(_val) = self.verified_with() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Consent_VerificationBuilder {
    pub(crate) value: Value,
}

impl Consent_VerificationBuilder {
    pub fn build(&self) -> Consent_Verification {
        Consent_Verification {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Consent_Verification) -> Consent_VerificationBuilder {
        Consent_VerificationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Consent_VerificationBuilder {
        let mut __value: Value = json!({});
        return Consent_VerificationBuilder { value: __value };
    }

    pub fn _verification_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Consent_VerificationBuilder {
        self.value["_verificationDate"] = json!(val.value);
        return self;
    }

    pub fn _verified<'a>(&'a mut self, val: Element) -> &'a mut Consent_VerificationBuilder {
        self.value["_verified"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Consent_VerificationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Consent_VerificationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Consent_VerificationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn verification_date<'a>(&'a mut self, val: &str) -> &'a mut Consent_VerificationBuilder {
        self.value["verificationDate"] = json!(val);
        return self;
    }

    pub fn verified<'a>(&'a mut self, val: bool) -> &'a mut Consent_VerificationBuilder {
        self.value["verified"] = json!(val);
        return self;
    }

    pub fn verified_with<'a>(&'a mut self, val: Reference) -> &'a mut Consent_VerificationBuilder {
        self.value["verifiedWith"] = json!(val.value);
        return self;
    }
}
