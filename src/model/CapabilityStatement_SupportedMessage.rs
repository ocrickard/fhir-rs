#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_SupportedMessage<'a> {
    pub value: &'a Value,
}

impl CapabilityStatement_SupportedMessage<'_> {
    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Points to a message definition that identifies the messaging event, message
    /// structure, allowed responses, etc.
    pub fn definition(&self) -> &str {
        self.value.get("definition").unwrap().as_str().unwrap()
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The mode of this event declaration - whether application is sender or receiver.
    pub fn mode(&self) -> Option<CapabilityStatement_SupportedMessageMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(CapabilityStatement_SupportedMessageMode::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        let _ = self.definition();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.mode() {}
        return true;
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_SupportedMessageMode {
    Sender,
    Receiver,
}

impl CapabilityStatement_SupportedMessageMode {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_SupportedMessageMode> {
        match string {
            "sender" => Some(CapabilityStatement_SupportedMessageMode::Sender),
            "receiver" => Some(CapabilityStatement_SupportedMessageMode::Receiver),
            _ => None,
        }
    }
}
