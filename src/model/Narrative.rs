#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A human-readable summary of the resource conveying the essential clinical and
/// business information for the resource.

#[derive(Debug)]
pub struct Narrative<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Narrative<'_> {
    pub fn new(value: &Value) -> Narrative {
        Narrative {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual narrative content, a stripped down version of XHTML.
    pub fn div(&self) -> &str {
        self.value.get("div").unwrap().as_str().unwrap()
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

    /// The status of the narrative - whether it's entirely generated (from just the
    /// defined data or the extensions too), or whether a human authored it and it may
    /// contain additional data.
    pub fn status(&self) -> Option<NarrativeStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(NarrativeStatus::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._status() {
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
        if let Some(_val) = self.status() {}
        return true;
    }
}

#[derive(Debug)]
pub struct NarrativeBuilder {
    pub(crate) value: Value,
}

impl NarrativeBuilder {
    pub fn build(&self) -> Narrative {
        Narrative {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Narrative) -> NarrativeBuilder {
        NarrativeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(div: &str) -> NarrativeBuilder {
        let mut __value: Value = json!({});
        __value["div"] = json!(div);
        return NarrativeBuilder { value: __value };
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut NarrativeBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut NarrativeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NarrativeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: NarrativeStatus) -> &'a mut NarrativeBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum NarrativeStatus {
    Generated,
    Extensions,
    Additional,
    Empty,
}

impl NarrativeStatus {
    pub fn from_string(string: &str) -> Option<NarrativeStatus> {
        match string {
            "generated" => Some(NarrativeStatus::Generated),
            "extensions" => Some(NarrativeStatus::Extensions),
            "additional" => Some(NarrativeStatus::Additional),
            "empty" => Some(NarrativeStatus::Empty),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NarrativeStatus::Generated => "generated".to_string(),
            NarrativeStatus::Extensions => "extensions".to_string(),
            NarrativeStatus::Additional => "additional".to_string(),
            NarrativeStatus::Empty => "empty".to_string(),
        }
    }
}
