#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A concept that may be defined by a formal reference to a terminology or ontology
/// or may be provided by text.

#[derive(Debug)]
pub struct CodeableConcept<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CodeableConcept<'_> {
    pub fn new(value: &Value) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to a code defined by a terminology system.
    pub fn coding(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("coding") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A human language representation of the concept as seen/selected/uttered by the
    /// user who entered the data and/or which represents the intended meaning of the
    /// user.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.coding() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CodeableConceptBuilder {
    pub(crate) value: Value,
}

impl CodeableConceptBuilder {
    pub fn build(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CodeableConcept) -> CodeableConceptBuilder {
        CodeableConceptBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CodeableConceptBuilder {
        let mut __value: Value = json!({});
        return CodeableConceptBuilder { value: __value };
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut CodeableConceptBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn coding<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut CodeableConceptBuilder {
        self.value["coding"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CodeableConceptBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CodeableConceptBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut CodeableConceptBuilder {
        self.value["text"] = json!(val);
        return self;
    }
}
