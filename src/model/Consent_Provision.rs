#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Consent_Actor::Consent_Actor;
use crate::model::Consent_Data::Consent_Data;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Provision<'a> {
    pub value: &'a Value,
}

impl Consent_Provision<'_> {
    /// The resources controlled by this rule if specific resources are referenced.
    pub fn data(&self) -> Option<Vec<Consent_Data>> {
        if let Some(Value::Array(val)) = self.value.get("data") {
            return Some(
                val.into_iter()
                    .map(|e| Consent_Data { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Actions controlled by this Rule.
    pub fn action(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The context of the activities a user is taking - why the user is accessing the
    /// data - that are controlled by this rule.
    pub fn purpose(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("purpose") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The class of information covered by this rule. The type can be a FHIR resource
    /// type, a profile on a type, or a CDA document, or some other type that indicates
    /// what sort of information the consent relates to.
    pub fn class(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("class") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Who or what is controlled by this rule. Use group to identify a set of actors by
    /// some property they share (e.g. 'admitting officers').
    pub fn actor(&self) -> Option<Vec<Consent_Actor>> {
        if let Some(Value::Array(val)) = self.value.get("actor") {
            return Some(
                val.into_iter()
                    .map(|e| Consent_Actor { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Rules which provide exceptions to the base rule or subrules.
    pub fn provision(&self) -> Option<Vec<Consent_Provision>> {
        if let Some(Value::Array(val)) = self.value.get("provision") {
            return Some(
                val.into_iter()
                    .map(|e| Consent_Provision { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The timeframe in this rule is valid.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A security label, comprised of 0..* security label fields (Privacy tags), which
    /// define which resources are controlled by this exception.
    pub fn security_label(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabel") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// Action  to take - permit or deny - when the rule conditions are met.  Not
    /// permitted in root rule, required in all nested rules.
    pub fn fhir_type(&self) -> Option<Consent_ProvisionType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Consent_ProvisionType::from_string(&val).unwrap());
        }
        return None;
    }

    /// If this code is found in an instance, then the rule applies.
    pub fn code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Clinical or Operational Relevant period of time that bounds the data controlled
    /// by this rule.
    pub fn data_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("dataPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.data() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.purpose() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.class() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.actor() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.provision() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.security_label() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.data_period() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Consent_ProvisionType {
    Deny,
    Permit,
}

impl Consent_ProvisionType {
    pub fn from_string(string: &str) -> Option<Consent_ProvisionType> {
        match string {
            "deny" => Some(Consent_ProvisionType::Deny),
            "permit" => Some(Consent_ProvisionType::Permit),
            _ => None,
        }
    }
}
