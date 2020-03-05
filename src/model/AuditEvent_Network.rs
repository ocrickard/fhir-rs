#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent_Network<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent_Network<'_> {
    pub fn new(value: &Value) -> AuditEvent_Network {
        AuditEvent_Network {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for address
    pub fn _address(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_address") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

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

    /// An identifier for the type of network access point that originated the audit
    /// event.
    pub fn fhir_type(&self) -> Option<AuditEvent_NetworkType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(AuditEvent_NetworkType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.address() {}
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
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEvent_NetworkBuilder {
    pub(crate) value: Value,
}

impl AuditEvent_NetworkBuilder {
    pub fn build(&self) -> AuditEvent_Network {
        AuditEvent_Network {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent_Network) -> AuditEvent_NetworkBuilder {
        AuditEvent_NetworkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AuditEvent_NetworkBuilder {
        let mut __value: Value = json!({});
        return AuditEvent_NetworkBuilder { value: __value };
    }

    pub fn _address<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["_address"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["address"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: AuditEvent_NetworkType,
    ) -> &'a mut AuditEvent_NetworkBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            AuditEvent_NetworkType::One => "1".to_string(),
            AuditEvent_NetworkType::Two => "2".to_string(),
            AuditEvent_NetworkType::Three => "3".to_string(),
            AuditEvent_NetworkType::Four => "4".to_string(),
            AuditEvent_NetworkType::Five => "5".to_string(),
        }
    }
}
