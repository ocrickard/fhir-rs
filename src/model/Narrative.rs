#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A human-readable summary of the resource conveying the essential clinical and
/// business information for the resource.

#[derive(Debug)]
pub struct Narrative<'a> {
    pub value: &'a Value,
}

impl Narrative<'_> {
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

    /// The status of the narrative - whether it's entirely generated (from just the
    /// defined data or the extensions too), or whether a human authored it and it may
    /// contain additional data.
    pub fn status(&self) -> Option<NarrativeStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(NarrativeStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
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

    /// The actual narrative content, a stripped down version of XHTML.
    pub fn div(&self) -> &str {
        self.value.get("div").unwrap().as_str().unwrap()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        let _ = self.div();
        return true;
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
}
