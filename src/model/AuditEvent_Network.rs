#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent_Network<'a> {
    pub value: &'a Value,
}

impl AuditEvent_Network<'_> {
    /// An identifier for the network access point of the user device for the audit
    /// event.
    pub fn address(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("address") {
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

    /// Extensions for address
    pub fn _address(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_address") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An identifier for the type of network access point that originated the audit
    /// event.
    pub fn fhir_type(&self) -> Option<AuditEvent_NetworkType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(AuditEvent_NetworkType::from_string(&val).unwrap());
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.address() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._address() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}

#[derive(Debug)]
pub enum AuditEvent_NetworkType {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl AuditEvent_NetworkType {
    pub fn from_string(string: &str) -> Option<AuditEvent_NetworkType> {
        match string {
            "1" => Some(AuditEvent_NetworkType::One),
            "2" => Some(AuditEvent_NetworkType::Two),
            "3" => Some(AuditEvent_NetworkType::Three),
            "4" => Some(AuditEvent_NetworkType::Four),
            "5" => Some(AuditEvent_NetworkType::Five),
            _ => None,
        }
    }
}
