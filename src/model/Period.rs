#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A time period defined by a start and end date and optionally time.

#[derive(Debug)]
pub struct Period<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Period<'_> {
    pub fn new(value: &Value) -> Period {
        Period {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The end of the period. If the end of the period is missing, it means no end was
    /// known or planned at the time the instance was created. The start may be in the
    /// past, and the end date in the future, which means that period is
    /// expected/planned to end at that time.
    pub fn end(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("end") {
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

    /// The start of the period. The boundary is inclusive.
    pub fn start(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("start") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._end() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.start() {}
        return true;
    }
}

#[derive(Debug)]
pub struct PeriodBuilder {
    pub(crate) value: Value,
}

impl PeriodBuilder {
    pub fn build(&self) -> Period {
        Period {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Period) -> PeriodBuilder {
        PeriodBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PeriodBuilder {
        let mut __value: Value = json!({});
        return PeriodBuilder { value: __value };
    }

    pub fn _end<'a>(&'a mut self, val: Element) -> &'a mut PeriodBuilder {
        self.value["_end"] = json!(val.value);
        return self;
    }

    pub fn _start<'a>(&'a mut self, val: Element) -> &'a mut PeriodBuilder {
        self.value["_start"] = json!(val.value);
        return self;
    }

    pub fn end<'a>(&'a mut self, val: &str) -> &'a mut PeriodBuilder {
        self.value["end"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PeriodBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PeriodBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn start<'a>(&'a mut self, val: &str) -> &'a mut PeriodBuilder {
        self.value["start"] = json!(val);
        return self;
    }
}
