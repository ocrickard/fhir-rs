#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An amount of economic utility in some recognized currency.

#[derive(Debug)]
pub struct Money<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Money<'_> {
    pub fn new(value: &Value) -> Money {
        Money {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for currency
    pub fn _currency(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_currency") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// ISO 4217 Currency Code.
    pub fn currency(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("currency") {
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

    /// Numerical value (with implicit precision).
    pub fn value(&self) -> Option<f64> {
        if let Some(val) = self.value.get("value") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._currency() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.currency() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MoneyBuilder {
    pub(crate) value: Value,
}

impl MoneyBuilder {
    pub fn build(&self) -> Money {
        Money {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Money) -> MoneyBuilder {
        MoneyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MoneyBuilder {
        let mut __value: Value = json!({});
        return MoneyBuilder { value: __value };
    }

    pub fn _currency<'a>(&'a mut self, val: Element) -> &'a mut MoneyBuilder {
        self.value["_currency"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut MoneyBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn currency<'a>(&'a mut self, val: &str) -> &'a mut MoneyBuilder {
        self.value["currency"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MoneyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MoneyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: f64) -> &'a mut MoneyBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
