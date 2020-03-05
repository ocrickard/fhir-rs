#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Base definition for all elements in a resource.

#[derive(Debug)]
pub struct Element<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Element<'_> {
    pub fn new(value: &Value) -> Element {
        Element {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ElementBuilder {
    pub(crate) value: Value,
}

impl ElementBuilder {
    pub fn build(&self) -> Element {
        Element {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Element) -> ElementBuilder {
        ElementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementBuilder {
        let mut __value: Value = json!({});
        return ElementBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ElementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementBuilder {
        self.value["id"] = json!(val);
        return self;
    }
}
