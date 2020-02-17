#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reference to a code defined by a terminology system.

#[derive(Debug)]
pub struct Coding<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Coding<'_> {
    pub fn new(value: &Value) -> Coding {
        Coding {
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

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for userSelected
    pub fn _user_selected(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_userSelected") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A symbol in syntax defined by the system. The symbol may be a predefined code or
    /// an expression in a syntax defined by the coding system (e.g. post-coordination).
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// A representation of the meaning of the code in the system, following the rules
    /// of the system.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// The identification of the code system that defines the meaning of the symbol in
    /// the code.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// Indicates that this coding was chosen by a user directly - e.g. off a pick list
    /// of available items (codes or displays).
    pub fn user_selected(&self) -> Option<bool> {
        if let Some(val) = self.value.get("userSelected") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The version of the code system which was used when choosing this code. Note that
    /// a well-maintained code system does not need the version reported, because the
    /// meaning of codes is consistent across versions. However this cannot consistently
    /// be assured, and when the meaning is not guaranteed to be consistent, the version
    /// SHOULD be exchanged.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
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
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._user_selected() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.user_selected() {}
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CodingBuilder {
    pub(crate) value: Value,
}

impl CodingBuilder {
    pub fn build(&self) -> Coding {
        Coding {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Coding) -> CodingBuilder {
        CodingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CodingBuilder {
        let mut __value: Value = json!({});
        return CodingBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut CodingBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut CodingBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _system<'a>(&'a mut self, val: Element) -> &'a mut CodingBuilder {
        self.value["_system"] = json!(val.value);
        return self;
    }

    pub fn _user_selected<'a>(&'a mut self, val: Element) -> &'a mut CodingBuilder {
        self.value["_userSelected"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut CodingBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut CodingBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut CodingBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CodingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CodingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn system<'a>(&'a mut self, val: &str) -> &'a mut CodingBuilder {
        self.value["system"] = json!(val);
        return self;
    }

    pub fn user_selected<'a>(&'a mut self, val: bool) -> &'a mut CodingBuilder {
        self.value["userSelected"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut CodingBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
