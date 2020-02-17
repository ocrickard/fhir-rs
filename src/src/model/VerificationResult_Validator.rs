#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult_Validator<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl VerificationResult_Validator<'_> {
    pub fn new(value: &Value) -> VerificationResult_Validator {
        VerificationResult_Validator {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for identityCertificate
    pub fn _identity_certificate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identityCertificate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Signed assertion by the validator that they have validated the information.
    pub fn attestation_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("attestationSignature") {
            return Some(Signature {
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

    /// A digital identity certificate associated with the validator.
    pub fn identity_certificate(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identityCertificate") {
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

    /// Reference to the organization validating information.
    pub fn organization(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["organization"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._identity_certificate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.attestation_signature() {
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
        if let Some(_val) = self.identity_certificate() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.organization().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct VerificationResult_ValidatorBuilder {
    pub(crate) value: Value,
}

impl VerificationResult_ValidatorBuilder {
    pub fn build(&self) -> VerificationResult_Validator {
        VerificationResult_Validator {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: VerificationResult_Validator) -> VerificationResult_ValidatorBuilder {
        VerificationResult_ValidatorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(organization: Reference) -> VerificationResult_ValidatorBuilder {
        let mut __value: Value = json!({});
        __value["organization"] = json!(organization.value);
        return VerificationResult_ValidatorBuilder { value: __value };
    }

    pub fn _identity_certificate<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["_identityCertificate"] = json!(val.value);
        return self;
    }

    pub fn attestation_signature<'a>(
        &'a mut self,
        val: Signature,
    ) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["attestationSignature"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identity_certificate<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["identityCertificate"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut VerificationResult_ValidatorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
