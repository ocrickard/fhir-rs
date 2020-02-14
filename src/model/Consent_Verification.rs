#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Verification<'a> {
    pub value: &'a Value,
}

impl Consent_Verification<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for verified
    pub fn _verified(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_verified") {
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

    /// Has the instruction been verified.
    pub fn verified(&self) -> Option<bool> {
        if let Some(val) = self.value.get("verified") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for verificationDate
    pub fn _verification_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_verificationDate") {
            return Some(Element { value: val });
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

    /// Who verified the instruction (Patient, Relative or other Authorized Person).
    pub fn verified_with(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("verifiedWith") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._verified() {
            _val.validate();
        }
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
        if let Some(_val) = self.verified() {}
        if let Some(_val) = self._verification_date() {
            _val.validate();
        }
        if let Some(_val) = self.verification_date() {}
        if let Some(_val) = self.verified_with() {
            _val.validate();
        }
        return true;
    }
}
