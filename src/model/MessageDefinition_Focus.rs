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
pub struct MessageDefinition_Focus<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MessageDefinition_Focus<'_> {
    pub fn new(value: &Value) -> MessageDefinition_Focus {
        MessageDefinition_Focus {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The kind of resource that must be the focus for this message.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// Identifies the maximum number of resources of this type that must be pointed to
    /// by a message in order for it to be valid against this MessageDefinition.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the minimum number of resources of this type that must be pointed to
    /// by a message in order for it to be valid against this MessageDefinition.
    pub fn min(&self) -> Option<u64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_u64().unwrap());
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

    /// A profile that reflects constraints for the focal resource (and potentially for
    /// related resources).
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._min() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.profile() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MessageDefinition_FocusBuilder {
    pub(crate) value: Value,
}

impl MessageDefinition_FocusBuilder {
    pub fn build(&self) -> MessageDefinition_Focus {
        MessageDefinition_Focus {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MessageDefinition_Focus) -> MessageDefinition_FocusBuilder {
        MessageDefinition_FocusBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MessageDefinition_FocusBuilder {
        let mut __value: Value = json!({});
        return MessageDefinition_FocusBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: u64) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: &str) -> &'a mut MessageDefinition_FocusBuilder {
        self.value["profile"] = json!(val);
        return self;
    }
}
