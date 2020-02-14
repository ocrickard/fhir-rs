#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use serde_json::value::Value;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult_Validator<'a> {
    pub value: &'a Value,
}

impl VerificationResult_Validator<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Reference to the organization validating information.
    pub fn organization(&self) -> Reference {
        Reference {
            value: &self.value["organization"],
        }
    }

    /// Extensions for identityCertificate
    pub fn _identity_certificate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identityCertificate") {
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

    /// A digital identity certificate associated with the validator.
    pub fn identity_certificate(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identityCertificate") {
            return Some(string);
        }
        return None;
    }

    /// Signed assertion by the validator that they have validated the information.
    pub fn attestation_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("attestationSignature") {
            return Some(Signature { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        let _ = self.organization().validate();
        if let Some(_val) = self._identity_certificate() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identity_certificate() {}
        if let Some(_val) = self.attestation_signature() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
