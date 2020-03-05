#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A relationship of two Quantity values - expressed as a numerator and a
/// denominator.

#[derive(Debug)]
pub struct Ratio<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Ratio<'_> {
    pub fn new(value: &Value) -> Ratio {
        Ratio {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The value of the denominator.
    pub fn denominator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("denominator") {
            return Some(Quantity {
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

    /// The value of the numerator.
    pub fn numerator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("numerator") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.denominator() {
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
        if let Some(_val) = self.numerator() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RatioBuilder {
    pub(crate) value: Value,
}

impl RatioBuilder {
    pub fn build(&self) -> Ratio {
        Ratio {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Ratio) -> RatioBuilder {
        RatioBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RatioBuilder {
        let mut __value: Value = json!({});
        return RatioBuilder { value: __value };
    }

    pub fn denominator<'a>(&'a mut self, val: Quantity) -> &'a mut RatioBuilder {
        self.value["denominator"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut RatioBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RatioBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn numerator<'a>(&'a mut self, val: Quantity) -> &'a mut RatioBuilder {
        self.value["numerator"] = json!(val.value);
        return self;
    }
}
