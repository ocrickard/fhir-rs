#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of ordered Quantities defined by a low and high limit.

#[derive(Debug)]
pub struct Range<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Range<'_> {
    pub fn new(value: &Value) -> Range {
        Range {
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

    /// The high limit. The boundary is inclusive.
    pub fn high(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("high") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
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

    /// The low limit. The boundary is inclusive.
    pub fn low(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("low") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.high() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.low() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RangeBuilder {
    pub(crate) value: Value,
}

impl RangeBuilder {
    pub fn build(&self) -> Range {
        Range {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Range) -> RangeBuilder {
        RangeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RangeBuilder {
        let mut __value: Value = json!({});
        return RangeBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut RangeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn high<'a>(&'a mut self, val: Quantity) -> &'a mut RangeBuilder {
        self.value["high"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RangeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn low<'a>(&'a mut self, val: Quantity) -> &'a mut RangeBuilder {
        self.value["low"] = json!(val.value);
        return self;
    }
}
