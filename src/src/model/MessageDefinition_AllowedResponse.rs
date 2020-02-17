#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.

#[derive(Debug)]
pub struct MessageDefinition_AllowedResponse<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MessageDefinition_AllowedResponse<'_> {
    pub fn new(value: &Value) -> MessageDefinition_AllowedResponse {
        MessageDefinition_AllowedResponse {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for situation
    pub fn _situation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_situation") {
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

    /// A reference to the message definition that must be adhered to by this supported
    /// response.
    pub fn message(&self) -> &str {
        self.value.get("message").unwrap().as_str().unwrap()
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

    /// Provides a description of the circumstances in which this response should be
    /// used (as opposed to one of the alternative responses).
    pub fn situation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("situation") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._situation() {
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
        if let Some(_val) = self.situation() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MessageDefinition_AllowedResponseBuilder {
    pub(crate) value: Value,
}

impl MessageDefinition_AllowedResponseBuilder {
    pub fn build(&self) -> MessageDefinition_AllowedResponse {
        MessageDefinition_AllowedResponse {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MessageDefinition_AllowedResponse,
    ) -> MessageDefinition_AllowedResponseBuilder {
        MessageDefinition_AllowedResponseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(message: &str) -> MessageDefinition_AllowedResponseBuilder {
        let mut __value: Value = json!({});
        __value["message"] = json!(message);
        return MessageDefinition_AllowedResponseBuilder { value: __value };
    }

    pub fn _situation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MessageDefinition_AllowedResponseBuilder {
        self.value["_situation"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageDefinition_AllowedResponseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MessageDefinition_AllowedResponseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageDefinition_AllowedResponseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn situation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MessageDefinition_AllowedResponseBuilder {
        self.value["situation"] = json!(val);
        return self;
    }
}
