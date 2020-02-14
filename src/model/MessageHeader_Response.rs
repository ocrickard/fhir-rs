#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.

#[derive(Debug)]
pub struct MessageHeader_Response<'a> {
    pub value: &'a Value,
}

impl MessageHeader_Response<'_> {
    /// Extensions for identifier
    pub fn _identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identifier") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Code that identifies the type of response to the message - whether it was
    /// successful or not, and whether it should be resent or not.
    pub fn code(&self) -> Option<MessageHeader_ResponseCode> {
        if let Some(Value::String(val)) = self.value.get("code") {
            return Some(MessageHeader_ResponseCode::from_string(&val).unwrap());
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

    /// Full details of any issues found in the message.
    pub fn details(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("details") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The MessageHeader.id of the message to which this message is a response.
    pub fn identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identifier") {
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

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
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
        if let Some(_val) = self._identifier() {
            _val.validate();
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.details() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}

#[derive(Debug)]
pub enum MessageHeader_ResponseCode {
    Ok,
    TransientError,
    FatalError,
}

impl MessageHeader_ResponseCode {
    pub fn from_string(string: &str) -> Option<MessageHeader_ResponseCode> {
        match string {
            "ok" => Some(MessageHeader_ResponseCode::Ok),
            "transient-error" => Some(MessageHeader_ResponseCode::TransientError),
            "fatal-error" => Some(MessageHeader_ResponseCode::FatalError),
            _ => None,
        }
    }
}
